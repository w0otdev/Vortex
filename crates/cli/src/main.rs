#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use vortex_classify::parse_user_rule_dsl;
use vortex_core::DeviceTier;
use vortex_core::config::AccountConfig;
use vortex_core::model::{ItemCategory, ItemFilter, QuickAction, SourceKind};
use vortex_daemon::service::install_user_service;
use vortex_daemon::{
    DaemonState, build_tokio_runtime, run_http_daemon, run_http_daemon_with_listener,
    sync_single_account,
};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("daemon");

    match cmd {
        "--version" | "-V" => {
            println!("vortex {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        "--hold-ms" => {
            let ms = args
                .get(1)
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(500);
            thread::sleep(Duration::from_millis(ms));
            ExitCode::SUCCESS
        }
        "--check-budgets" => run_check_budgets(&args),
        "status" => run_status(&args),
        "account" => run_account(&args),
        "sync" => run_sync(&args),
        "inbox" => run_inbox(&args),
        "codes" | "latest-code" => run_codes(&args),
        "action" => run_action(&args),
        "rule" => run_rule(&args),
        "install-service" => run_install_service(&args),
        "google-login" => run_google_login_window(&args),
        "daemon" => run_daemon(&args),
        _ => {
            println!(
                "Usage: vortex <status|account|sync|inbox|codes|action|rule|install-service|daemon|--check-budgets>"
            );
            ExitCode::SUCCESS
        }
    }
}

fn run_check_budgets(args: &[String]) -> ExitCode {
    let tier_name = args.get(1).map(String::as_str).unwrap_or("lite");
    let Some(tier) = DeviceTier::from_name(tier_name) else {
        eprintln!("unknown tier: {tier_name}");
        return ExitCode::FAILURE;
    };
    let binary_bytes = flag_u64(args, "--binary-bytes").unwrap_or(0);
    let rss_bytes = flag_u64(args, "--rss-bytes").unwrap_or(0);
    let web_ui_bytes = match tier {
        DeviceTier::Lite => 0,
        DeviceTier::Standard | DeviceTier::Full => vortex_ui_web::bundle_size_bytes(),
    };
    let budget = vortex_daemon::active_budget(tier);
    if let Err(err) = budget.check(binary_bytes, rss_bytes, web_ui_bytes) {
        eprintln!("BUDGET_FAIL: {err}");
        return ExitCode::FAILURE;
    }
    println!(
        "BUDGET_OK tier={} binary_kb={:.1} rss_kb={:.1} web_ui_kb={:.1}",
        tier.as_str(),
        binary_bytes as f64 / 1024.0,
        rss_bytes as f64 / 1024.0,
        web_ui_bytes as f64 / 1024.0
    );
    ExitCode::SUCCESS
}

fn open_state(args: &[String]) -> Result<DaemonState, String> {
    let cfg_path = flag_str(args, "--config").unwrap_or("vortex.toml");
    DaemonState::open_from_config(Path::new(cfg_path))
}

fn run_status(args: &[String]) -> ExitCode {
    let st = match open_state(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let accounts = st.store.list_accounts().unwrap_or_default();
    let items = st
        .store
        .list_items(&ItemFilter {
            limit: 100,
            ..Default::default()
        })
        .unwrap_or_default();
    println!(
        "tier={} ram_mb={} cores={} arch={} battery={} web_ui={} min_poll_s={} accounts={} items={}",
        st.policy.tier.as_str(),
        st.probe.ram_mb,
        st.probe.cpu_cores,
        st.probe.arch,
        st.probe.on_battery,
        st.policy.web_ui_enabled,
        st.policy.min_poll_interval_secs,
        accounts.len(),
        items.len()
    );
    ExitCode::SUCCESS
}

fn run_account(args: &[String]) -> ExitCode {
    let sub = args.get(1).map(String::as_str).unwrap_or("list");
    let st = match open_state(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };

    match sub {
        "list" => {
            for a in st.store.list_accounts().unwrap_or_default() {
                println!("{}\t{}\t{}:{}", a.id, a.source.as_str(), a.host, a.port);
            }
            ExitCode::SUCCESS
        }
        "add" => {
            let Some(id) = args.get(2) else {
                eprintln!(
                    "usage: vortex account add <email> <host> <port> <secret> [--source email|telegram|rss]"
                );
                return ExitCode::FAILURE;
            };
            let host = args
                .get(3)
                .cloned()
                .unwrap_or_else(|| "imap.gmail.com".into());
            let port: u16 = args.get(4).and_then(|p| p.parse().ok()).unwrap_or(993);
            let secret = args.get(5).cloned().unwrap_or_default();
            let source = flag_str(args, "--source")
                .map(SourceKind::from_str_name)
                .unwrap_or(SourceKind::Email);

            if !secret.is_empty()
                && let Err(e) = st.secrets.set_secret(id, &secret)
            {
                eprintln!("failed storing secret in OS keyring: {e}");
                return ExitCode::FAILURE;
            }
            let acc = AccountConfig {
                id: id.clone(),
                source,
                email_or_handle: id.clone(),
                host: host.clone(),
                port,
                use_tls: port == 993 || port == 443,
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
            if let Err(e) = st.store.save_account(&acc) {
                eprintln!("failed saving account: {e}");
                return ExitCode::FAILURE;
            }
            println!("connected account {id}");
            ExitCode::SUCCESS
        }
        "remove" => {
            let Some(id) = args.get(2) else {
                eprintln!("usage: vortex account remove <id>");
                return ExitCode::FAILURE;
            };
            let _ = st.secrets.delete_secret(id);
            let _ = st.store.remove_account(id);
            println!("removed account {id}");
            ExitCode::SUCCESS
        }
        _ => ExitCode::FAILURE,
    }
}

fn run_sync(args: &[String]) -> ExitCode {
    let st = match open_state(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let accounts = st.store.list_accounts().unwrap_or_default();
    for acc in &accounts {
        match sync_single_account(&st.store, acc, &st.config, &st.policy, &st.secrets) {
            Ok(res) => println!("synced {}: {} new items", acc.id, res.len()),
            Err(e) => eprintln!("sync failed for {}: {e}", acc.id),
        }
    }
    ExitCode::SUCCESS
}

fn run_inbox(args: &[String]) -> ExitCode {
    let st = match open_state(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let items = if let Some(q) = flag_str(args, "--q") {
        st.store.search_fts(q, 50).unwrap_or_default()
    } else {
        let category = flag_str(args, "--category").map(ItemCategory::from_str_name);
        st.store
            .list_items(&ItemFilter {
                category,
                limit: 50,
                ..Default::default()
            })
            .unwrap_or_default()
    };
    for item in items {
        println!(
            "#{} [{}] {} | {} | {}",
            item.id,
            item.category.as_str(),
            item.state.as_str(),
            item.sender,
            item.subject
        );
    }
    ExitCode::SUCCESS
}

fn run_codes(args: &[String]) -> ExitCode {
    let st = match open_state(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let now = now_epoch();
    let codes = st.store.list_recent_codes(now).unwrap_or_default();
    for c in codes {
        let rem = (c.expires_at - now).max(0);
        println!("{}\t{}\texpires_in={}s", c.code, c.service, rem);
    }
    ExitCode::SUCCESS
}

fn run_action(args: &[String]) -> ExitCode {
    let Some(act_name) = args.get(1).map(String::as_str) else {
        eprintln!("usage: vortex action <archive|delete|mark_read|pin|unpin|report_spam> <id>");
        return ExitCode::FAILURE;
    };
    let Some(item_id) = args.get(2).and_then(|s| s.parse::<i64>().ok()) else {
        eprintln!("invalid item id");
        return ExitCode::FAILURE;
    };
    let action = match act_name {
        "archive" => QuickAction::Archive,
        "delete" => QuickAction::Delete,
        "mark_read" | "read" => QuickAction::MarkRead,
        "pin" => QuickAction::Pin,
        "unpin" => QuickAction::Unpin,
        "report_spam" | "spam" => QuickAction::ReportSpam,
        _ => {
            eprintln!("unsupported action: {act_name}");
            return ExitCode::FAILURE;
        }
    };
    let st = match open_state(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(e) = st.store.apply_action(item_id, &action) {
        eprintln!("action failed: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run_rule(args: &[String]) -> ExitCode {
    let sub = args.get(1).map(String::as_str).unwrap_or("list");
    let st = match open_state(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    if sub == "add" {
        let Some(name) = args.get(2) else {
            eprintln!("usage: vortex rule add <name> \"<dsl>\"");
            return ExitCode::FAILURE;
        };
        let Some(dsl) = args.get(3) else {
            eprintln!("usage: vortex rule add <name> \"<dsl>\"");
            return ExitCode::FAILURE;
        };
        match parse_user_rule_dsl(name, dsl) {
            Ok(rule) => {
                let _ = st.store.save_user_rule(&rule);
                println!("saved rule {name}");
            }
            Err(e) => {
                eprintln!("invalid rule DSL: {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        for r in st.store.list_user_rules().unwrap_or_default() {
            println!("{}: {:?}", r.name, r);
        }
    }
    ExitCode::SUCCESS
}

fn run_install_service(args: &[String]) -> ExitCode {
    let target_dir = args.get(1).map(String::as_str).unwrap_or(".");
    let cfg_path = flag_str(args, "--config").unwrap_or("vortex.toml");
    let exe = env::current_exe().unwrap_or_else(|_| Path::new("vortex").to_path_buf());
    match install_user_service(Path::new(target_dir), &exe, Path::new(cfg_path)) {
        Ok(written) => {
            println!("installed service manifest: {}", written.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("service install failed: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run_daemon(args: &[String]) -> ExitCode {
    let st = match open_state(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let bind = flag_str(args, "--bind")
        .map(str::to_string)
        .unwrap_or_else(|| st.config.bind_addr.clone());
    let single_instance =
        args.iter().any(|a| a == "--single-instance") || !args.iter().any(|a| a == "--allow-multi");
    let prebound_listener = match TcpListener::bind(&bind) {
        Ok(l) => Some(l),
        Err(e) => {
            if single_instance {
                eprintln!("Vortex is already running on {bind} ({e})");
                return ExitCode::SUCCESS;
            }
            TcpListener::bind("127.0.0.1:0").ok()
        }
    };

    let web_ui_enabled = st.policy.web_ui_enabled;
    let poll_secs = st.policy.min_poll_interval_secs.max(30);
    let rt = match build_tokio_runtime(&st.policy) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("runtime init error: {e}");
            return ExitCode::FAILURE;
        }
    };

    let shared = Arc::new(Mutex::new(st));
    let bg_shared = Arc::clone(&shared);
    let bg_bind = bind.clone();
    rt.spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(poll_secs));
        loop {
            interval.tick().await;
            let _ = vortex_daemon::sync_accounts_concurrent(&bg_shared, None, &bg_bind);
        }
    });

    if web_ui_enabled {
        let no_window = args.iter().any(|a| a == "--no-window" || a == "--headless");
        if !no_window {
            let Some(listener) = prebound_listener else {
                eprintln!("http bind error on {bind}");
                return ExitCode::FAILURE;
            };
            let bound_addr = listener
                .local_addr()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| bind.clone());
            let url = format!("http://{bound_addr}");
            println!("Vortex desktop app starting ({url})");
            thread::spawn(move || {
                let _ = run_http_daemon_with_listener(listener, shared);
            });
            return run_native_window(&url);
        }

        let url = format!("http://{bind}");
        println!("Vortex daemon listening on {url}");
        if let Some(listener) = prebound_listener {
            if let Err(e) = run_http_daemon_with_listener(listener, shared) {
                eprintln!("http daemon error: {e}");
                return ExitCode::FAILURE;
            }
        } else if let Err(e) = run_http_daemon(&bind, shared) {
            eprintln!("http daemon error: {e}");
            return ExitCode::FAILURE;
        }
    } else {
        println!("Vortex headless/LITE daemon running (poll={}s)", poll_secs);
        loop {
            thread::sleep(Duration::from_secs(poll_secs));
        }
    }
    ExitCode::SUCCESS
}

#[cfg(target_os = "windows")]
fn run_native_window(url: &str) -> ExitCode {
    use tao::{
        dpi::{LogicalPosition, LogicalSize},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    use wry::{WebContext, WebViewBuilder};

    let event_loop = EventLoop::new();
    let window = match WindowBuilder::new()
        .with_title("Vortex Inbox")
        .with_inner_size(LogicalSize::new(1260.0, 820.0))
        .with_position(LogicalPosition::new(80.0, 50.0))
        .with_visible(true)
        .with_focused(true)
        .build(&event_loop)
    {
        Ok(w) => w,
        Err(e) => {
            eprintln!("failed creating native window: {e}");
            return ExitCode::FAILURE;
        }
    };

    let data_dir = env::temp_dir().join("vortex-inbox-webview2");
    let mut web_context = WebContext::new(Some(data_dir));
    let webview = match WebViewBuilder::with_web_context(&mut web_context)
        .with_url(url)
        .build(&window)
    {
        Ok(wv) => wv,
        Err(e) => {
            eprintln!("failed initializing native webview: {e}");
            return ExitCode::FAILURE;
        }
    };

    window.set_minimized(false);
    window.set_visible(false);
    window.set_visible(true);
    window.set_always_on_top(true);
    window.set_focus();

    let mut topped_cleared = false;
    event_loop.run(move |event, _, control_flow| {
        let _ = (&webview, &web_context);
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) | Event::Resumed => {
                window.set_minimized(false);
                window.set_visible(true);
                window.set_always_on_top(true);
                window.set_focus();
            }
            Event::RedrawEventsCleared if !topped_cleared => {
                topped_cleared = true;
                window.set_always_on_top(false);
                window.set_focus();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

#[cfg(not(target_os = "windows"))]
fn run_native_window(_url: &str) -> ExitCode {
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}

#[cfg(target_os = "windows")]
fn run_google_login_window(args: &[String]) -> ExitCode {
    use tao::{
        dpi::{LogicalPosition, LogicalSize},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    use wry::{WebContext, WebViewBuilder};

    let bind = flag_str(args, "--bind")
        .unwrap_or("127.0.0.1:7788")
        .to_string();
    let initial_email = flag_str(args, "--email").unwrap_or("").trim().to_string();
    let silent = args.iter().any(|a| a == "--silent");

    if silent {
        std::thread::spawn(|| {
            std::thread::sleep(Duration::from_secs(12));
            std::process::exit(0);
        });
    }

    let start_url = if silent {
        let first_email = initial_email.split(',').next().unwrap_or("").trim();
        if first_email.is_empty() {
            "https://mail.google.com/mail/u/0/".to_string()
        } else {
            format!(
                "https://mail.google.com/mail/?authuser={}",
                first_email.replace('@', "%40")
            )
        }
    } else if initial_email.is_empty() {
        "https://accounts.google.com/AccountChooser?service=mail&continue=https%3A%2F%2Fmail.google.com%2Fmail%2F"
            .to_string()
    } else {
        format!(
            "https://accounts.google.com/AccountChooser?service=mail&Email={}&continue=https%3A%2F%2Fmail.google.com%2Fmail%2F",
            initial_email.replace('@', "%40")
        )
    };

    let init_js = format!(
        r#"(() => {{
  const DEFAULT_EMAIL = {default_email:?};
  const IS_SILENT = {is_silent};
  let stage1Sent = false;
  let stage2Sent = false;
  let submitted = false;
  let hiddenSent = false;
  let mailTicks = 0;
  let atomCache = null;
  let atomFetching = false;

  if (location.hostname === "mail.google.com") {{
    try {{
      if (window.ipc && !hiddenSent) {{
        hiddenSent = true;
        window.ipc.postMessage("HIDE_WINDOW");
      }}
      const st = document.createElement("style");
      st.textContent = "html, body {{ opacity: 0 !important; pointer-events: none !important; background: #f5f4f0 !important; }}";
      (document.head || document.documentElement).appendChild(st);
    }} catch (_) {{}}
  }}

  function mailBasePath() {{
    const m = (location.pathname || "").match(/^\/mail\/u\/\d+\//);
    return m ? m[0] : "/mail/u/0/";
  }}

  function detectEmail(atomEmail) {{
    if (atomEmail && atomEmail.includes("@")) return atomEmail.trim();
    const titleMatch = (document.title || "").match(/[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{{2,}}/);
    if (titleMatch) return titleMatch[0];
    if (Array.isArray(window.GLOBALS)) {{
      const gMail = window.GLOBALS.find((x) => typeof x === "string" && /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{{2,}}$/.test(x));
      if (gMail) return gMail;
    }}
    const el = document.querySelector('[data-email],[data-identifier],a[aria-label*="@"],button[aria-label*="@"]');
    if (el) {{
      const raw = el.getAttribute("data-email") || el.getAttribute("data-identifier") || el.getAttribute("aria-label") || "";
      const m = raw.match(/[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{{2,}}/);
      if (m) {{
        try {{ localStorage.setItem("vortex_google_email", m[0]); }} catch (_) {{}}
        return m[0];
      }}
    }}
    const bodyTxt = (document.body && document.body.innerText) || "";
    const m = bodyTxt.match(/[a-zA-Z0-9._%+-]+@gmail\.com/);
    if (m) {{
      try {{ localStorage.setItem("vortex_google_email", m[0]); }} catch (_) {{}}
      return m[0];
    }}
    const firstDefault = (DEFAULT_EMAIL || "").split(",")[0].trim();
    if (firstDefault) return firstDefault;
    try {{
      const saved = localStorage.getItem("vortex_google_email");
      if (saved) return saved;
    }} catch (_) {{}}
    return "";
  }}

  function ensureHelperBar(text) {{
    if (!document.body || location.hostname === "mail.google.com") return;
    let bar = document.getElementById("vortexGoogleBar");
    if (!bar) {{
      bar = document.createElement("div");
      bar.id = "vortexGoogleBar";
      bar.style.cssText = "position:fixed;top:0;left:0;right:0;z-index:999999;background:#165640;color:#ffffff;padding:10px 16px;font-family:system-ui,sans-serif;font-size:14px;font-weight:500;display:flex;align-items:center;justify-content:space-between;";
      document.body.appendChild(bar);
      document.body.style.paddingTop = "44px";
    }}
    bar.textContent = text;
  }}

  async function parseAtomUrl(url) {{
    try {{
      const res = await fetch(url, {{ credentials: "include" }});
      if (!res.ok) return null;
      const xmlText = await res.text();
      const doc = new DOMParser().parseFromString(xmlText, "text/xml");
      const feedTitle = (doc.querySelector("feed > title") || {{}}).textContent || "";
      const emailMatch = feedTitle.match(/[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{{2,}}/);
      let atomEmail = emailMatch ? emailMatch[0] : "";
      if (!atomEmail) {{
        const linkEl = doc.querySelector("link[href*='account_id=']");
        if (linkEl) {{
          const href = linkEl.getAttribute("href") || "";
          const m = href.match(/account_id=([^&]+)/);
          if (m) atomEmail = decodeURIComponent(m[1]);
        }}
      }}
      const entries = Array.from(doc.querySelectorAll("entry")).map((en, idx) => {{
        const id = ((en.querySelector("id") || {{}}).textContent || "").trim() || ("atom-" + idx);
        const subject = ((en.querySelector("title") || {{}}).textContent || "").trim() || "(Không có tiêu đề)";
        const summary = ((en.querySelector("summary") || {{}}).textContent || "").trim();
        const issued = ((en.querySelector("issued") || en.querySelector("modified") || {{}}).textContent || "").trim();
        const authorName = ((en.querySelector("author > name") || {{}}).textContent || "").trim();
        const authorEmail = ((en.querySelector("author > email") || {{}}).textContent || "").trim();
        const sender = (authorName && authorEmail && !authorName.includes(authorEmail))
          ? (authorName + " <" + authorEmail + ">")
          : (authorEmail || authorName || "Gmail");
        const parsedTs = issued ? Math.floor(Date.parse(issued) / 1000) : 0;
        return {{
          id,
          sender,
          subject,
          body: summary || subject,
          received_at: Number.isFinite(parsedTs) && parsedTs > 0 ? parsedTs : (Math.floor(Date.now() / 1000) - idx * 60),
          unread: true
        }};
      }});
      return {{ email: atomEmail, entries }};
    }} catch (_) {{
      return null;
    }}
  }}

  async function fetchAtomFeed() {{
    if (atomCache || atomFetching) return atomCache;
    atomFetching = true;
    const base = mailBasePath();
    atomCache = await parseAtomUrl(base + "feed/atom");
    atomFetching = false;
    return atomCache;
  }}

  async function syncAdditionalSilentAccounts(primaryEmail) {{
    if (!IS_SILENT || !window.ipc) return;
    const targets = (DEFAULT_EMAIL || "").split(",").map((s) => s.trim()).filter(Boolean);
    const urls = ["/mail/u/0/feed/atom", "/mail/u/1/feed/atom", "/mail/u/2/feed/atom", "/mail/u/3/feed/atom"];
    for (const em of targets) {{
      urls.push("/mail/feed/atom?authuser=" + encodeURIComponent(em));
    }}
    const seenEmails = new Set(primaryEmail ? [primaryEmail.toLowerCase()] : []);
    await Promise.all(urls.map(async (u) => {{
      const parsed = await parseAtomUrl(u);
      if (parsed && parsed.email && !seenEmails.has(parsed.email.toLowerCase())) {{
        seenEmails.add(parsed.email.toLowerCase());
        window.ipc.postMessage("WEB_SYNC_PROGRESS:" + JSON.stringify({{
          email: parsed.email,
          messages: parsed.entries || []
        }}));
      }}
    }}));
  }}

  function collectDomRows() {{
    const nowSec = Math.floor(Date.now() / 1000);
    const rows = Array.from(document.querySelectorAll("tr.zA"));
    const out = [];
    rows.slice(0, 30).forEach((tr, idx) => {{
      const unread = tr.classList.contains("zE");
      const senderEl = tr.querySelector("[email],[data-hovercard-id]");
      const senderEmail = senderEl
        ? (senderEl.getAttribute("email") || senderEl.getAttribute("data-hovercard-id") || "").trim()
        : "";
      const senderName = senderEl
        ? (senderEl.getAttribute("name") || senderEl.innerText || "").trim()
        : ((tr.querySelector(".yW, .bA4, .zF, .yP") || {{}}).innerText || "").trim();
      const sender = (senderName && senderEmail && !senderName.includes(senderEmail))
        ? (senderName + " <" + senderEmail + ">")
        : (senderEmail || senderName || "Gmail");
      const subjEl = tr.querySelector("span.bog") || tr.querySelector(".bqe");
      const subject = subjEl ? (subjEl.innerText || subjEl.textContent || "").trim() : "(Không có tiêu đề)";
      const snipEl = tr.querySelector("span.y2");
      let snippet = snipEl ? (snipEl.innerText || snipEl.textContent || "") : "";
      snippet = snippet.replace(/^[\s\u00a0—–\-:]+/, "").trim();
      const legacyEl = tr.querySelector("[data-legacy-thread-id],[data-legacy-last-non-draft-message-id]");
      let hexId = tr.getAttribute("data-legacy-thread-id")
        || (legacyEl && (legacyEl.getAttribute("data-legacy-thread-id") || legacyEl.getAttribute("data-legacy-last-non-draft-message-id")))
        || "";
      if (!hexId) {{
        const thEl = tr.querySelector("[data-thread-id]");
        const rawTh = (thEl && thEl.getAttribute("data-thread-id")) || "";
        const decMatch = rawTh.match(/(\d{{15,22}})/);
        if (decMatch) {{
          try {{ hexId = BigInt(decMatch[1]).toString(16); }} catch (_) {{}}
        }}
      }}
      const extId = hexId
        || tr.id
        || ("gmail-row-" + idx + "-" + subject.slice(0, 20));
      out.push({{
        id: extId,
        hexId,
        sender,
        subject,
        body: snippet || subject,
        body_html: "",
        received_at: nowSec - idx * 60,
        unread
      }});
    }});
    return out;
  }}

  function detectIk(userEmail) {{
    if (Array.isArray(window.GLOBALS)) {{
      if (userEmail) {{
        const idx = window.GLOBALS.indexOf(userEmail);
        if (idx > 0 && typeof window.GLOBALS[idx - 1] === "string" && /^[0-9a-f]{{6,20}}$/i.test(window.GLOBALS[idx - 1])) {{
          return window.GLOBALS[idx - 1];
        }}
      }}
      if (typeof window.GLOBALS[9] === "string" && /^[0-9a-f]{{6,20}}$/i.test(window.GLOBALS[9])) {{
        return window.GLOBALS[9];
      }}
    }}
    const html = (document.documentElement && document.documentElement.innerHTML) || "";
    const m1 = html.match(/["']([0-9a-f]{{8,16}})["']\s*,\s*["'][a-zA-Z0-9._%+-]+@gmail\.com["']/i);
    if (m1) return m1[1];
    const m2 = html.match(/[?&]ik=([0-9a-f]{{6,20}})/i);
    return m2 ? m2[1] : "";
  }}

  async function enrichFullBodies(rows, ik) {{
    if (!ik || !rows.length) return;
    const base = mailBasePath();
    const targets = rows.slice(0, 6).filter((r) => r.hexId);
    await Promise.all(targets.map(async (r) => {{
      const ctrl = new AbortController();
      const timer = setTimeout(() => ctrl.abort(), 1200);
      try {{
        const url = base + "?ui=2&ik=" + encodeURIComponent(ik) + "&view=pt&search=all&th=" + encodeURIComponent(r.hexId);
        const res = await fetch(url, {{ credentials: "include", signal: ctrl.signal }});
        if (!res.ok) return;
        const html = await res.text();
        const doc = new DOMParser().parseFromString(html, "text/html");
        doc.querySelectorAll("script, meta, title").forEach((n) => n.remove());
        const msgTables = Array.from(doc.querySelectorAll("table.message"));
        let bodyNode = null;
        if (msgTables.length > 0) {{
          const lastTbl = msgTables[msgTables.length - 1];
          const trs = Array.from(lastTbl.querySelectorAll(":scope > tbody > tr, :scope > tr"));
          bodyNode = trs.length > 0 ? trs[trs.length - 1] : lastTbl;
        }} else {{
          bodyNode = doc.querySelector(".maincontent") || doc.body;
        }}
        if (bodyNode) {{
          const cleanHtml = (bodyNode.innerHTML || "").trim();
          const cleanTxt = (bodyNode.innerText || bodyNode.textContent || "").trim();
          if (cleanHtml.length > 20) {{
            r.body_html = cleanHtml;
          }}
          if (cleanTxt.length > (r.body || "").length) {{
            r.body = cleanTxt;
          }}
        }}
      }} catch (_) {{
      }} finally {{
        clearTimeout(timer);
      }}
    }}));
  }}

  function mergeMessages(domRows, atom) {{
    const merged = [];
    const seenKeys = new Set();
    for (const r of domRows) {{
      const key = (r.subject || "").toLowerCase();
      if (atom && atom.entries) {{
        const match = atom.entries.find((a) => (a.subject || "").toLowerCase() === key);
        if (match && match.body && match.body.length > r.body.length) {{
          r.body = match.body;
          r.received_at = match.received_at || r.received_at;
        }}
      }}
      seenKeys.add(key);
      merged.push(r);
    }}
    if (atom && atom.entries) {{
      for (const a of atom.entries) {{
        const key = (a.subject || "").toLowerCase();
        if (!seenKeys.has(key)) {{
          seenKeys.add(key);
          merged.push(a);
        }}
      }}
    }}
    return merged;
  }}

  async function pollAndSync() {{
    if (submitted) return;

    if (location.hostname.includes("myaccount.google.com")) {{
      if (window.ipc && !hiddenSent) {{
        hiddenSent = true;
        window.ipc.postMessage("HIDE_WINDOW");
      }}
      location.replace("https://mail.google.com/mail/");
      return;
    }}

    if (location.hostname.includes("accounts.google.com")) {{
      ensureHelperBar("Vortex Inbox: Chọn hoặc đăng nhập tài khoản Google của bạn.");
      detectEmail("");
      return;
    }}

    if (location.hostname === "mail.google.com") {{
      if (window.ipc && !hiddenSent) {{
        hiddenSent = true;
        window.ipc.postMessage("HIDE_WINDOW");
      }}
      mailTicks += 1;
      const atom = await fetchAtomFeed();
      const domRows = collectDomRows();
      const email = detectEmail(atom ? atom.email : "");

      if (!stage1Sent && email && window.ipc) {{
        stage1Sent = true;
        const fastMsgs = mergeMessages(domRows, atom);
        window.ipc.postMessage("WEB_SYNC_PROGRESS:" + JSON.stringify({{
          email,
          messages: fastMsgs
        }}));
        syncAdditionalSilentAccounts(email);
      }}

      if (!stage2Sent && domRows.length > 0 && email && window.ipc) {{
        stage2Sent = true;
        window.ipc.postMessage("WEB_SYNC_PROGRESS:" + JSON.stringify({{
          email,
          messages: mergeMessages(domRows, atom)
        }}));
      }}

      const readyToFinish = (domRows.length > 0) || (mailTicks >= 8 && email);
      if (readyToFinish && email && window.ipc) {{
        submitted = true;
        const ik = detectIk(email);
        if (domRows.length > 0 && ik) {{
          await enrichFullBodies(domRows, ik);
        }}
        window.ipc.postMessage("WEB_SYNC_DONE:" + JSON.stringify({{
          email,
          messages: mergeMessages(domRows, atom)
        }}));
      }}
    }}
  }}

  window.addEventListener("DOMContentLoaded", () => {{
    pollAndSync();
    setInterval(pollAndSync, 200);
  }});
  setInterval(pollAndSync, 220);
}})();"#,
        default_email = initial_email,
        is_silent = if silent { "true" } else { "false" }
    );

    let event_loop = EventLoop::new();
    let proxy = event_loop.create_proxy();
    let mut wb = WindowBuilder::new()
        .with_title("Đăng nhập với Google - Vortex Inbox")
        .with_inner_size(LogicalSize::new(520.0, 680.0));
    wb = if silent {
        wb.with_position(LogicalPosition::new(-10000.0, -10000.0))
            .with_decorations(false)
            .with_visible(false)
            .with_focused(false)
    } else {
        wb.with_position(LogicalPosition::new(260.0, 70.0))
            .with_visible(true)
            .with_focused(true)
    };
    let window = match wb.build(&event_loop) {
        Ok(w) => w,
        Err(_) => return ExitCode::FAILURE,
    };

    let data_dir = env::temp_dir().join("vortex-google-signin-profile");
    let mut web_context = WebContext::new(Some(data_dir));
    let bind_for_ipc = bind.clone();
    let fallback_email = initial_email.clone();

    let webview = match WebViewBuilder::with_web_context(&mut web_context)
        .with_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .with_initialization_script(&init_js)
        .with_ipc_handler(move |req| {
            let body = req.body();
            if body == "HIDE_WINDOW" {
                let _ = proxy.send_event(());
                return;
            }
            if let Some(json_payload) = body
                .strip_prefix("WEB_SYNC_PROGRESS:")
                .or_else(|| body.strip_prefix("WEB_SYNC_DONE:"))
                .or_else(|| body.strip_prefix("WEB_SYNC:"))
            {
                let is_done = !body.starts_with("WEB_SYNC_PROGRESS:");
                let _ = proxy.send_event(());
                if let Ok(mut stream) = TcpStream::connect(&bind_for_ipc) {
                    let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
                    let http_req = format!(
                        "POST /api/google-web-sync HTTP/1.1\r\nHost: {bind_for_ipc}\r\nOrigin: http://127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{json_payload}",
                        json_payload.len()
                    );
                    let _ = stream.write_all(http_req.as_bytes());
                    let mut resp = Vec::new();
                    let _ = stream.read_to_end(&mut resp);
                }
                if is_done {
                    std::process::exit(0);
                }
            } else if let Some((raw_email, raw_secret)) = body.split_once('\n') {
                let mut email = raw_email.trim().to_string();
                if email.is_empty() {
                    email = fallback_email.clone();
                }
                let secret = raw_secret.trim().to_string();
                if !email.is_empty() && !secret.is_empty() {
                    let payload = format!(
                        "{{\"id\":\"{email}\",\"source\":\"email\",\"email_or_handle\":\"{email}\",\"host\":\"imap.gmail.com\",\"port\":993,\"use_tls\":true,\"secret\":\"{secret}\"}}"
                    );
                    if let Ok(mut stream) = TcpStream::connect(&bind_for_ipc) {
                        let _ = stream.set_read_timeout(Some(Duration::from_secs(20)));
                        let http_req = format!(
                            "POST /api/accounts HTTP/1.1\r\nHost: {bind_for_ipc}\r\nOrigin: http://127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{payload}",
                            payload.len()
                        );
                        let _ = stream.write_all(http_req.as_bytes());
                        let mut resp = Vec::new();
                        let _ = stream.read_to_end(&mut resp);
                    }
                    std::process::exit(0);
                }
            }
        })
        .with_url(&start_url)
        .build(&window)
    {
        Ok(wv) => wv,
        Err(_) => return ExitCode::FAILURE,
    };

    if !silent {
        window.set_minimized(false);
        window.set_visible(true);
        window.set_always_on_top(true);
        window.set_focus();
    }

    let mut topped_cleared = false;
    let mut window_hidden = silent;
    event_loop.run(move |event, _, control_flow| {
        let _ = (&webview, &web_context);
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(()) => {
                window_hidden = true;
                window.set_always_on_top(false);
                window.set_visible(false);
                window.set_outer_position(LogicalPosition::new(-10000.0, -10000.0));
            }
            Event::NewEvents(StartCause::Init) | Event::Resumed if !window_hidden => {
                window.set_minimized(false);
                window.set_visible(true);
                window.set_always_on_top(true);
                window.set_focus();
            }
            Event::RedrawEventsCleared if !topped_cleared && !window_hidden => {
                topped_cleared = true;
                window.set_always_on_top(false);
                window.set_focus();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

#[cfg(not(target_os = "windows"))]
fn run_google_login_window(_args: &[String]) -> ExitCode {
    ExitCode::SUCCESS
}

fn flag_u64(args: &[String], flag: &str) -> Option<u64> {
    args.windows(2)
        .find(|pair| pair[0] == flag)
        .and_then(|pair| pair[1].parse::<u64>().ok())
}

fn flag_str<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].as_str())
}

fn now_epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
