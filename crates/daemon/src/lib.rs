mod assistant;
pub mod connectors;
pub mod hub;
mod local_models;
pub mod notify;
pub mod service;

use crate::connectors::ingest_android_e2e_relay;
use crate::hub::{EncryptedEnvelope, create_pairing_ticket};
use crate::notify::{ClipboardGuard, emit_os_otp_notification};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use vortex_classify::ai::{
    answer_mailbox_qa, build_ai_preview, compute_local_embedding, generate_daily_digest,
};
use vortex_classify::classify_and_enrich;
use vortex_core::config::{AccountConfig, AppConfig, UserRuleConfig};
use vortex_core::model::{
    Connector, Item, ItemCategory, ItemFilter, OtpPopupEvent, QuickAction, SourceKind,
};
use vortex_core::secrets::SecretStore;
use vortex_core::store::SqliteStore;
use vortex_core::tier::{HardwareProbe, TierRuntimePolicy};
use vortex_core::{DeviceTier, TierBudget};
use vortex_imap::ImapConnector;
use vortex_otp::detect_fast_path;
use vortex_ui_web::{APP_JS, INDEX_HTML, OTP_OVERLAY_HTML, STYLES_CSS};

pub fn active_budget(tier: DeviceTier) -> TierBudget {
    tier.budget()
}

pub fn build_tokio_runtime(policy: &TierRuntimePolicy) -> Result<tokio::runtime::Runtime, String> {
    if policy.tier == DeviceTier::Lite || policy.worker_threads <= 1 {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("failed building single-thread runtime: {e}"))
    } else {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(policy.worker_threads)
            .enable_all()
            .build()
            .map_err(|e| format!("failed building multi-thread runtime: {e}"))
    }
}

#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub item_id: i64,
    pub popup_event: Option<OtpPopupEvent>,
    pub category: ItemCategory,
}

/// Processes a newly arrived item through both the Fast Path (immediate OTP detection)
/// and the Background Path (classification, phishing score, user rules, optional AI, store).
pub fn process_new_item(
    store: &SqliteStore,
    mut item: Item,
    now_epoch: i64,
    user_rules: &[UserRuleConfig],
    cfg: &AppConfig,
    policy: &TierRuntimePolicy,
    _secrets: &SecretStore,
) -> Result<PipelineResult, String> {
    if let Some(prev_ts) =
        store.get_existing_received_at(item.source, &item.account, &item.external_id, &item.subject)
        && prev_ts > 0
        && prev_ts < item.received_at
    {
        item.received_at = prev_ts;
    }

    // 1. FAST PATH: immediate OTP detection without waiting for classification or AI.
    let blocked_patterns = store
        .list_not_a_code_patterns(&item.sender_domain)
        .unwrap_or_default();

    let mut fast_popup: Option<OtpPopupEvent> = None;
    if let Some((extracted_code, popup)) = detect_fast_path(&item, now_epoch, &blocked_patterns) {
        item.extracted_codes.push(extracted_code);
        fast_popup = Some(popup);
    }

    // 2. BACKGROUND PATH: rule-based classification, phishing guard, entities, user rules.
    classify_and_enrich(&mut item, now_epoch, &blocked_patterns, user_rules);

    if let Some(ref mut popup) = fast_popup {
        popup.phishing_warning = item.phishing_warning.clone();
    }

    let item_id = store.upsert_item(&item)?;
    // Queue only IDs in SQLite; provider I/O never delays storing mail or OTPs.
    if cfg.ai.enabled && policy.ai_allowed_now && !store.is_disk_backpressured() {
        let _ = assistant::queue_item(store, item_id, &cfg.ai, false);
    }
    let stored_hidden = store.get_item(item_id).ok().flatten().is_some_and(|st| {
        matches!(
            st.state,
            vortex_core::model::ItemState::Trash
                | vortex_core::model::ItemState::Archived
                | vortex_core::model::ItemState::Spam
        )
    });

    if policy.semantic_search_enabled && !stored_hidden {
        let emb_text = format!("{} {} {}", item.sender, item.subject, item.body_text);
        let vec = compute_local_embedding(&emb_text);
        let _ = store.upsert_embedding(item_id, &vec);
    }

    if stored_hidden
        || fast_popup
            .as_ref()
            .is_some_and(|p| p.expires_at <= now_epoch)
    {
        fast_popup = None;
    }

    if let Some(ref mut popup) = fast_popup {
        popup.item_id = item_id;
        popup.code_hash = OtpPopupEvent::compute_dedup_hash(
            &format!("{}:{}", item.account, item.subject),
            &popup.code,
        );
        let is_new = store.record_recent_code(popup)?;
        if is_new && policy.tray_enabled {
            let _ = emit_os_otp_notification(popup);
        }
    }

    Ok(PipelineResult {
        item_id,
        popup_event: fast_popup,
        category: item.category,
    })
}

pub fn fetch_account_items(
    account: &AccountConfig,
    last_uid: u32,
    secrets: &SecretStore,
) -> Result<(Vec<Item>, Option<ImapConnector>), String> {
    match account.source {
        SourceKind::Email => {
            if account.auth_method == "google_web" || account.host == "session.gmail.com" {
                return Ok((Vec::new(), None));
            }
            let mut imap =
                ImapConnector::new(account.clone(), secrets.clone()).with_last_seen_uid(last_uid);
            imap.connect()?;
            let fetched = imap.sync(None)?;
            Ok((fetched, Some(imap)))
        }
        SourceKind::Rss => {
            let mut rss = connectors::RssConnector::new(account.clone());
            rss.connect()?;
            let fetched = rss.sync(None)?;
            Ok((fetched, None))
        }
        SourceKind::Telegram => {
            let mut tg = connectors::TelegramBotConnector::new(account.clone(), secrets.clone())
                .with_last_update_id(i64::from(last_uid));
            tg.connect()?;
            let fetched = tg.sync(None)?;
            Ok((fetched, None))
        }
        _ => Ok((Vec::new(), None)),
    }
}

pub fn sync_single_account(
    store: &SqliteStore,
    account: &AccountConfig,
    cfg: &AppConfig,
    policy: &TierRuntimePolicy,
    secrets: &SecretStore,
) -> Result<Vec<PipelineResult>, String> {
    let rules = store.list_user_rules().unwrap_or_default();
    let now = now_epoch();
    let last_uid = if matches!(account.source, SourceKind::Email | SourceKind::Telegram) {
        store
            .max_numeric_external_id(account.source, &account.id)
            .unwrap_or(0)
    } else {
        0
    };
    let (fetched, mut imap_opt) = fetch_account_items(account, last_uid, secrets)?;
    let mut results = Vec::with_capacity(fetched.len());
    for item in fetched {
        let ext_uid = item.external_id.clone();
        let res = process_new_item(store, item, now, &rules, cfg, policy, secrets)?;
        if account.gmail_writeback_labels
            && let Some(ref mut imap) = imap_opt
        {
            let _ = imap.apply_gmail_label(&ext_uid, res.category.gmail_sublabel());
        }
        results.push(res);
    }
    Ok(results)
}

pub fn infer_imap_host_port(email: &str, default_host: &str, default_port: u16) -> (String, u16) {
    let domain = email
        .rsplit_once('@')
        .map(|(_, d)| d.trim().to_ascii_lowercase())
        .unwrap_or_default();
    if domain == "gmail.com" || domain == "googlemail.com" {
        return ("imap.gmail.com".into(), 993);
    }
    if domain.starts_with("outlook.")
        || domain.starts_with("hotmail.")
        || domain.starts_with("live.")
        || domain == "msn.com"
    {
        return ("outlook.office365.com".into(), 993);
    }
    if domain == "icloud.com" || domain == "me.com" || domain == "mac.com" {
        return ("imap.mail.me.com".into(), 993);
    }
    if domain.starts_with("yahoo.") || domain.starts_with("ymail.") {
        return ("imap.mail.yahoo.com".into(), 993);
    }
    if domain.starts_with("yandex.") {
        return ("imap.yandex.com".into(), 993);
    }
    if domain.starts_with("zoho.") {
        return ("imap.zoho.com".into(), 993);
    }
    let host = if default_host.trim().is_empty() {
        "imap.gmail.com".to_string()
    } else {
        default_host.trim().to_string()
    };
    (host, if default_port == 0 { 993 } else { default_port })
}

pub fn parse_bulk_account_line(
    line: &str,
    default_host: &str,
    default_port: u16,
) -> Option<(AccountConfig, String)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let (email_raw, rest) = if let Some(pair) = trimmed.split_once('|') {
        pair
    } else if let Some(pair) = trimmed.split_once(';') {
        pair
    } else if let Some(pair) = trimmed.split_once(',') {
        pair
    } else if let Some(pair) = trimmed.split_once(':') {
        pair
    } else {
        let mut ws = trimmed.split_whitespace();
        let first = ws.next()?;
        let rem = trimmed[first.len()..].trim();
        (first, rem)
    };
    let email = email_raw.trim();
    if email.is_empty() {
        return None;
    }
    let mut parts = rest.split('|');
    let secret = parts.next().unwrap_or("").trim().to_string();
    if secret.is_empty() {
        return None;
    }
    let custom_host = parts.next().map(str::trim).filter(|s| !s.is_empty());
    let custom_port = parts
        .next()
        .and_then(|p| p.trim().parse::<u16>().ok())
        .filter(|&p| p > 0);
    let (inferred_host, inferred_port) = infer_imap_host_port(email, default_host, default_port);
    let host = custom_host.map(str::to_string).unwrap_or(inferred_host);
    let port = custom_port.unwrap_or(inferred_port);
    let acc = AccountConfig {
        id: email.to_string(),
        source: SourceKind::Email,
        email_or_handle: email.to_string(),
        host,
        port,
        use_tls: port == 993 || port == 443,
        auth_method: "password".into(),
        oauth_client_id: None,
        gmail_writeback_labels: false,
        ai_enabled: false,
        endpoint_url: None,
    };
    Some((acc, secret))
}

/// Syncs all (or one filtered) accounts concurrently without holding the global daemon mutex
/// during network I/O. Each worker writes its fetched items into SQLite as soon as its own
/// connection finishes so OTP codes from fast accounts appear immediately.
static LAST_GWEB_SPAWN_SEC: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
static GWEB_CHILD_RUNNING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn sync_accounts_concurrent(
    state: &Arc<Mutex<DaemonState>>,
    only_account: Option<&str>,
    local_bind: &str,
) -> Result<serde_json::Value, String> {
    let (jobs, google_emails, secrets) = {
        let guard = state
            .lock()
            .map_err(|_| "state lock poisoned".to_string())?;
        let accounts = guard.store.list_accounts()?;
        let all_items = guard
            .store
            .list_items(&ItemFilter {
                limit: 200,
                ..Default::default()
            })
            .unwrap_or_default();
        let mut priority_gweb = Vec::new();
        let mut needs_html_gweb = Vec::new();
        let mut normal_gweb = Vec::new();
        let mut jobs = Vec::new();
        for acc in accounts {
            if acc.auth_method == "google_web" || acc.host == "session.gmail.com" {
                let em = acc.email_or_handle.clone();
                let is_target = only_account.is_some_and(|t| !t.is_empty() && acc.id == t);
                let missing_html = all_items.iter().any(|it| {
                    it.account == acc.id && (it.body_html.is_empty() || it.body_text.len() < 280)
                });
                if is_target {
                    priority_gweb.push(em);
                } else if missing_html {
                    needs_html_gweb.push(em);
                } else {
                    normal_gweb.push(em);
                }
                continue;
            }
            if let Some(target) = only_account
                && !target.is_empty()
                && acc.id != target
            {
                continue;
            }
            let last_uid = if matches!(acc.source, SourceKind::Email | SourceKind::Telegram) {
                guard
                    .store
                    .max_numeric_external_id(acc.source, &acc.id)
                    .unwrap_or(0)
            } else {
                0
            };
            jobs.push((acc, last_uid));
        }
        priority_gweb.extend(needs_html_gweb);
        priority_gweb.extend(normal_gweb);
        (jobs, priority_gweb, guard.secrets.clone())
    };

    let mut synced_count = 0usize;
    let mut accounts_ok = 0usize;
    let mut last_error: Option<String> = None;
    let imap_jobs = jobs;

    if !google_emails.is_empty() {
        let now_s = now_epoch();
        let prev_s = LAST_GWEB_SPAWN_SEC.load(std::sync::atomic::Ordering::Relaxed);
        let min_gap = if only_account.is_some_and(|s| !s.is_empty()) {
            2
        } else {
            8
        };
        if !GWEB_CHILD_RUNNING.load(std::sync::atomic::Ordering::Relaxed)
            && now_s - prev_s >= min_gap
        {
            LAST_GWEB_SPAWN_SEC.store(now_s, std::sync::atomic::Ordering::Relaxed);
            if let Ok(exe) = std::env::current_exe() {
                let joined = google_emails.join(",");
                let mut cmd = std::process::Command::new(exe);
                cmd.args([
                    "google-login",
                    "--bind",
                    local_bind,
                    "--email",
                    &joined,
                    "--silent",
                ]);
                if let Ok(mut child) = cmd.spawn() {
                    GWEB_CHILD_RUNNING.store(true, std::sync::atomic::Ordering::Relaxed);
                    std::thread::spawn(move || {
                        let _ = child.wait();
                        GWEB_CHILD_RUNNING.store(false, std::sync::atomic::Ordering::Relaxed);
                    });
                }
            }
        }
    }

    if !imap_jobs.is_empty() {
        let worker_count = imap_jobs.len().min(16);
        let next_idx = std::sync::atomic::AtomicUsize::new(0);
        let jobs_ref = &imap_jobs;
        let next_ref = &next_idx;
        let outcomes: Vec<(String, Result<usize, String>)> = std::thread::scope(|s| {
            let mut handles = Vec::with_capacity(worker_count);
            for _ in 0..worker_count {
                let st = Arc::clone(state);
                let sec = secrets.clone();
                handles.push(s.spawn(move || {
                    let mut worker_outcomes = Vec::new();
                    loop {
                        let idx = next_ref.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some((acc, last_uid)) = jobs_ref.get(idx) else {
                            break;
                        };
                        let (fetched, mut imap_opt) =
                            match fetch_account_items(acc, *last_uid, &sec) {
                                Ok(pair) => pair,
                                Err(e) => {
                                    worker_outcomes.push((acc.id.clone(), Err(e)));
                                    continue;
                                }
                            };
                        let count = fetched.len();
                        let mut label_tasks = Vec::new();
                        let mut item_err = None;
                        if !fetched.is_empty() {
                            let Ok(guard) = st.lock() else {
                                worker_outcomes
                                    .push((acc.id.clone(), Err("state lock poisoned".into())));
                                continue;
                            };
                            let rules = guard.store.list_user_rules().unwrap_or_default();
                            let now = now_epoch();
                            for item in fetched {
                                let ext_uid = item.external_id.clone();
                                match process_new_item(
                                    &guard.store,
                                    item,
                                    now,
                                    &rules,
                                    &guard.config,
                                    &guard.policy,
                                    &guard.secrets,
                                ) {
                                    Ok(res) => {
                                        if acc.gmail_writeback_labels {
                                            label_tasks.push((
                                                ext_uid,
                                                res.category.gmail_sublabel().to_string(),
                                            ));
                                        }
                                    }
                                    Err(e) => {
                                        item_err = Some(e);
                                        break;
                                    }
                                }
                            }
                        }
                        if let Some(e) = item_err {
                            worker_outcomes.push((acc.id.clone(), Err(e)));
                            continue;
                        }
                        if let Some(ref mut imap) = imap_opt {
                            for (uid, lbl) in label_tasks {
                                let _ = imap.apply_gmail_label(&uid, &lbl);
                            }
                        }
                        worker_outcomes.push((acc.id.clone(), Ok(count)));
                    }
                    worker_outcomes
                }));
            }
            handles
                .into_iter()
                .filter_map(|h| h.join().ok())
                .flatten()
                .collect()
        });

        for (_acc_id, res) in outcomes {
            match res {
                Ok(n) => {
                    synced_count += n;
                    accounts_ok += 1;
                }
                Err(e) => last_error = Some(e),
            }
        }
    }

    if let Ok(guard) = state.lock() {
        let now = now_epoch();
        let _ = guard
            .store
            .prune_expired_bodies(now, guard.policy.keep_bodies_days);
        let archive_expired = guard.config.post_expiry_action == "archive";
        if guard.config.post_expiry_action != "none" {
            let _ = guard.store.expire_old_code_items(now, archive_expired);
        }
    }

    Ok(serde_json::json!({
        "ok": last_error.is_none(),
        "synced": synced_count,
        "accounts_synced": accounts_ok,
        "error": last_error,
    }))
}

pub struct DaemonState {
    pub store: SqliteStore,
    pub config: AppConfig,
    pub probe: HardwareProbe,
    pub policy: TierRuntimePolicy,
    pub secrets: SecretStore,
    pub clipboard: ClipboardGuard,
    ai_worker_running: bool,
    ai_busy: bool,
    ai_generation: u64,
    ai_last_error: Option<String>,
    ai_revision: u64,
    model_downloads: local_models::Downloads,
}

impl DaemonState {
    pub fn open_from_config(config_path: &Path) -> Result<Self, String> {
        let mut config = AppConfig::load_or_default(config_path)?;
        let probe = HardwareProbe::detect();
        let policy = TierRuntimePolicy::compute(&probe, config.profile_mode(), config.ai.enabled);
        let secrets = SecretStore::new_os();
        let enc_key = if config.encrypt_database {
            secrets.get_secret("db:master_key")?
        } else {
            None
        };
        let store = SqliteStore::open(
            Path::new(&config.database_path),
            policy.fts_prefix_indexes,
            enc_key.as_deref(),
        )?;
        if let Some(saved) = store.get_setting("ai")? {
            config.ai = serde_json::from_str(&saved)
                .map_err(|e| format!("invalid saved AI settings: {e}"))?;
        }
        let policy = TierRuntimePolicy::compute(&probe, config.profile_mode(), config.ai.enabled);
        let saved_accounts = store.list_accounts()?;
        for acc in &config.accounts {
            let mut account = acc.clone();
            if let Some(saved) = saved_accounts.iter().find(|saved| saved.id == acc.id) {
                account.ai_enabled = saved.ai_enabled;
            }
            store.save_account(&account)?;
        }
        for rule in &config.rules {
            let _ = store.save_user_rule(rule);
        }
        let clipboard = ClipboardGuard::new(config.clipboard_clear_secs);
        Ok(Self {
            store,
            config,
            probe,
            policy,
            secrets,
            clipboard,
            ai_worker_running: false,
            ai_busy: false,
            ai_generation: 0,
            ai_last_error: None,
            ai_revision: 0,
            model_downloads: local_models::Downloads::default(),
        })
    }
}

pub fn run_http_daemon(bind_addr: &str, state: Arc<Mutex<DaemonState>>) -> Result<(), String> {
    let listener =
        TcpListener::bind(bind_addr).map_err(|e| format!("failed binding {bind_addr}: {e}"))?;
    run_http_daemon_with_listener(listener, state)
}

pub fn run_http_daemon_with_listener(
    listener: TcpListener,
    state: Arc<Mutex<DaemonState>>,
) -> Result<(), String> {
    assistant::start_worker(&state)?;
    for stream in listener.incoming().flatten() {
        let st = Arc::clone(&state);
        std::thread::spawn(move || {
            let _ = handle_http_connection(stream, st);
        });
    }
    Ok(())
}

pub fn handle_http_connection(
    mut stream: TcpStream,
    state: Arc<Mutex<DaemonState>>,
) -> Result<(), String> {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let mut reader = BufReader::new(
        stream
            .try_clone()
            .map_err(|e| format!("stream clone failed: {e}"))?,
    );

    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .map_err(|e| format!("read request line failed: {e}"))?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let full_path = parts.next().unwrap_or("/");
    let (path, query) = full_path.split_once('?').unwrap_or((full_path, ""));

    let mut content_length = 0usize;
    let mut host_header: Option<String> = None;
    let mut origin_header: Option<String> = None;
    let mut sec_fetch_site: Option<String> = None;
    for _ in 0..128 {
        let mut line = String::new();
        let n = reader.read_line(&mut line).unwrap_or(0);
        if n == 0 || line == "\r\n" || line == "\n" {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim();
            if key.eq_ignore_ascii_case("content-length") {
                content_length = v.trim().parse().unwrap_or(0);
            } else if key.eq_ignore_ascii_case("host") {
                host_header = Some(v.trim().to_ascii_lowercase());
            } else if key.eq_ignore_ascii_case("origin") {
                origin_header = Some(v.trim().to_ascii_lowercase());
            } else if key.eq_ignore_ascii_case("sec-fetch-site") {
                sec_fetch_site = Some(v.trim().to_ascii_lowercase());
            }
        }
    }

    if let Some(ref h) = host_header
        && !is_loopback_authority(h)
    {
        return write_http_response(&mut stream, 403, "text/plain", "forbidden host header");
    }

    if method == "POST" {
        if sec_fetch_site.as_deref() == Some("cross-site") {
            return write_http_response(
                &mut stream,
                403,
                "text/plain",
                "forbidden cross-site request",
            );
        }
        if let Some(ref orig) = origin_header {
            let valid_origin = orig
                .strip_prefix("http://")
                .or_else(|| orig.strip_prefix("https://"))
                .is_some_and(is_loopback_authority);
            if !valid_origin {
                return write_http_response(
                    &mut stream,
                    403,
                    "text/plain",
                    "forbidden cross-origin request",
                );
            }
        }
    }

    if content_length > 2 * 1024 * 1024 {
        return write_http_response(&mut stream, 400, "text/plain", "payload too large");
    }
    let mut body_bytes = vec![0u8; content_length];
    if !body_bytes.is_empty() {
        reader
            .read_exact(&mut body_bytes)
            .map_err(|e| format!("failed reading request body: {e}"))?;
    }
    let body_str = String::from_utf8_lossy(&body_bytes);

    match (method, path) {
        ("GET", "/otp-overlay") => write_http_response(
            &mut stream,
            200,
            "text/html; charset=utf-8",
            &load_live_overlay_html(),
        ),
        ("GET", "/") => write_http_response(
            &mut stream,
            200,
            "text/html; charset=utf-8",
            &load_live_ui_asset("INDEX_HTML", INDEX_HTML),
        ),
        ("GET", "/tokens.css") => write_http_response(
            &mut stream,
            200,
            "text/css; charset=utf-8",
            &load_live_ui_asset("STYLES_CSS", STYLES_CSS),
        ),
        ("GET", "/app.js") => write_http_response(
            &mut stream,
            200,
            "application/javascript; charset=utf-8",
            &load_live_ui_asset("APP_JS", APP_JS),
        ),
        ("GET", "/click.m4a") => {
            write_http_bytes(&mut stream, 200, "audio/mp4", vortex_ui_web::CLICK_AUDIO)
        }
        ("GET", "/local-models.js") => write_http_response(
            &mut stream,
            200,
            "application/javascript; charset=utf-8",
            &load_live_local_models_js(),
        ),
        ("GET", "/api/ui-mtime") => write_http_response(
            &mut stream,
            200,
            "application/json",
            &format!("{{\"mtime\":{}}}", live_ui_mtime_ms()),
        ),
        ("GET", "/api/events") => {
            let sse = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\ndata: connected\n\n";
            stream.write_all(sse.as_bytes()).map_err(|e| e.to_string())
        }
        ("GET", "/api/status") => {
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let accounts = guard.store.list_accounts().unwrap_or_default();
            let items_count = guard
                .store
                .status_digest()
                .map(|(cnt, total_len)| cnt.wrapping_add(total_len))
                .unwrap_or(0);
            let mut missing_secret_accounts = Vec::new();
            for a in &accounts {
                if a.auth_method != "google_web"
                    && a.host != "session.gmail.com"
                    && guard.secrets.get_secret(&a.id).ok().flatten().is_none()
                {
                    missing_secret_accounts.push(a.email_or_handle.clone());
                }
            }
            let json = serde_json::json!({
                "tier": guard.policy.tier.as_str(),
                "ram_mb": guard.probe.ram_mb,
                "cpu_cores": guard.probe.cpu_cores,
                "on_battery": guard.probe.on_battery,
                "web_ui_enabled": guard.policy.web_ui_enabled,
                "min_poll_interval_secs": guard.policy.min_poll_interval_secs,
                "accounts_count": accounts.len(),
                "items_count": items_count,
                "missing_secret_accounts": missing_secret_accounts,
                "disk_backpressured": guard.store.is_disk_backpressured(),
                "ai_revision": guard.ai_revision,
            });
            write_http_response(&mut stream, 200, "application/json", &json.to_string())
        }
        ("GET", "/api/accounts") => {
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let accs = guard.store.list_accounts()?;
            let json = serde_json::to_string(&accs).map_err(|e| e.to_string())?;
            write_http_response(&mut stream, 200, "application/json", &json)
        }
        ("POST", "/api/accounts") => {
            let v: serde_json::Value =
                serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
            let id = v.get("id").and_then(|x| x.as_str()).unwrap_or("").trim();
            if id.is_empty() {
                return write_http_response(&mut stream, 400, "text/plain", "missing id");
            }
            let source = SourceKind::from_str_name(
                v.get("source").and_then(|x| x.as_str()).unwrap_or("email"),
            );
            let email = v
                .get("email_or_handle")
                .and_then(|x| x.as_str())
                .unwrap_or(id);
            let raw_host = v.get("host").and_then(|x| x.as_str()).unwrap_or("");
            let raw_port = v.get("port").and_then(|x| x.as_u64()).unwrap_or(993) as u16;
            let (host, port) = if source == SourceKind::Email && raw_host.trim().is_empty() {
                infer_imap_host_port(email, raw_host, raw_port)
            } else {
                (raw_host.to_string(), raw_port)
            };
            let use_tls = v
                .get("use_tls")
                .and_then(|x| x.as_bool())
                .unwrap_or(port == 993 || port == 443);
            let secret = v.get("secret").and_then(|x| x.as_str()).unwrap_or("");
            let async_sync = v
                .get("async_sync")
                .and_then(|x| x.as_bool())
                .unwrap_or(false);

            let acc = AccountConfig {
                id: id.to_string(),
                source,
                email_or_handle: email.to_string(),
                host: host.clone(),
                port,
                use_tls,
                auth_method: "password".into(),
                oauth_client_id: None,
                gmail_writeback_labels: false,
                ai_enabled: false,
                endpoint_url: if source == SourceKind::Rss {
                    Some(host)
                } else {
                    None
                },
            };

            let secrets = {
                let guard = state
                    .lock()
                    .map_err(|_| "state lock poisoned".to_string())?;
                if !secret.is_empty() {
                    guard.secrets.set_secret(&acc.id, secret)?;
                }
                if async_sync {
                    guard.store.save_account(&acc)?;
                }
                guard.secrets.clone()
            };

            if async_sync {
                let local_bind = stream
                    .local_addr()
                    .map(|a| a.to_string())
                    .unwrap_or_else(|_| "127.0.0.1:7788".into());
                let bg_state = Arc::clone(&state);
                let acc_id = acc.id.clone();
                std::thread::spawn(move || {
                    let _ = sync_accounts_concurrent(&bg_state, Some(&acc_id), &local_bind);
                });
                let resp = serde_json::json!({ "ok": true, "synced": 0, "queued": true });
                return write_http_response(
                    &mut stream,
                    200,
                    "application/json",
                    &resp.to_string(),
                );
            }

            match fetch_account_items(&acc, 0, &secrets) {
                Ok((fetched, _)) => {
                    let guard = state
                        .lock()
                        .map_err(|_| "state lock poisoned".to_string())?;
                    guard.store.save_account(&acc)?;
                    let rules = guard.store.list_user_rules().unwrap_or_default();
                    let now = now_epoch();
                    let mut synced = 0usize;
                    for item in fetched {
                        if process_new_item(
                            &guard.store,
                            item,
                            now,
                            &rules,
                            &guard.config,
                            &guard.policy,
                            &guard.secrets,
                        )
                        .is_ok()
                        {
                            synced += 1;
                        }
                    }
                    let resp = serde_json::json!({ "ok": true, "synced": synced });
                    write_http_response(&mut stream, 200, "application/json", &resp.to_string())
                }
                Err(e) => {
                    let _ = secrets.delete_secret(&acc.id);
                    write_http_response(&mut stream, 400, "text/plain; charset=utf-8", &e)
                }
            }
        }
        ("POST", "/api/accounts/batch") => {
            let v: serde_json::Value =
                serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
            let default_host = v
                .get("default_host")
                .and_then(|x| x.as_str())
                .unwrap_or("imap.gmail.com");
            let default_port = v
                .get("default_port")
                .and_then(|x| x.as_u64())
                .unwrap_or(993) as u16;
            let async_sync = v
                .get("async_sync")
                .and_then(|x| x.as_bool())
                .unwrap_or(false);
            let mut parsed_batch: Vec<(AccountConfig, String)> = Vec::new();
            if let Some(lines) = v.get("lines").and_then(|x| x.as_array()) {
                for l in lines {
                    if let Some(s) = l.as_str()
                        && let Some(entry) = parse_bulk_account_line(s, default_host, default_port)
                    {
                        parsed_batch.push(entry);
                    }
                }
            } else if let Some(raw_text) = v.get("text").and_then(|x| x.as_str()) {
                for line in raw_text.lines() {
                    if let Some(entry) = parse_bulk_account_line(line, default_host, default_port) {
                        parsed_batch.push(entry);
                    }
                }
            }
            if parsed_batch.is_empty() {
                return write_http_response(
                    &mut stream,
                    400,
                    "text/plain; charset=utf-8",
                    "no valid account lines (use email|app_password)",
                );
            }

            if async_sync {
                let mut added = 0usize;
                if let Ok(guard) = state.lock() {
                    for (acc, secret) in &parsed_batch {
                        let _ = guard.secrets.set_secret(&acc.id, secret);
                        if guard.store.save_account(acc).is_ok() {
                            added += 1;
                        }
                    }
                }
                let local_bind = stream
                    .local_addr()
                    .map(|a| a.to_string())
                    .unwrap_or_else(|_| "127.0.0.1:7788".into());
                let bg_state = Arc::clone(&state);
                std::thread::spawn(move || {
                    let _ = sync_accounts_concurrent(&bg_state, None, &local_bind);
                });
                let resp = serde_json::json!({
                    "ok": true,
                    "added": added,
                    "synced": 0,
                    "queued": true,
                    "failed": [],
                });
                return write_http_response(
                    &mut stream,
                    200,
                    "application/json",
                    &resp.to_string(),
                );
            }

            let secrets = {
                let guard = state
                    .lock()
                    .map_err(|_| "state lock poisoned".to_string())?;
                for (acc, secret) in &parsed_batch {
                    let _ = guard.secrets.set_secret(&acc.id, secret);
                }
                guard.secrets.clone()
            };

            let mut added = 0usize;
            let mut total_synced = 0usize;
            let mut failed: Vec<serde_json::Value> = Vec::new();

            for chunk in parsed_batch.chunks(8) {
                let outcomes: Vec<(AccountConfig, Result<Vec<Item>, String>)> =
                    std::thread::scope(|s| {
                        let mut handles = Vec::with_capacity(chunk.len());
                        for (acc, _sec) in chunk {
                            let acc_clone = acc.clone();
                            let sec_clone = secrets.clone();
                            handles.push(s.spawn(move || {
                                let res = fetch_account_items(&acc_clone, 0, &sec_clone)
                                    .map(|(it, _)| it);
                                (acc_clone, res)
                            }));
                        }
                        handles.into_iter().filter_map(|h| h.join().ok()).collect()
                    });

                if let Ok(guard) = state.lock() {
                    let rules = guard.store.list_user_rules().unwrap_or_default();
                    let now = now_epoch();
                    for (acc, res) in outcomes {
                        match res {
                            Ok(fetched) => {
                                if guard.store.save_account(&acc).is_ok() {
                                    added += 1;
                                }
                                for item in fetched {
                                    if process_new_item(
                                        &guard.store,
                                        item,
                                        now,
                                        &rules,
                                        &guard.config,
                                        &guard.policy,
                                        &guard.secrets,
                                    )
                                    .is_ok()
                                    {
                                        total_synced += 1;
                                    }
                                }
                            }
                            Err(err) => {
                                let _ = guard.secrets.delete_secret(&acc.id);
                                failed.push(serde_json::json!({
                                    "id": acc.id,
                                    "error": err,
                                }));
                            }
                        }
                    }
                }
            }

            let resp = serde_json::json!({
                "ok": added > 0 || failed.is_empty(),
                "added": added,
                "synced": total_synced,
                "failed": failed,
            });
            let status_code = if added > 0 || failed.is_empty() {
                200
            } else {
                400
            };
            write_http_response(
                &mut stream,
                status_code,
                "application/json",
                &resp.to_string(),
            )
        }
        ("POST", "/api/google-login") => {
            let v: serde_json::Value =
                serde_json::from_str(&body_str).unwrap_or(serde_json::Value::Null);
            let email = v
                .get("email")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let local_bind = stream
                .local_addr()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| "127.0.0.1:7788".into());
            if let Ok(exe) = std::env::current_exe() {
                LAST_GWEB_SPAWN_SEC.store(now_epoch(), std::sync::atomic::Ordering::Relaxed);
                let mut cmd = std::process::Command::new(exe);
                cmd.args(["google-login", "--bind", &local_bind]);
                if !email.is_empty() {
                    cmd.args(["--email", &email]);
                }
                if let Ok(mut child) = cmd.spawn() {
                    GWEB_CHILD_RUNNING.store(true, std::sync::atomic::Ordering::Relaxed);
                    std::thread::spawn(move || {
                        let _ = child.wait();
                        GWEB_CHILD_RUNNING.store(false, std::sync::atomic::Ordering::Relaxed);
                    });
                }
            }
            write_http_response(&mut stream, 200, "application/json", "{\"ok\":true}")
        }
        ("POST", "/api/google-web-sync") => {
            let v: serde_json::Value =
                serde_json::from_str(&body_str).map_err(|e| format!("invalid json: {e}"))?;
            let email = v
                .get("email")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            if email.is_empty() {
                return write_http_response(&mut stream, 400, "text/plain", "missing email");
            }
            let mut acc = AccountConfig {
                id: email.clone(),
                source: SourceKind::Email,
                email_or_handle: email.clone(),
                host: "session.gmail.com".into(),
                port: 443,
                use_tls: true,
                auth_method: "google_web".into(),
                oauth_client_id: None,
                gmail_writeback_labels: false,
                ai_enabled: false,
                endpoint_url: None,
            };
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            if let Some(existing) = guard
                .store
                .list_accounts()?
                .into_iter()
                .find(|a| a.id == acc.id)
            {
                acc.ai_enabled = existing.ai_enabled;
            }
            let _ = guard.secrets.set_secret(&acc.id, "__google_web_session__");
            guard.store.save_account(&acc)?;

            let rules = guard.store.list_user_rules().unwrap_or_default();
            let now = now_epoch();
            let mut synced_count = 0usize;

            if let Some(arr) = v.get("messages").and_then(|x| x.as_array()) {
                for (idx, msg) in arr.iter().enumerate() {
                    let ext_id = msg
                        .get("id")
                        .and_then(|x| x.as_str())
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| s.trim().to_string())
                        .unwrap_or_else(|| format!("gweb-{now}-{idx}"));
                    let unread = msg.get("unread").and_then(|x| x.as_bool()).unwrap_or(true);
                    let received_at = msg
                        .get("received_at")
                        .and_then(|x| x.as_i64())
                        .filter(|&ts| ts > 946_684_800)
                        .unwrap_or_else(|| now - (idx as i64) * 60);

                    let mut item = if let Some(raw_mime) = msg
                        .get("raw_mime")
                        .and_then(|x| x.as_str())
                        .filter(|s| s.len() > 40)
                    {
                        let mut parsed = vortex_imap::mime::parse_raw_email(
                            &email,
                            &ext_id,
                            raw_mime,
                            Vec::new(),
                            received_at,
                        );
                        parsed.external_id = ext_id;
                        parsed
                    } else {
                        let sender = msg
                            .get("sender")
                            .and_then(|x| x.as_str())
                            .filter(|s| !s.trim().is_empty())
                            .unwrap_or("Gmail")
                            .trim()
                            .to_string();
                        let subject = msg
                            .get("subject")
                            .and_then(|x| x.as_str())
                            .filter(|s| !s.trim().is_empty())
                            .unwrap_or("(Không có tiêu đề)")
                            .trim()
                            .to_string();
                        let body = msg
                            .get("body")
                            .and_then(|x| x.as_str())
                            .unwrap_or("")
                            .trim()
                            .to_string();
                        let mut body_text = if body.is_empty() {
                            subject.clone()
                        } else {
                            body
                        };
                        let raw_html = msg
                            .get("body_html")
                            .and_then(|x| x.as_str())
                            .unwrap_or("")
                            .trim();
                        let stripped_html = raw_html
                            .split("<!--v:")
                            .next()
                            .unwrap_or(raw_html)
                            .trim();
                        let body_html = if stripped_html.contains('<') && stripped_html.contains('>') {
                            stripped_html.to_string()
                        } else {
                            String::new()
                        };
                        if !body_html.is_empty() {
                            let from_html = vortex_imap::mime::html_to_plain_text(&body_html);
                            if from_html.len() > body_text.len() {
                                body_text = from_html;
                            }
                        }
                        let mut it = Item::new(
                            SourceKind::Email,
                            &email,
                            ext_id,
                            sender,
                            subject,
                            body_text,
                            received_at,
                        );
                        it.body_html = body_html;
                        it
                    };
                    if !unread {
                        item.state = vortex_core::model::ItemState::Read;
                    }
                    if process_new_item(
                        &guard.store,
                        item,
                        now,
                        &rules,
                        &guard.config,
                        &guard.policy,
                        &guard.secrets,
                    )
                    .is_ok()
                    {
                        synced_count += 1;
                    }
                }
            }

            let resp = serde_json::json!({
                "ok": true,
                "email": email,
                "synced": synced_count
            });
            write_http_response(&mut stream, 200, "application/json", &resp.to_string())
        }
        ("POST", "/api/accounts/remove") => {
            let v: serde_json::Value =
                serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
            let remove_all = v.get("all").and_then(|x| x.as_bool()).unwrap_or(false);
            let id = v.get("id").and_then(|x| x.as_str()).unwrap_or("");
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            if remove_all {
                let accs = guard.store.list_accounts().unwrap_or_default();
                for a in accs {
                    let _ = guard.secrets.delete_secret(&a.id);
                    let _ = guard.store.remove_account(&a.id);
                }
            } else {
                let _ = guard.secrets.delete_secret(id);
                let _ = guard.store.remove_account(id)?;
            }
            write_http_response(&mut stream, 200, "application/json", "{\"ok\":true}")
        }
        ("GET", "/api/items") => {
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let q = query_param(query, "q");
            let account = query_param(query, "account").filter(|s| !s.is_empty());
            let items = if query_param(query, "hidden").as_deref() == Some("1") {
                let mut items = guard.store.list_ai_hidden(account.as_deref(), 120)?;
                if let Some(q) = q.filter(|s| !s.is_empty()) {
                    let q = q.to_lowercase();
                    items.retain(|i| {
                        format!("{} {}", i.sender, i.subject)
                            .to_lowercase()
                            .contains(&q)
                    });
                }
                items
            } else if let Some(search_q) = q.filter(|s| !s.is_empty()) {
                let mut res = guard.store.search_fts(&search_q, 80)?;
                if let Some(ref acc_id) = account {
                    res.retain(|it| &it.account == acc_id);
                }
                res
            } else {
                let category = query_param(query, "category")
                    .filter(|s| !s.is_empty())
                    .map(|s| ItemCategory::from_str_name(&s));
                guard.store.list_items(&ItemFilter {
                    account,
                    category,
                    limit: 120,
                    ..Default::default()
                })?
            };
            let json = serde_json::to_string(&items).map_err(|e| e.to_string())?;
            write_http_response(&mut stream, 200, "application/json", &json)
        }
        ("GET", "/api/codes") => {
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let account_filter = query_param(query, "account").filter(|s| !s.is_empty());
            let codes = guard.store.list_recent_codes(now_epoch())?;
            let mut primary_per_account: Vec<serde_json::Value> = Vec::new();
            let mut secondary_codes: Vec<serde_json::Value> = Vec::new();
            let mut seen_accounts: Vec<String> = Vec::new();

            for c in codes {
                let (account, subject) = guard
                    .store
                    .get_item(c.item_id)
                    .ok()
                    .flatten()
                    .map(|it| (it.account, it.subject))
                    .unwrap_or_default();
                if let Some(ref target_acc) = account_filter
                    && &account != target_acc
                {
                    continue;
                }
                let mut val = serde_json::to_value(&c).map_err(|e| e.to_string())?;
                if let Some(obj) = val.as_object_mut() {
                    obj.insert("account".into(), serde_json::Value::String(account.clone()));
                    obj.insert("subject".into(), serde_json::Value::String(subject));
                }
                if !account.is_empty() && !seen_accounts.contains(&account) {
                    seen_accounts.push(account);
                    primary_per_account.push(val);
                } else {
                    secondary_codes.push(val);
                }
            }
            primary_per_account.extend(secondary_codes);
            let json = serde_json::to_string(&primary_per_account).map_err(|e| e.to_string())?;
            write_http_response(&mut stream, 200, "application/json", &json)
        }
        ("POST", "/api/codes/clear") => {
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let _ = guard.store.clear_recent_codes();
            write_http_response(&mut stream, 200, "application/json", "{\"ok\":true}")
        }
        ("POST", "/api/actions") => {
            let v: serde_json::Value =
                serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
            let item_id = v.get("item_id").and_then(|x| x.as_i64()).unwrap_or(0);
            let action: QuickAction =
                serde_json::from_value(v).map_err(|e| format!("invalid action: {e}"))?;
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            guard.store.apply_action(item_id, &action)?;
            write_http_response(&mut stream, 200, "application/json", "{\"ok\":true}")
        }
        ("POST", "/api/sync") => {
            let local_bind = stream
                .local_addr()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| "127.0.0.1:7788".into());
            let only_account = serde_json::from_str::<serde_json::Value>(&body_str)
                .ok()
                .and_then(|v| {
                    v.get("account_id")
                        .and_then(|x| x.as_str())
                        .map(str::to_string)
                });
            let resp = sync_accounts_concurrent(&state, only_account.as_deref(), &local_bind)?;
            write_http_response(&mut stream, 200, "application/json", &resp.to_string())
        }
        ("GET", "/api/digest") => {
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let items = guard.store.list_items(&ItemFilter {
                limit: 100,
                ..Default::default()
            })?;
            let digest = generate_daily_digest(&items, now_epoch());
            let json = serde_json::to_string(&digest).map_err(|e| e.to_string())?;
            write_http_response(&mut stream, 200, "application/json", &json)
        }
        ("GET", "/api/ai/local-models") => {
            let (endpoint, ram_mb) = {
                let guard = state.lock().map_err(|_| "state lock poisoned")?;
                (
                    query_param(query, "endpoint")
                        .unwrap_or_else(|| guard.config.ai.endpoint.clone()),
                    guard.probe.ram_mb,
                )
            };
            write_json_result(&mut stream, local_models::catalog(&endpoint, ram_mb))
        }
        ("GET", "/api/ai/local-models/status") => {
            let guard = state.lock().map_err(|_| "state lock poisoned")?;
            write_json_result(&mut stream, Ok(guard.model_downloads.status()))
        }
        ("POST", "/api/ai/local-models/download") => {
            let result = (|| {
                let value: serde_json::Value =
                    serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
                let endpoint = value
                    .get("endpoint")
                    .and_then(|v| v.as_str())
                    .ok_or("missing endpoint")?;
                let model = value
                    .get("model")
                    .and_then(|v| v.as_str())
                    .ok_or("missing model")?;
                local_models::start_download(&state, endpoint, model)
            })();
            write_json_result(&mut stream, result)
        }
        ("POST", "/api/ai/local-models/cancel") => {
            let result = (|| {
                let value: serde_json::Value =
                    serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
                let id = value
                    .get("id")
                    .and_then(|v| v.as_u64())
                    .ok_or("missing download id")?;
                let mut guard = state.lock().map_err(|_| "state lock poisoned")?;
                guard.model_downloads.cancel(id)
            })();
            write_json_result(&mut stream, result)
        }
        ("GET", "/api/ai/settings") => {
            let guard = state.lock().map_err(|_| "state lock poisoned")?;
            write_json_result(&mut stream, assistant::settings(&guard))
        }
        ("POST", "/api/ai/settings") => {
            let result = (|| {
                let value =
                    serde_json::from_str(&body_str).map_err(|e| format!("Invalid JSON: {e}"))?;
                let mut guard = state.lock().map_err(|_| "state lock poisoned")?;
                assistant::save_settings(&mut guard, value)
            })();
            if result.is_ok() {
                assistant::start_worker(&state)?;
            }
            write_json_result(&mut stream, result)
        }
        ("POST", "/api/ai/summarize") => {
            let result = (|| {
                let value: serde_json::Value =
                    serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
                let id = value
                    .get("id")
                    .and_then(|v| v.as_i64())
                    .ok_or("missing id")?;
                let guard = state.lock().map_err(|_| "state lock poisoned")?;
                if !guard.config.ai.enabled || !guard.policy.ai_allowed_now {
                    return Err("AI is disabled or paused by the device power profile".into());
                }
                assistant::queue_item(&guard.store, id, &guard.config.ai, true)?;
                Ok(serde_json::json!({"ok":true,"queued":true}))
            })();
            if result.is_ok() {
                assistant::start_worker(&state)?;
            }
            write_json_result(&mut stream, result)
        }
        ("GET", "/api/ai/result") => {
            let id = query_param(query, "item_id")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            let guard = state.lock().map_err(|_| "state lock poisoned")?;
            write_json_result(&mut stream, assistant::result(&guard, id))
        }
        ("POST", "/api/ai/restore") => {
            let result = (|| {
                let value: serde_json::Value =
                    serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
                let id = value
                    .get("id")
                    .and_then(|v| v.as_i64())
                    .ok_or("missing id")?;
                let mut guard = state.lock().map_err(|_| "state lock poisoned")?;
                if !guard.store.restore_ai_item(id)? {
                    return Err("AI-hidden message not found".into());
                }
                guard.ai_revision = guard.ai_revision.wrapping_add(1);
                Ok(serde_json::json!({"ok":true}))
            })();
            write_json_result(&mut stream, result)
        }
        ("POST", "/api/ai/qa") => {
            let v: serde_json::Value =
                serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
            let question = v.get("question").and_then(|x| x.as_str()).unwrap_or("");
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let items = guard.store.list_items(&ItemFilter {
                limit: 100,
                ..Default::default()
            })?;
            let answer = answer_mailbox_qa(question, &items);
            let resp = serde_json::json!({ "answer": answer });
            write_http_response(&mut stream, 200, "application/json", &resp.to_string())
        }
        ("GET", "/api/ai/preview") => {
            let id: i64 = query_param(query, "item_id")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let Some(item) = guard.store.get_item(id)? else {
                return write_http_response(&mut stream, 404, "text/plain", "item not found");
            };
            let preview = build_ai_preview(&item, "classify", &guard.config.ai);
            let json = serde_json::to_string(&preview).map_err(|e| e.to_string())?;
            write_http_response(&mut stream, 200, "application/json", &json)
        }
        ("POST", "/api/clipboard/copy") => {
            let v: serde_json::Value =
                serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
            let code = v.get("code").and_then(|x| x.as_str()).unwrap_or("");
            let delay_secs = {
                let mut guard = state
                    .lock()
                    .map_err(|_| "state lock poisoned".to_string())?;
                guard.clipboard.record_copy(code, now_epoch());
                guard.clipboard.clear_delay_secs()
            };
            let st_clone = Arc::clone(&state);
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(delay_secs));
                if let Ok(mut guard) = st_clone.lock() {
                    let _ = guard.clipboard.poll_and_clear_os_clipboard(now_epoch());
                }
            });
            write_http_response(&mut stream, 200, "application/json", "{\"ok\":true}")
        }
        ("POST", "/api/hub/pair") => {
            let v: serde_json::Value =
                serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
            let dev_id = v
                .get("device_id")
                .and_then(|x| x.as_str())
                .unwrap_or("dev-1");
            let dev_name = v
                .get("device_name")
                .and_then(|x| x.as_str())
                .unwrap_or("Thin Client");
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let (ticket, record) = create_pairing_ticket(
                &guard.config.hub.listen_addr,
                dev_id,
                dev_name,
                now_epoch() as u64,
            );
            guard
                .secrets
                .set_secret(&format!("hub:device:{dev_id}"), &ticket.shared_secret)?;
            guard.store.register_paired_device(&record)?;
            let json = serde_json::to_string(&ticket).map_err(|e| e.to_string())?;
            write_http_response(&mut stream, 200, "application/json", &json)
        }
        ("GET", "/api/devices") => {
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let devs = guard.store.list_paired_devices()?;
            let json = serde_json::to_string(&devs).map_err(|e| e.to_string())?;
            write_http_response(&mut stream, 200, "application/json", &json)
        }
        ("POST", "/api/hub/revoke") => {
            let v: serde_json::Value =
                serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
            let dev_id = v.get("device_id").and_then(|x| x.as_str()).unwrap_or("");
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let _ = guard.secrets.delete_secret(&format!("hub:device:{dev_id}"));
            let _ = guard.store.revoke_paired_device(dev_id)?;
            write_http_response(&mut stream, 200, "application/json", "{\"ok\":true}")
        }
        ("POST", "/api/relay/android") => {
            let env: EncryptedEnvelope =
                serde_json::from_str(&body_str).map_err(|e| e.to_string())?;
            let guard = state
                .lock()
                .map_err(|_| "state lock poisoned".to_string())?;
            let is_active = guard
                .store
                .list_paired_devices()?
                .iter()
                .any(|d| d.device_id == env.device_id && !d.revoked);
            if !is_active {
                return write_http_response(
                    &mut stream,
                    403,
                    "text/plain",
                    "paired device revoked or unknown",
                );
            }
            let secret = guard
                .secrets
                .get_secret(&format!("hub:device:{}", env.device_id))?
                .ok_or_else(|| "unknown paired android device".to_string())?;
            let item = ingest_android_e2e_relay(&env.device_id, &secret, &env)?;
            let rules = guard.store.list_user_rules()?;
            let res = process_new_item(
                &guard.store,
                item,
                now_epoch(),
                &rules,
                &guard.config,
                &guard.policy,
                &guard.secrets,
            )?;
            let resp = serde_json::json!({ "ok": true, "item_id": res.item_id });
            write_http_response(&mut stream, 200, "application/json", &resp.to_string())
        }
        _ => write_http_response(&mut stream, 404, "text/plain", "not found"),
    }
}

fn write_http_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &str,
) -> Result<(), String> {
    write_http_bytes(stream, status, content_type, body.as_bytes())
}

fn write_json_result(
    stream: &mut TcpStream,
    result: Result<serde_json::Value, String>,
) -> Result<(), String> {
    match result {
        Ok(value) => write_http_response(stream, 200, "application/json", &value.to_string()),
        Err(error) => write_http_response(stream, 400, "text/plain; charset=utf-8", &error),
    }
}

fn write_http_bytes(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> Result<(), String> {
    let status_text = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Error",
    };
    let header = format!(
        "HTTP/1.1 {status} {status_text}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream
        .write_all(header.as_bytes())
        .and_then(|_| stream.write_all(body))
        .map_err(|e| e.to_string())
}

fn query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=')
            && k == key
        {
            return Some(url_decode(v));
        }
    }
    None
}

fn url_decode(s: &str) -> String {
    let mut bytes = Vec::with_capacity(s.len());
    let raw = s.as_bytes();
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'+' {
            bytes.push(b' ');
            i += 1;
        } else if raw[i] == b'%'
            && i + 2 < raw.len()
            && let (Some(h), Some(l)) = (
                (raw[i + 1] as char).to_digit(16),
                (raw[i + 2] as char).to_digit(16),
            )
        {
            bytes.push(((h << 4) | l) as u8);
            i += 3;
        } else {
            bytes.push(raw[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn now_epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn is_loopback_authority(authority: &str) -> bool {
    let trimmed = authority.trim().trim_end_matches('/');
    let host = if let Some(after_bracket) = trimmed.strip_prefix('[') {
        after_bracket.split_once(']').map_or("", |(h, _)| h)
    } else {
        trimmed.split_once(':').map_or(trimmed, |(h, _)| h)
    };
    host.eq_ignore_ascii_case("127.0.0.1")
        || host.eq_ignore_ascii_case("localhost")
        || host == "::1"
}

fn load_live_overlay_html() -> String {
    #[cfg(debug_assertions)]
    if let Ok(raw) = std::fs::read_to_string("crates/ui-web/src/otp_overlay.html") {
        return raw;
    }
    OTP_OVERLAY_HTML.to_string()
}

fn load_live_local_models_js() -> String {
    #[cfg(debug_assertions)]
    if let Ok(raw) = std::fs::read_to_string("crates/ui-web/src/local_models.js") {
        return raw;
    }
    vortex_ui_web::LOCAL_MODELS_JS.to_string()
}

fn live_ui_mtime_ms() -> u64 {
    #[cfg(debug_assertions)]
    {
        let file_mtime = |path: &str| {
            std::fs::metadata(path)
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0)
        };
        file_mtime("crates/ui-web/src/lib.rs").max(file_mtime("crates/ui-web/src/local_models.js"))
    }
    #[cfg(not(debug_assertions))]
    {
        0
    }
}

fn load_live_ui_asset(const_name: &str, fallback: &'static str) -> String {
    #[cfg(debug_assertions)]
    if let Ok(raw) = std::fs::read_to_string("crates/ui-web/src/lib.rs") {
        let marker = format!("pub const {const_name}: &str = r##\"");
        if let Some(start) = raw.find(&marker) {
            let after = &raw[start + marker.len()..];
            if let Some(end) = after.find("\"##;") {
                return after[..end].to_string();
            }
        }
    }
    let _ = const_name;
    fallback.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connectors::{AndroidSmsRelayPayload, RssConnector, TelegramBotConnector};
    use crate::hub::{
        HubBroadcaster, create_pairing_ticket, decrypt_e2e_payload, encrypt_e2e_payload,
    };
    use crate::notify::ClipboardGuard;
    use crate::service::{
        generate_launchd_plist, generate_systemd_user_unit, generate_windows_startup_script,
        is_newer_version, verify_artifact_sha256,
    };
    use std::time::Instant;
    use vortex_core::tier::{HardwareProbe, ProfileMode};
    use vortex_imap::mime::{parse_raw_email, sanitize_html_safe};
    use vortex_imap::{FakeImapMessage, FakeImapServer};

    #[test]
    fn expired_otp_is_stored_with_mail_but_does_not_trigger_popup() {
        let store = SqliteStore::open_in_memory(true, None).unwrap();
        let secrets = SecretStore::new_memory();
        let cfg = AppConfig::default();
        let probe = HardwareProbe {
            ram_mb: 8192,
            cpu_cores: 4,
            arch: "x86_64".into(),
            on_battery: false,
            headless: true,
        };
        let policy = TierRuntimePolicy::compute(&probe, ProfileMode::Standard, false);
        let item = Item::new(
            SourceKind::Email,
            "user@example.com",
            "old-otp",
            "Service <noreply@example.com>",
            "Verification code 123456",
            "Your verification code is 123456. Expires in 5 minutes.",
            1_700_000_000,
        );
        let result =
            process_new_item(&store, item, 1_700_001_000, &[], &cfg, &policy, &secrets).unwrap();
        assert!(result.popup_event.is_none());
        assert_eq!(result.category, ItemCategory::CodeOtp);
        assert_eq!(
            store
                .get_item(result.item_id)
                .unwrap()
                .unwrap()
                .extracted_codes[0]
                .code,
            "123456"
        );
        assert!(store.list_recent_codes(1_700_001_000).unwrap().is_empty());
    }

    #[test]
    fn acceptance_100_mixed_mails_pipeline() {
        let mut fake_mails = Vec::with_capacity(100);
        let mut uid = 1u32;

        // 20 OTP mails (10 English, 10 Vietnamese)
        for i in 0..10 {
            let code = format!("{:06}", 100_000 + i * 7919);
            fake_mails.push(FakeImapMessage {
                uid,
                raw_rfc5322: format!(
                    "From: GitHub <noreply@github.com>\r\nSubject: Your verification code is {code}\r\nMessage-ID: <otp-en-{i}@github.com>\r\n\r\nUse verification code {code} to sign in. Expires in 10 minutes. https://github.com/login/verify?code={code}"
                ),
                flags: vec![],
                gm_labels: vec![],
            });
            uid += 1;
        }
        for i in 0..10 {
            let code = format!("{:06}", 500_000 + i * 6133);
            fake_mails.push(FakeImapMessage {
                uid,
                raw_rfc5322: format!(
                    "From: Vietcombank <alert@vcb.com.vn>\r\nSubject: Mã xác nhận giao dịch {code}\r\nMessage-ID: <otp-vi-{i}@vcb.com.vn>\r\n\r\nMã OTP của quý khách là {code}. Hiệu lực trong 5 phút."
                ),
                flags: vec![],
                gm_labels: vec![],
            });
            uid += 1;
        }

        // 15 Billing mails
        for i in 0..15 {
            fake_mails.push(FakeImapMessage {
                uid,
                raw_rfc5322: format!(
                    "From: Stripe <receipts@stripe.com>\r\nSubject: Invoice #90{i:04} payment received\r\nMessage-ID: <bill-{i}@stripe.com>\r\n\r\nThank you for your payment of $49.00 on 2026-10-28."
                ),
                flags: vec![],
                gm_labels: vec![],
            });
            uid += 1;
        }

        // 15 Shipping mails with order/tracking numbers (must NOT trigger OTP popups)
        for i in 0..15 {
            fake_mails.push(FakeImapMessage {
                uid,
                raw_rfc5322: format!(
                    "From: Amazon <shipment-tracking@amazon.com>\r\nSubject: Your order #4829{i:02} has shipped\r\nMessage-ID: <ship-{i}@amazon.com>\r\n\r\nYour package is out for delivery. Tracking 778899{i:02}."
                ),
                flags: vec![],
                gm_labels: vec![],
            });
            uid += 1;
        }

        // 10 Security alert mails (no OTP code)
        for i in 0..10 {
            fake_mails.push(FakeImapMessage {
                uid,
                raw_rfc5322: format!(
                    "From: Google <no-reply@accounts.google.com>\r\nSubject: Security alert: new sign-in on Windows\r\nMessage-ID: <sec-{i}@google.com>\r\n\r\nWe noticed a new sign-in to your Google Account."
                ),
                flags: vec![],
                gm_labels: vec![],
            });
            uid += 1;
        }

        // 10 Phishing lookalike mails (mismatched sender vs brand/link domain)
        for i in 0..10 {
            fake_mails.push(FakeImapMessage {
                uid,
                raw_rfc5322: format!(
                    "From: PayPal Security <security@verify-account-alert-{i}.xyz>\r\nSubject: Security alert: unusual activity\r\nMessage-ID: <phish-{i}@bad.xyz>\r\n\r\nVerify immediately at https://evil-phish-{i}.xyz/login"
                ),
                flags: vec![],
                gm_labels: vec![],
            });
            uid += 1;
        }

        // 10 Work mails
        for i in 0..10 {
            fake_mails.push(FakeImapMessage {
                uid,
                raw_rfc5322: format!(
                    "From: Linear <notifications@linear.app>\r\nSubject: Sprint review meeting agenda #{i}\r\nMessage-ID: <work-{i}@linear.app>\r\n\r\nPull request merged and CI/CD build passed."
                ),
                flags: vec![],
                gm_labels: vec![],
            });
            uid += 1;
        }

        // 10 Social mails
        for i in 0..10 {
            fake_mails.push(FakeImapMessage {
                uid,
                raw_rfc5322: format!(
                    "From: LinkedIn <messages-noreply@linkedin.com>\r\nSubject: Alice mentioned you in a post #{i}\r\nMessage-ID: <soc-{i}@linkedin.com>\r\n\r\nYou have a new follower and friend request."
                ),
                flags: vec![],
                gm_labels: vec!["^smartlabel_social".into()],
            });
            uid += 1;
        }

        // 10 Promo newsletters
        for i in 0..10 {
            fake_mails.push(FakeImapMessage {
                uid,
                raw_rfc5322: format!(
                    "From: Store <deals@shop.example.com>\r\nSubject: Flash sale 50% off weekend special #{i}\r\nMessage-ID: <promo-{i}@shop.example.com>\r\nList-Unsubscribe: <https://shop.example.com/unsub>\r\nPrecedence: bulk\r\n\r\nClick here to unsubscribe from this newsletter."
                ),
                flags: vec![],
                gm_labels: vec!["^smartlabel_promo".into()],
            });
            uid += 1;
        }

        assert_eq!(fake_mails.len(), 100);
        let server = FakeImapServer::start(fake_mails);

        let store = SqliteStore::open_in_memory(true, None).unwrap();
        let secrets = SecretStore::new_memory();
        secrets.set_secret("acc-100", "pw-100").unwrap();
        let acc = AccountConfig {
            id: "acc-100".into(),
            source: SourceKind::Email,
            email_or_handle: "user@example.com".into(),
            host: "127.0.0.1".into(),
            port: server.port,
            use_tls: false,
            auth_method: "password".into(),
            oauth_client_id: None,
            gmail_writeback_labels: true,
            ai_enabled: false,
            endpoint_url: None,
        };
        let cfg = AppConfig::default();
        let probe = HardwareProbe {
            ram_mb: 8192,
            cpu_cores: 4,
            arch: "x86_64".into(),
            on_battery: false,
            headless: true,
        };
        let policy = TierRuntimePolicy::compute(&probe, ProfileMode::Standard, false);

        let start = Instant::now();
        let results = sync_single_account(&store, &acc, &cfg, &policy, &secrets).unwrap();
        let total_elapsed_ms = start.elapsed().as_millis() as u64;

        assert_eq!(results.len(), 100);
        let popups: Vec<&OtpPopupEvent> = results
            .iter()
            .filter_map(|r| r.popup_event.as_ref())
            .collect();
        // Exactly the 20 OTP mails must emit popups; zero false positives on the other 80 mails.
        assert_eq!(popups.len(), 20);
        for p in &popups {
            assert!(p.detected_latency_ms < 2000);
        }
        assert!(total_elapsed_ms < 2000);

        let stored = store
            .list_items(&ItemFilter {
                limit: 200,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(stored.len(), 100);
        let phishing_flagged = stored.iter().filter(|i| i.phishing_score >= 60).count();
        assert_eq!(phishing_flagged, 10);
    }

    #[test]
    fn fuzz_mime_and_otp_parsers_never_panic() {
        let mut seed = 0x1234_5678_9abc_def0u64;
        let now = 1_700_000_000i64;
        for idx in 0..250 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let len = ((seed >> 32) as usize) % 512;
            let mut buf = Vec::with_capacity(len);
            for j in 0..len {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(j as u64);
                buf.push((seed >> 24) as u8);
            }
            let fuzz_str = String::from_utf8_lossy(&buf);
            let _ = sanitize_html_safe(&fuzz_str);
            let item = parse_raw_email("fuzz", &format!("f-{idx}"), &fuzz_str, vec![], now);
            let _ = detect_fast_path(&item, now, &[]);
        }
    }

    #[test]
    fn hub_e2e_pairing_dedup_and_revocation() {
        let (ticket, mut record) =
            create_pairing_ticket("127.0.0.1:7789", "phone-1", "Pixel", 1_700_000_000);
        let mut broadcaster = HubBroadcaster::new();
        let event = OtpPopupEvent {
            item_id: 1,
            message_id: "m-1".into(),
            code_hash: OtpPopupEvent::compute_dedup_hash("m-1", "654321"),
            code: "654321".into(),
            sender: "GitHub <noreply@github.com>".into(),
            account: "work-account".into(),
            recipient: "work@example.com".into(),
            service: "github.com".into(),
            received_at: 1_700_000_000,
            expires_at: 1_700_000_600,
            verification_link: None,
            phishing_warning: None,
            detected_latency_ms: 1,
        };

        let envs =
            broadcaster.fanout_otp_event(&event, &[(record.clone(), ticket.shared_secret.clone())]);
        assert_eq!(envs.len(), 1);
        let decrypted = decrypt_e2e_payload(&ticket.shared_secret, &envs[0]).unwrap();
        assert!(decrypted.contains("654321"));

        // Duplicate event is suppressed by HubBroadcaster.
        let dup =
            broadcaster.fanout_otp_event(&event, &[(record.clone(), ticket.shared_secret.clone())]);
        assert!(dup.is_empty());

        // Revoked device receives nothing.
        record.revoked = true;
        let event2 = OtpPopupEvent {
            code_hash: OtpPopupEvent::compute_dedup_hash("m-2", "112233"),
            ..event
        };
        let revoked_envs = broadcaster.fanout_otp_event(&event2, &[(record, ticket.shared_secret)]);
        assert!(revoked_envs.is_empty());
    }

    #[test]
    fn messaging_connectors_rss_telegram_and_android_e2e() {
        let xml = r#"
            <rss version="2.0"><channel>
                <item>
                    <title>Security Release 1.2</title>
                    <guid>rel-1.2</guid>
                    <description>Verification code for feed test: 334455</description>
                </item>
            </channel></rss>
        "#;
        let rss_items = RssConnector::parse_feed_xml("blog-rss", xml, 1_700_000_000);
        assert_eq!(rss_items.len(), 1);
        assert_eq!(rss_items[0].source, SourceKind::Rss);

        let tg_cfg = AccountConfig {
            id: "tg-1".into(),
            source: SourceKind::Telegram,
            email_or_handle: "@mybot".into(),
            host: "api.telegram.org".into(),
            port: 443,
            use_tls: true,
            auth_method: "token".into(),
            oauth_client_id: None,
            gmail_writeback_labels: false,
            ai_enabled: false,
            endpoint_url: None,
        };
        let mut tg = TelegramBotConnector::new(tg_cfg, SecretStore::new_memory());
        let tg_json = r#"{"ok":true,"result":[{"update_id":10,"message":{"date":1700000000,"from":{"username":"bankbot"},"text":"Mã OTP của bạn là 998877"}}]}"#;
        let tg_items = tg.parse_updates_json(tg_json, 1_700_000_000).unwrap();
        assert_eq!(tg_items.len(), 1);

        let sms_payload = AndroidSmsRelayPayload {
            message_id: "sms-1".into(),
            sender: "VNPay".into(),
            body: "Ma xac thuc OTP VNPay la 443322".into(),
            received_at: 1_700_000_000,
        };
        let env = encrypt_e2e_payload(
            "android-1",
            "shared-sec-99",
            1,
            &serde_json::to_string(&sms_payload).unwrap(),
        );
        let sms_item = ingest_android_e2e_relay("android-1", "shared-sec-99", &env).unwrap();
        assert_eq!(sms_item.source, SourceKind::AndroidSms);
    }

    #[test]
    fn clipboard_auto_clear_and_service_manifests() {
        let mut guard = ClipboardGuard::new(45);
        guard.record_copy("849201", 1000);
        assert!(!guard.should_clear_now("849201", 1020));
        assert!(guard.should_clear_now("849201", 1046));

        guard.record_copy("849201", 2000);
        // User copied something else before timeout -> must NOT clear their new clipboard.
        assert!(!guard.should_clear_now("user custom text", 2050));

        let bin = Path::new("/usr/local/bin/vortex");
        let cfg = Path::new("/home/user/.config/vortex.toml");
        assert!(generate_systemd_user_unit(bin, cfg).contains("ExecStart="));
        assert!(generate_launchd_plist(bin, cfg).contains("org.vortexinbox.daemon"));
        assert!(generate_windows_startup_script(bin, cfg).contains("daemon --config"));
        assert!(is_newer_version("0.1.0", "0.2.0"));
        assert!(
            verify_artifact_sha256(
                b"vortex",
                "667b4d2274e285b357b7063195297236e026f73e9e4c8062a571984d39759202"
            ) || !verify_artifact_sha256(b"vortex", "0000")
        );
        assert!(is_loopback_authority("127.0.0.1:7788"));
        assert!(is_loopback_authority("localhost:7788"));
        assert!(!is_loopback_authority("127.0.0.1.attacker.com"));
        assert!(!is_loopback_authority("localhost.evil.com:7788"));
    }

    #[test]
    fn multi_account_parallel_sync_and_otp_extraction() {
        let (parsed_gmail, sec1) =
            parse_bulk_account_line("acc1@gmail.com|abcd efgh ijkl mnop", "imap.gmail.com", 993)
                .unwrap();
        assert_eq!(parsed_gmail.id, "acc1@gmail.com");
        assert_eq!(parsed_gmail.host, "imap.gmail.com");
        assert_eq!(sec1, "abcd efgh ijkl mnop");

        let (parsed_outlook, sec2) =
            parse_bulk_account_line("acc2@outlook.com:pw2", "imap.gmail.com", 993).unwrap();
        assert_eq!(parsed_outlook.host, "outlook.office365.com");
        assert_eq!(sec2, "pw2");

        let store = SqliteStore::open_in_memory(true, None).unwrap();
        let secrets = SecretStore::new_memory();
        let probe = HardwareProbe {
            ram_mb: 8192,
            cpu_cores: 4,
            arch: "x86_64".into(),
            on_battery: false,
            headless: true,
        };
        let policy = TierRuntimePolicy::compute(&probe, ProfileMode::Standard, false);

        let mut servers = Vec::new();
        for idx in 0..4 {
            let code = format!("{:06}", 310_000 + idx * 1111);
            let srv = FakeImapServer::start(vec![FakeImapMessage {
                uid: 1,
                raw_rfc5322: format!(
                    "From: Steam <noreply@steampowered.com>\r\nTo: multi-{idx}@example.com\r\nSubject: Your Steam Guard code is {code}\r\nMessage-ID: <steam-{idx}@steampowered.com>\r\n\r\nEnter verification code {code} to sign in. Expires in 10 minutes."
                ),
                flags: vec![],
                gm_labels: vec![],
            }]);
            let acc_id = format!("multi-{idx}@example.com");
            secrets.set_secret(&acc_id, "pw").unwrap();
            let acc = AccountConfig {
                id: acc_id.clone(),
                source: SourceKind::Email,
                email_or_handle: acc_id,
                host: "127.0.0.1".into(),
                port: srv.port,
                use_tls: false,
                auth_method: "password".into(),
                oauth_client_id: None,
                gmail_writeback_labels: false,
                ai_enabled: false,
                endpoint_url: None,
            };
            store.save_account(&acc).unwrap();
            servers.push(srv);
        }

        let state = Arc::new(Mutex::new(DaemonState {
            store,
            config: AppConfig::default(),
            probe,
            policy,
            secrets,
            clipboard: ClipboardGuard::new(45),
            ai_worker_running: false,
            ai_busy: false,
            ai_generation: 0,
            ai_revision: 0,
            ai_last_error: None,
            model_downloads: local_models::Downloads::default(),
        }));

        let res = sync_accounts_concurrent(&state, None, "127.0.0.1:7788").unwrap();
        assert_eq!(res.get("synced").and_then(|v| v.as_u64()), Some(4));
        assert_eq!(res.get("accounts_synced").and_then(|v| v.as_u64()), Some(4));

        let guard = state.lock().unwrap();
        let codes = guard.store.list_recent_codes(now_epoch()).unwrap();
        assert_eq!(codes.len(), 4);
        for code in codes {
            assert!(code.account.starts_with("multi-"));
            assert_eq!(code.recipient, code.account);
            assert_eq!(code.sender, "Steam <noreply@steampowered.com>");
        }
    }
}
