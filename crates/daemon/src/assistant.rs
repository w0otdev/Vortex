use super::*;
use serde_json::{Value, json};
use vortex_classify::ai::{
    AiClassificationProposal, ai_cache_key, execute_ai_proposal, validate_ai_config,
    validate_ai_proposal_json,
};
use vortex_core::config::AiConfig;
use vortex_core::model::ItemState;

const TASK: &str = "classify";

fn account_enabled(store: &SqliteStore, account: &str) -> Result<bool, String> {
    Ok(store
        .list_accounts()?
        .iter()
        .any(|a| a.id == account && a.ai_enabled))
}

pub(super) fn queue_item(
    store: &SqliteStore,
    id: i64,
    cfg: &AiConfig,
    force: bool,
) -> Result<(), String> {
    let item = store.get_item(id)?.ok_or("item not found")?;
    if !account_enabled(store, &item.account)? {
        return Err("AI is not enabled for this mailbox".into());
    }
    if !matches!(item.state, ItemState::Read | ItemState::Unread) {
        return Err("Only visible mail can be analyzed".into());
    }
    let preview = build_ai_preview(&item, TASK, cfg);
    if let Some(reason) = preview.blocked_reason {
        return Err(reason);
    }
    store.queue_ai_item(id, &ai_cache_key(&item, TASK, cfg), force)
}

pub(super) fn settings(state: &DaemonState) -> Result<Value, String> {
    let cfg = &state.config.ai;
    let key_id = format!("ai:{}", cfg.provider.trim().to_ascii_lowercase());
    Ok(json!({
        "enabled": cfg.enabled, "provider": cfg.provider, "endpoint": cfg.endpoint,
        "small_model": cfg.small_model, "auto_hide": cfg.auto_hide,
        "accounts": state.store.list_accounts()?.iter().filter(|a| a.ai_enabled)
            .map(|a| a.id.clone()).collect::<Vec<_>>(),
        "has_api_key": matches!(cfg.provider.as_str(), "openai" | "anthropic")
            && state.secrets.get_secret(&key_id)?.is_some(),
        "allowed_now": state.policy.ai_allowed_now,
        "busy": state.ai_busy, "last_error": state.ai_last_error,
    }))
}

pub(super) fn save_settings(state: &mut DaemonState, value: Value) -> Result<Value, String> {
    let mut cfg = state.config.ai.clone();
    cfg.enabled = value
        .get("enabled")
        .and_then(Value::as_bool)
        .ok_or("missing enabled")?;
    cfg.auto_hide = value
        .get("auto_hide")
        .and_then(Value::as_bool)
        .unwrap_or(cfg.auto_hide);
    if let Some(provider) = value.get("provider").and_then(Value::as_str) {
        cfg.provider = provider.trim().to_ascii_lowercase();
    }
    if let Some(endpoint) = value.get("endpoint").and_then(Value::as_str) {
        cfg.endpoint = endpoint.trim().trim_end_matches('/').to_string();
    }
    if let Some(model) = value.get("small_model").and_then(Value::as_str) {
        cfg.small_model = model.trim().to_string();
    }
    if cfg.enabled {
        validate_ai_config(&cfg)?;
    }
    let selected = value
        .get("accounts")
        .and_then(Value::as_array)
        .ok_or("missing accounts")?;
    let mut accounts = state.store.list_accounts()?;
    for id in selected {
        if !accounts.iter().any(|a| Some(a.id.as_str()) == id.as_str()) {
            return Err("Unknown AI mailbox".into());
        }
    }
    let key_id = format!("ai:{}", cfg.provider);
    let mut previous_key = None;
    let new_key = value
        .get("api_key")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty());
    if cfg.enabled
        && matches!(cfg.provider.as_str(), "openai" | "anthropic")
        && new_key.is_none()
        && state.secrets.get_secret(&key_id)?.is_none()
    {
        return Err("An API key is required for this provider".into());
    }
    if let Some(key) = new_key {
        if !matches!(cfg.provider.as_str(), "openai" | "anthropic")
            || key.len() > 4096
            || key.chars().any(char::is_control)
        {
            return Err("Invalid provider API key".into());
        }
        previous_key = state.secrets.get_secret(&key_id)?;
        state.secrets.set_secret(&key_id, key)?;
    }
    for account in &mut accounts {
        account.ai_enabled = selected.iter().any(|id| id.as_str() == Some(&account.id));
    }
    // Config and mailbox consent are committed together; credentials stay in the OS keyring.
    if let Err(error) = state.store.save_ai_settings(
        &serde_json::to_string(&cfg).map_err(|e| e.to_string())?,
        &accounts,
    ) {
        if new_key.is_some() {
            let rollback = if let Some(key) = previous_key {
                state.secrets.set_secret(&key_id, &key)
            } else {
                state.secrets.delete_secret(&key_id)
            };
            if let Err(rollback_error) = rollback {
                return Err(format!(
                    "{error}; could not restore provider key: {rollback_error}"
                ));
            }
        }
        return Err(error);
    }
    state.config.ai = cfg;
    state.policy = TierRuntimePolicy::compute(
        &state.probe,
        state.config.profile_mode(),
        state.config.ai.enabled,
    );
    state.ai_generation = state.ai_generation.wrapping_add(1);
    state.ai_last_error = None;
    settings(state)
}

pub(super) fn result(state: &DaemonState, id: i64) -> Result<Value, String> {
    let item = state.store.get_item(id)?.ok_or("item not found")?;
    let (status, error) = state
        .store
        .ai_job_status(id)?
        .unwrap_or(("none".into(), None));
    Ok(json!({"status":status, "summary":item.summary, "last_error":error}))
}

pub(super) fn start_worker(state: &Arc<Mutex<DaemonState>>) -> Result<(), String> {
    let mut guard = state.lock().map_err(|_| "state lock poisoned")?;
    if !guard.config.ai.enabled || guard.ai_worker_running || !guard.policy.ai_allowed_now {
        return Ok(());
    }
    guard.ai_worker_running = true;
    let weak = Arc::downgrade(state);
    if let Err(error) = std::thread::Builder::new()
        .name("vortex-assistant".into())
        .spawn(move || {
            loop {
                let Some(state) = weak.upgrade() else { break };
                let work = {
                    let Ok(mut guard) = state.lock() else { break };
                    if !guard.config.ai.enabled || !guard.policy.ai_allowed_now {
                        guard.ai_worker_running = false;
                        guard.ai_busy = false;
                        break;
                    }
                    match next_work(&mut guard) {
                        Ok(work) => work,
                        Err(error) => {
                            guard.ai_last_error = Some(error);
                            None
                        }
                    }
                };
                if let Some(work) = work {
                    if run_work(&state, work) {
                        drop(state);
                        std::thread::sleep(Duration::from_secs(5));
                    }
                } else {
                    // Do not keep the daemon alive when its listener has been released.
                    drop(state);
                    std::thread::sleep(Duration::from_millis(1000));
                }
            }
        })
    {
        guard.ai_worker_running = false;
        return Err(format!("Could not start AI worker: {error}"));
    }
    Ok(())
}

struct Work {
    item: Item,
    key: String,
    cfg: AiConfig,
    secrets: SecretStore,
    generation: u64,
    cached: Option<AiClassificationProposal>,
}

fn next_work(state: &mut DaemonState) -> Result<Option<Work>, String> {
    if state.store.is_disk_backpressured() {
        return Ok(None);
    }
    let Some((id, key)) = state.store.next_ai_job()? else {
        return Ok(None);
    };
    let Some(item) = state.store.get_item(id)? else {
        state
            .store
            .finish_ai_job(id, &key, Some("Message removed"))?;
        return Ok(None);
    };
    let cfg = state.config.ai.clone();
    if !account_enabled(&state.store, &item.account)?
        || !matches!(item.state, ItemState::Read | ItemState::Unread)
        || build_ai_preview(&item, TASK, &cfg).blocked_reason.is_some()
        || ai_cache_key(&item, TASK, &cfg) != key
    {
        state.store.finish_ai_job(
            id,
            &key,
            Some("Analysis canceled: message or settings changed"),
        )?;
        return Ok(None);
    }
    let cached = state
        .store
        .get_ai_cache(&key, TASK)?
        .and_then(|json| validate_ai_proposal_json(&json).ok());
    let secrets = SecretStore::new_memory();
    let provider = cfg.provider.trim().to_ascii_lowercase();
    if matches!(provider.as_str(), "openai" | "anthropic") {
        let id = format!("ai:{provider}");
        if let Some(key) = state.secrets.get_secret(&id)? {
            secrets.set_secret(&id, &key)?;
        }
    }
    state.ai_busy = true;
    Ok(Some(Work {
        item,
        key,
        cfg,
        secrets,
        generation: state.ai_generation,
        cached,
    }))
}

fn can_hide(item: &Item, proposal: &AiClassificationProposal) -> bool {
    item.category == ItemCategory::PromoNewsletter
        && proposal.category == ItemCategory::PromoNewsletter
        && proposal.confidence >= 90
        && proposal.priority_score <= 30
        && proposal.entities.is_empty()
        && !item.pinned
        && item.snoozed_until.is_none()
        && item.extracted_codes.is_empty()
        && item.phishing_score < 60
        && item.extracted_entities.is_empty()
}

fn current_work(state: &DaemonState, work: &Work) -> Result<Item, String> {
    let item = state
        .store
        .get_item(work.item.id)?
        .ok_or("Message removed")?;
    if state.ai_generation != work.generation
        || !state.config.ai.enabled
        || !state.policy.ai_allowed_now
        || !state.store.ai_job_is_current(item.id, &work.key)?
        || !account_enabled(&state.store, &item.account)?
        || !matches!(item.state, ItemState::Read | ItemState::Unread)
        || item.state != work.item.state
        || build_ai_preview(&item, TASK, &state.config.ai)
            .blocked_reason
            .is_some()
        || ai_cache_key(&item, TASK, &state.config.ai) != work.key
    {
        return Err("Analysis canceled: message or settings changed".into());
    }
    Ok(item)
}

fn run_work(state: &Arc<Mutex<DaemonState>>, mut work: Work) -> bool {
    {
        let Ok(mut guard) = state.lock() else {
            return false;
        };
        if let Err(error) = current_work(&guard, &work) {
            guard.ai_busy = false;
            let _ = guard
                .store
                .finish_ai_job(work.item.id, &work.key, Some(&error));
            return false;
        }
    }
    let response = if let Some(proposal) = work.cached.take() {
        Ok((proposal, None))
    } else {
        execute_ai_proposal(&work.item, TASK, &work.cfg, &work.secrets)
            .map(|(proposal, preview)| (proposal, Some(preview)))
    };
    let request_failed = response.is_err();
    let Ok(mut guard) = state.lock() else {
        return request_failed;
    };
    guard.ai_busy = false;
    let outcome = (|| -> Result<(), String> {
        if !guard.store.ai_job_is_current(work.item.id, &work.key)? {
            return Ok(());
        }
        let current = current_work(&guard, &work)?;
        let (proposal, preview) = response?;
        if let Some(preview) = preview {
            guard.store.log_ai_call(
                now_epoch(),
                &preview.provider,
                &preview.model,
                TASK,
                preview.estimated_tokens,
                &preview.redacted_user_data,
            )?;
            guard.store.set_ai_cache(
                &work.key,
                TASK,
                &serde_json::to_string(&proposal).map_err(|e| e.to_string())?,
                now_epoch(),
            )?;
        }
        // Keep rule-detected actionable mail in its protected category and priority.
        let protected = matches!(
            current.category,
            ItemCategory::CodeOtp
                | ItemCategory::SecurityAlert
                | ItemCategory::Billing
                | ItemCategory::Shipping
                | ItemCategory::Work
        ) || !current.extracted_codes.is_empty()
            || current.phishing_score >= 60;
        let category = if protected {
            current.category
        } else {
            proposal.category
        };
        let priority = if protected {
            current.priority_score.max(proposal.priority_score)
        } else {
            proposal.priority_score
        };
        guard.store.apply_ai_analysis(
            current.id,
            &proposal.summary,
            category,
            priority,
            guard.config.ai.auto_hide && can_hide(&current, &proposal),
        )?;
        guard.store.finish_ai_job(current.id, &work.key, None)?;
        guard.ai_revision = guard.ai_revision.wrapping_add(1);
        Ok(())
    })();
    if let Err(error) = outcome {
        let _ = guard
            .store
            .finish_ai_job(work.item.id, &work.key, Some(&error));
        guard.ai_last_error = Some(error);
    }
    request_failed
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use vortex_core::tier::ProfileMode;

    fn test_state() -> DaemonState {
        let config = AppConfig::default();
        let probe = HardwareProbe {
            ram_mb: 8192,
            cpu_cores: 4,
            arch: "x86_64".into(),
            on_battery: false,
            headless: true,
        };
        let policy = TierRuntimePolicy::compute(&probe, ProfileMode::Standard, false);
        let store = SqliteStore::open_in_memory(true, None).unwrap();
        store
            .save_account(&AccountConfig {
                id: "mailbox".into(),
                source: SourceKind::Email,
                email_or_handle: "me@example.com".into(),
                host: "localhost".into(),
                port: 993,
                use_tls: true,
                auth_method: "password".into(),
                oauth_client_id: None,
                gmail_writeback_labels: false,
                ai_enabled: false,
                endpoint_url: None,
            })
            .unwrap();
        DaemonState {
            store,
            config,
            probe,
            policy,
            secrets: SecretStore::new_memory(),
            clipboard: ClipboardGuard::new(45),
            ai_worker_running: false,
            ai_busy: false,
            ai_generation: 0,
            ai_last_error: None,
            ai_revision: 0,
        }
    }

    fn enable(state: &mut DaemonState, endpoint: &str) {
        state.config.ai.enabled = true;
        state.config.ai.auto_hide = true;
        state.config.ai.endpoint = endpoint.into();
        state.policy = TierRuntimePolicy::compute(&state.probe, ProfileMode::Standard, true);
        let mut account = state.store.list_accounts().unwrap().remove(0);
        account.ai_enabled = true;
        state.store.save_account(&account).unwrap();
    }

    fn mail() -> Item {
        Item::new(
            SourceKind::Email,
            "mailbox",
            "newsletter",
            "Shop <sales@example.com>",
            "Weekly newsletter sale",
            "Special offer. Unsubscribe from this newsletter.",
            now_epoch(),
        )
    }

    fn ingest(state: &DaemonState, item: Item) -> i64 {
        process_new_item(
            &state.store,
            item,
            now_epoch(),
            &[],
            &state.config,
            &state.policy,
            &state.secrets,
        )
        .unwrap()
        .item_id
    }

    fn provider() -> (
        String,
        mpsc::Receiver<()>,
        mpsc::Sender<()>,
        std::thread::JoinHandle<()>,
    ) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let (arrived_tx, arrived_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut length = 0;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                if let Some((name, value)) = line.split_once(':')
                    && name.eq_ignore_ascii_case("content-length")
                {
                    length = value.trim().parse().unwrap();
                }
            }
            let mut payload = vec![0; length];
            reader.read_exact(&mut payload).unwrap();
            arrived_tx.send(()).unwrap();
            release_rx.recv_timeout(Duration::from_secs(3)).unwrap();
            let body=json!({"response":r#"{"category":"promo_newsletter","summary":"A weekly sale newsletter.","priority_score":5,"confidence":97}"#}).to_string();
            write_http_response(&mut stream, 200, "application/json", &body).unwrap();
        });
        (endpoint, arrived_rx, release_tx, handle)
    }

    #[test]
    fn disabled_or_unselected_mailbox_does_not_enqueue_ai() {
        let mut state = test_state();
        ingest(&state, mail());
        assert!(state.store.next_ai_job().unwrap().is_none());
        state.config.ai.enabled = true;
        state.policy = TierRuntimePolicy::compute(&state.probe, ProfileMode::Standard, true);
        ingest(&state, mail());
        assert!(state.store.next_ai_job().unwrap().is_none());
        enable(&mut state, "http://127.0.0.1:9");
        ingest(&state, mail());
        assert!(state.store.next_ai_job().unwrap().is_some());
        assert!(!state.ai_worker_running);
    }

    #[test]
    fn real_provider_analysis_cache_restore_and_resync() {
        let (endpoint, arrived, release, server) = provider();
        let mut state = test_state();
        enable(&mut state, &endpoint);
        let id = ingest(&state, mail());
        let work = next_work(&mut state).unwrap().unwrap();
        let state = Arc::new(Mutex::new(state));
        let background = Arc::clone(&state);
        let worker = std::thread::spawn(move || run_work(&background, work));
        arrived.recv_timeout(Duration::from_secs(3)).unwrap();
        release.send(()).unwrap();
        assert!(!worker.join().unwrap());
        server.join().unwrap();
        let mut guard = state.lock().unwrap();
        assert_eq!(guard.store.list_ai_hidden(None, 10).unwrap().len(), 1);
        assert_eq!(
            guard
                .store
                .get_item(id)
                .unwrap()
                .unwrap()
                .summary
                .as_deref(),
            Some("A weekly sale newsletter.")
        );
        assert!(guard.store.restore_ai_item(id).unwrap());
        queue_item(&guard.store, id, &guard.config.ai, true).unwrap();
        let work = next_work(&mut guard).unwrap().unwrap();
        assert!(work.cached.is_some());
        drop(guard);
        assert!(!run_work(&state, work));
        let guard = state.lock().unwrap();
        assert!(guard.store.list_ai_hidden(None, 10).unwrap().is_empty());
        assert_eq!(ingest(&guard, mail()), id);
        let item = guard.store.get_item(id).unwrap().unwrap();
        assert_eq!(item.priority_score, 5);
        assert_eq!(item.summary.as_deref(), Some("A weekly sale newsletter."));
        assert_eq!(guard.store.list_ai_calls(10).unwrap().len(), 1);
    }

    #[test]
    fn pending_network_keeps_mail_and_otp_available_and_ignores_disabled_result() {
        let (endpoint, arrived, release, server) = provider();
        let mut state = test_state();
        enable(&mut state, &endpoint);
        let id = ingest(&state, mail());
        let work = next_work(&mut state).unwrap().unwrap();
        let state = Arc::new(Mutex::new(state));
        let background = Arc::clone(&state);
        let worker = std::thread::spawn(move || run_work(&background, work));
        arrived.recv_timeout(Duration::from_secs(3)).unwrap();
        {
            let mut guard = state
                .try_lock()
                .expect("Provider I/O must release mail store lock");
            let otp = Item::new(
                SourceKind::Email,
                "mailbox",
                "otp",
                "Auth <noreply@example.com>",
                "Your verification code",
                "Your verification code is 654321. Expires in 5 minutes.",
                now_epoch(),
            );
            ingest(&guard, otp);
            assert_eq!(guard.store.list_recent_codes(now_epoch()).unwrap().len(), 1);
            guard.config.ai.enabled = false;
            guard.ai_generation += 1;
        }
        release.send(()).unwrap();
        worker.join().unwrap();
        server.join().unwrap();
        let guard = state.lock().unwrap();
        assert_eq!(guard.store.ai_job_status(id).unwrap().unwrap().0, "failed");
        assert!(guard.store.list_ai_hidden(None, 10).unwrap().is_empty());
        assert_eq!(guard.ai_revision, 0);
    }

    #[test]
    fn credentials_are_snapshotted_and_revoked_work_never_contacts_provider() {
        let mut state = test_state();
        enable(&mut state, "https://example.com");
        state.config.ai.provider = "openai".into();
        state.secrets.set_secret("ai:openai", "old-key").unwrap();
        let id = ingest(&state, mail());
        let work = next_work(&mut state).unwrap().unwrap();
        state
            .secrets
            .set_secret("ai:openai", "replacement-key")
            .unwrap();
        assert_eq!(
            work.secrets.get_secret("ai:openai").unwrap().as_deref(),
            Some("old-key")
        );
        let mut account = state.store.list_accounts().unwrap().remove(0);
        account.ai_enabled = false;
        state.store.save_account(&account).unwrap();
        let state = Arc::new(Mutex::new(state));
        assert!(!run_work(&state, work));
        let guard = state.lock().unwrap();
        assert_eq!(guard.store.ai_job_status(id).unwrap().unwrap().0, "failed");
        assert!(guard.store.list_ai_calls(1).unwrap().is_empty());
    }

    #[test]
    fn settings_validate_before_saving_and_can_always_disable_an_invalid_provider() {
        let mut state = test_state();
        let invalid = json!({"enabled":true,"provider":"openai","endpoint":"https://example.com",
            "small_model":"test-model","accounts":["mailbox"]});
        assert!(save_settings(&mut state, invalid).is_err());
        assert!(!state.config.ai.enabled);
        assert!(!state.store.list_accounts().unwrap()[0].ai_enabled);
        assert!(state.store.get_setting("ai").unwrap().is_none());
        let saved = save_settings(
            &mut state,
            json!({"enabled":false,"provider":"openai",
            "endpoint":"invalid","accounts":[],"api_key":"fake-test-key"}),
        )
        .unwrap();
        assert_eq!(saved["enabled"], false);
        assert!(saved.get("api_key").is_none());
        assert!(!saved.to_string().contains("fake-test-key"));
        assert_eq!(
            state.secrets.get_secret("ai:openai").unwrap().as_deref(),
            Some("fake-test-key")
        );
    }

    #[test]
    fn only_confident_non_actionable_promotions_can_be_hidden() {
        let mut item = Item::new(
            SourceKind::Email,
            "a",
            "1",
            "sales@example.com",
            "Sale",
            "Newsletter",
            1,
        );
        item.category = ItemCategory::PromoNewsletter;
        let mut proposal = validate_ai_proposal_json(
            r#"{"category":"promo_newsletter","summary":"Sale","priority_score":10,"confidence":95}"#,
        ).unwrap();
        assert!(can_hide(&item, &proposal));
        proposal.confidence = 89;
        assert!(!can_hide(&item, &proposal));
        proposal.confidence = 95;
        proposal.priority_score = 90;
        assert!(!can_hide(&item, &proposal));
        proposal.priority_score = 10;
        proposal.entities.push(vortex_core::model::ExtractedEntity {
            kind: "due_date".into(),
            value: "tomorrow".into(),
            label: "Due".into(),
        });
        assert!(!can_hide(&item, &proposal));
        proposal.entities.clear();
        item.pinned = true;
        assert!(!can_hide(&item, &proposal));
        item.pinned = false;
        for category in [
            ItemCategory::CodeOtp,
            ItemCategory::Billing,
            ItemCategory::Shipping,
            ItemCategory::SecurityAlert,
            ItemCategory::Work,
            ItemCategory::Other,
        ] {
            item.category = category;
            assert!(!can_hide(&item, &proposal));
        }
    }
}
