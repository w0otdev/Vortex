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
            let desktop_state = Arc::clone(&shared);
            thread::spawn(move || {
                let _ = run_http_daemon_with_listener(listener, shared);
            });
            return run_native_window(&url, desktop_state);
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
fn load_window_icon() -> Option<tao::window::Icon> {
    const RGBA: &[u8] = include_bytes!("../vortex_icon_64.rgba");
    tao::window::Icon::from_rgba(RGBA.to_vec(), 64, 64).ok()
}

#[cfg(target_os = "windows")]
fn run_native_window(url: &str, state: Arc<Mutex<DaemonState>>) -> ExitCode {
    use tao::{
        dpi::{LogicalPosition, LogicalSize},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoopBuilder},
        platform::windows::WindowBuilderExtWindows,
        window::WindowBuilder,
    };
    use wry::{WebContext, WebViewBuilder};

    enum DesktopEvent {
        Show(usize),
        Resize(usize),
        Hide,
        Refresh,
        WindowDrag,
        WindowMinimize,
        WindowToggleMax,
        WindowClose,
    }

    let event_loop = EventLoopBuilder::<DesktopEvent>::with_user_event().build();
    let window = match WindowBuilder::new()
        .with_title("Vortex")
        .with_decorations(false)
        .with_window_icon(load_window_icon())
        .with_taskbar_icon(load_window_icon())
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

    let main_proxy = event_loop.create_proxy();
    let data_dir = env::temp_dir().join("vortex-inbox-webview2");
    let mut web_context = WebContext::new(Some(data_dir));
    let webview = match WebViewBuilder::with_web_context(&mut web_context)
        .with_url(url)
        .with_ipc_handler(move |request| {
            let ev = match request.body().as_str() {
                "win:drag" => DesktopEvent::WindowDrag,
                "win:min" => DesktopEvent::WindowMinimize,
                "win:max" => DesktopEvent::WindowToggleMax,
                "win:close" => DesktopEvent::WindowClose,
                _ => return,
            };
            let _ = main_proxy.send_event(ev);
        })
        .build(&window)
    {
        Ok(wv) => wv,
        Err(e) => {
            eprintln!("failed initializing native webview: {e}");
            return ExitCode::FAILURE;
        }
    };

    let overlay_layout = |window: &tao::window::Window, count: usize| {
        let cards = count.clamp(1, 2);
        let width: f64 = 304.0;
        let height: f64 = if cards >= 2 { 164.0 } else { 84.0 };
        window
            .current_monitor()
            .map(|monitor| {
                let scale = monitor.scale_factor();
                let size = monitor.size().to_logical::<f64>(scale);
                let position = monitor.position().to_logical::<f64>(scale);
                let x = position.x + (size.width - width - 20.0).max(8.0);
                let y = position.y + (size.height - height - 56.0).max(8.0);
                (width, height, LogicalPosition::new(x, y))
            })
            .unwrap_or((width, height, LogicalPosition::new(80.0, 500.0)))
    };
    let (overlay_width, overlay_height, overlay_position) = overlay_layout(&window, 1);

    let overlay_window = match WindowBuilder::new()
        .with_title("Vortex OTP")
        .with_inner_size(LogicalSize::new(overlay_width, overlay_height))
        .with_position(overlay_position)
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(false)
        .with_visible(false)
        .with_focused(false)
        .with_always_on_top(true)
        .with_skip_taskbar(true)
        .build(&event_loop)
    {
        Ok(w) => w,
        Err(e) => {
            eprintln!("failed creating OTP overlay: {e}");
            return ExitCode::FAILURE;
        }
    };
    let overlay_proxy = event_loop.create_proxy();
    let overlay_url = format!("{url}/otp-overlay");
    let overlay_webview = match WebViewBuilder::with_web_context(&mut web_context)
        .with_transparent(true)
        .with_url(&overlay_url)
        .with_ipc_handler(move |request| {
            let body = request.body();
            let event = if let Some(n) = body.strip_prefix("otp-overlay:show:") {
                DesktopEvent::Show(n.parse().unwrap_or(1))
            } else if let Some(n) = body.strip_prefix("otp-overlay:resize:") {
                DesktopEvent::Resize(n.parse().unwrap_or(1))
            } else if body == "otp-overlay:show" {
                DesktopEvent::Show(1)
            } else if body == "otp-overlay:hide" {
                DesktopEvent::Hide
            } else {
                return;
            };
            let _ = overlay_proxy.send_event(event);
        })
        .build(&overlay_window)
    {
        Ok(view) => view,
        Err(e) => {
            eprintln!("failed initializing OTP overlay: {e}");
            return ExitCode::FAILURE;
        }
    };

    // Wake the overlay on changes even when WebView2 throttles hidden-page timers.
    let refresh_proxy = event_loop.create_proxy();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        let mut previous = Vec::new();
        loop {
            let current = state.lock().ok().and_then(|guard| {
                guard.store.list_recent_codes(now_epoch()).ok().map(|codes| {
                    codes.into_iter().map(|code| code.code_hash).collect::<Vec<_>>()
                })
            });
            if let Some(current) = current
                && current != previous
            {
                let count = current.len();
                previous = current;
                let _ = refresh_proxy.send_event(DesktopEvent::Refresh);
                let _ = refresh_proxy.send_event(if count > 0 {
                    DesktopEvent::Show(count.min(2))
                } else {
                    DesktopEvent::Hide
                });
            }
            thread::sleep(Duration::from_millis(800));
        }
    });

    window.set_minimized(false);
    window.set_visible(false);
    window.set_visible(true);
    window.set_always_on_top(true);
    window.set_focus();

    let mut topped_cleared = false;
    event_loop.run(move |event, _, control_flow| {
        let _ = (&webview, &overlay_webview, &web_context);
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
            Event::UserEvent(DesktopEvent::Show(count)) => {
                let (width, height, position) = overlay_layout(&window, count);
                overlay_window.set_inner_size(LogicalSize::new(width, height));
                overlay_window.set_outer_position(position);
                overlay_window.set_always_on_top(true);
                overlay_window.set_visible(true);
                let _ = overlay_webview.evaluate_script("window.refreshOtpOverlay?.()");
            }
            Event::UserEvent(DesktopEvent::Resize(count)) => {
                let (width, height, position) = overlay_layout(&window, count);
                overlay_window.set_inner_size(LogicalSize::new(width, height));
                overlay_window.set_outer_position(position);
            }
            Event::UserEvent(DesktopEvent::Hide) => {
                overlay_window.set_visible(false);
            }
            Event::UserEvent(DesktopEvent::Refresh) => {
                let _ = overlay_webview.evaluate_script("window.refreshOtpOverlay?.()");
            }
            Event::UserEvent(DesktopEvent::WindowDrag) => {
                let _ = window.drag_window();
            }
            Event::UserEvent(DesktopEvent::WindowMinimize) => {
                window.set_minimized(true);
            }
            Event::UserEvent(DesktopEvent::WindowToggleMax) => {
                window.set_maximized(!window.is_maximized());
            }
            Event::UserEvent(DesktopEvent::WindowClose) => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } if window_id == overlay_window.id() => overlay_window.set_visible(false),
            _ => {}
        }
    });
}

#[cfg(not(target_os = "windows"))]
fn run_native_window(_url: &str, _state: Arc<Mutex<DaemonState>>) -> ExitCode {
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
            std::thread::sleep(Duration::from_secs(24));
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
        const rawAtomId = ((en.querySelector("id") || {{}}).textContent || "").trim();
        const linkEl = en.querySelector("link[href]");
        const linkHref = (linkEl && linkEl.getAttribute("href")) || "";
        let hexId = "";
        const msgIdMatch = linkHref.match(/[?&](?:message_id|th)=([0-9a-fA-F]{{12,24}})/);
        if (msgIdMatch) {{
          hexId = msgIdMatch[1].toLowerCase();
        }} else {{
          const decMatch = rawAtomId.match(/:(\d{{15,22}})$/);
          if (decMatch) {{
            try {{ hexId = BigInt(decMatch[1]).toString(16); }} catch (_) {{}}
          }}
        }}
        const id = hexId || rawAtomId || ("atom-" + idx);
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
          hexId,
          sender,
          subject,
          body: summary || subject,
          body_html: "",
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

  let additionalSyncPromise = null;

  async function syncAdditionalSilentAccounts(primaryEmail) {{
    if (!IS_SILENT || !window.ipc) return;
    if (additionalSyncPromise) return additionalSyncPromise;
    additionalSyncPromise = (async () => {{
      const targets = (DEFAULT_EMAIL || "").split(",").map((s) => s.trim()).filter(Boolean);
      const urls = ["/mail/u/0/feed/atom", "/mail/u/1/feed/atom", "/mail/u/2/feed/atom", "/mail/u/3/feed/atom"];
      for (const em of targets) {{
        urls.push("/mail/feed/atom?authuser=" + encodeURIComponent(em));
      }}
      const allowed = new Set(targets.map((t) => t.toLowerCase()));
      const seenEmails = new Set(primaryEmail ? [primaryEmail.toLowerCase()] : []);
      for (const u of urls) {{
        const parsed = await parseAtomUrl(u);
        const emLower = parsed && parsed.email ? parsed.email.toLowerCase() : "";
        if (parsed && emLower && !seenEmails.has(emLower) && (allowed.size === 0 || allowed.has(emLower))) {{
          seenEmails.add(emLower);
          const entries = parsed.entries || [];
          const baseMatch = u.match(/^\/mail\/u\/\d+\//);
          const resolved = await resolveIkAndBase(baseMatch ? baseMatch[0] : "", parsed.email);
          await enrichFullBodies(entries, resolved.ik, resolved.base, parsed.email);
          window.ipc.postMessage("WEB_SYNC_PROGRESS:" + JSON.stringify({{
            email: parsed.email,
            messages: entries
          }}));
        }}
      }}
    }})();
    return additionalSyncPromise;
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
        const thEl = tr.querySelector("[data-thread-id],[data-thread-perm-id]");
        const rawTh = (thEl && (thEl.getAttribute("data-thread-id") || thEl.getAttribute("data-thread-perm-id"))) || "";
        const decMatch = rawTh.match(/(\d{{15,22}})/);
        if (decMatch) {{
          try {{ hexId = BigInt(decMatch[1]).toString(16); }} catch (_) {{}}
        }}
      }}
      const extId = hexId
        || ("gmail-msg-" + (senderEmail || senderName || "gmail").toLowerCase() + "-" + subject.toLowerCase());
      let rowTs = unread && idx === 0 ? nowSec : (nowSec - 3600 - idx * 120);
      const dateEl = tr.querySelector("td.xW span[title], td.xW span");
      const rawDate = dateEl ? (dateEl.getAttribute("title") || dateEl.innerText || "").trim() : "";
      if (rawDate) {{
        const parsedMs = Date.parse(rawDate);
        if (Number.isFinite(parsedMs) && parsedMs > 946684800000) {{
          rowTs = Math.floor(parsedMs / 1000);
        }} else {{
          const tm = rawDate.match(/(\d{{1,2}}):(\d{{2}})(?:\s*(AM|PM|SA|CH))?/i);
          if (tm) {{
            let hh = parseInt(tm[1], 10);
            const mm = parseInt(tm[2], 10);
            const ap = (tm[3] || "").toUpperCase();
            if ((ap === "PM" || ap === "CH") && hh < 12) hh += 12;
            if ((ap === "AM" || ap === "SA") && hh === 12) hh = 0;
            const d = new Date();
            d.setHours(hh, mm, 0, 0);
            let ts = Math.floor(d.getTime() / 1000);
            if (ts > nowSec + 120) ts -= 86400;
            rowTs = ts;
          }}
        }}
      }}
      out.push({{
        id: extId,
        hexId,
        sender,
        subject,
        body: snippet || subject,
        body_html: "",
        received_at: rowTs,
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
    try {{
      const resEntries = performance.getEntriesByType("resource") || [];
      for (const re of resEntries) {{
        const m = (re.name || "").match(/[?&]ik=([0-9a-f]{{6,20}})/i);
        if (m) return m[1];
      }}
    }} catch (_) {{}}
    const html = (document.documentElement && document.documentElement.innerHTML) || "";
    const m0 = html.match(/GLOBALS\s*=\s*\[(?:[^,\]]*,){{9}}\s*["']([0-9a-f]{{6,20}})["']/i);
    if (m0) return m0[1];
    const m1 = html.match(/["']([0-9a-f]{{8,16}})["']\s*,\s*["'][a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{{2,}}["']/i);
    if (m1) return m1[1];
    const m2 = html.match(/[?&]ik=([0-9a-f]{{6,20}})/i);
    return m2 ? m2[1] : "";
  }}

  async function resolveIkAndBase(hintBase, userEmail) {{
    if (hintBase === mailBasePath() && (!userEmail || userEmail.toLowerCase() === detectEmail("").toLowerCase())) {{
      const localIk = detectIk(userEmail);
      if (localIk) return {{ ik: localIk, base: mailBasePath() }};
    }}
    const fetchUrl = userEmail
      ? ("/mail/?authuser=" + encodeURIComponent(userEmail))
      : (hintBase || mailBasePath());
    try {{
      const ctrl = new AbortController();
      const t = setTimeout(() => ctrl.abort(), 5500);
      const res = await fetch(fetchUrl, {{ credentials: "include", signal: ctrl.signal }});
      clearTimeout(t);
      if (!res.ok) return {{ ik: "", base: hintBase || "/mail/u/0/" }};
      let resolvedBase = hintBase || "/mail/u/0/";
      if (res.url) {{
        const um = res.url.match(/\/mail\/u\/\d+\//);
        if (um) resolvedBase = um[0];
      }}
      const html = await res.text();
      if (!resolvedBase || resolvedBase === "/mail/u/0/") {{
        const bm = html.match(/\/mail\/u\/\d+\//);
        if (bm) resolvedBase = bm[0];
      }}
      if (userEmail) {{
        const lowHtml = html.toLowerCase();
        const lowEmail = userEmail.toLowerCase();
        if (!lowHtml.includes(lowEmail)) {{
          return {{ ik: "", base: resolvedBase, signedOut: true }};
        }}
        let pos = lowHtml.indexOf(lowEmail);
        while (pos !== -1) {{
          const windowBefore = html.slice(Math.max(0, pos - 80), pos);
          const m = windowBefore.match(/["']([0-9a-f]{{8,20}})["']\s*,\s*["']$/i);
          if (m) return {{ ik: m[1], base: resolvedBase }};
          pos = lowHtml.indexOf(lowEmail, pos + lowEmail.length);
        }}
      }}
      const m1 = html.match(/["']([0-9a-f]{{8,20}})["']\s*,\s*["'][a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{{2,}}["']/i);
      if (m1) return {{ ik: m1[1], base: resolvedBase }};
      const m0 = html.match(/GLOBALS\s*=\s*\[(?:[^,\]]*,){{9}}\s*["']([0-9a-f]{{6,20}})["']/i);
      if (m0) return {{ ik: m0[1], base: resolvedBase }};
      const m2 = html.match(/[?&]ik=([0-9a-f]{{6,20}})/i);
      if (m2) return {{ ik: m2[1], base: resolvedBase }};
      return {{ ik: "", base: resolvedBase }};
    }} catch (_) {{}}
    return {{ ik: "", base: hintBase || "/mail/u/0/" }};
  }}

  function cleanExtractedNodes(root) {{
    root.querySelectorAll("script, style, meta, title, link, [style*='display:none'], [style*='visibility:hidden']").forEach((n) => n.remove());
    let cleanHtml = (root.innerHTML || "").trim();
    let cleanTxt = (root.innerText || root.textContent || "").trim();
    if (/^(?:<!doctype|<html)/i.test(cleanTxt)) {{
      const innerDoc = new DOMParser().parseFromString(cleanTxt, "text/html");
      innerDoc.querySelectorAll("script, style, meta, title, link, [style*='display:none'], [style*='visibility:hidden']").forEach((n) => n.remove());
      if (innerDoc.body) {{
        cleanHtml = (innerDoc.body.innerHTML || "").trim();
        cleanTxt = (innerDoc.body.innerText || innerDoc.body.textContent || "").trim();
      }}
    }}
    return {{ cleanHtml, cleanTxt }};
  }}

  function extractPrintViewBody(html) {{
    const doc = new DOMParser().parseFromString(html, "text/html");
    const msgTables = Array.from(doc.querySelectorAll("table.message"));
    if (msgTables.length > 0) {{
      const lastTbl = msgTables[msgTables.length - 1];
      const trs = Array.from(lastTbl.querySelectorAll(":scope > tbody > tr, :scope > tr"));
      const bodyNode = trs.length > 0 ? trs[trs.length - 1] : lastTbl;
      const res = cleanExtractedNodes(bodyNode);
      if (res.cleanTxt.length > 10) return res;
    }}
    const specificNode = doc.querySelector(".maincontent, .a3s, .ii.gt, .msg");
    if (specificNode) {{
      const res = cleanExtractedNodes(specificNode);
      if (res.cleanTxt.length > 10) return res;
    }}
    return null;
  }}

  async function enrichFullBodies(rows, ik, customBase, userEmail) {{
    if (!rows || !rows.length) return;
    let base = customBase || mailBasePath();
    let activeIk = ik || "";
    if (!activeIk) {{
      const resolved = await resolveIkAndBase(base, userEmail || "");
      activeIk = resolved.ik;
      if (resolved.base) base = resolved.base;
    }}
    const targets = rows.slice(0, 20).filter((r) => r.hexId || /^[0-9a-f]{{12,24}}$/i.test(r.id || ""));
    for (let i = 0; i < targets.length; i += 6) {{
      const batch = targets.slice(i, i + 6);
      await Promise.all(batch.map(async (r) => {{
        const thId = r.hexId || r.id;
        const urls = [];
        if (activeIk) {{
          urls.push(base + "?ui=2&ik=" + encodeURIComponent(activeIk) + "&view=pt&search=all&th=" + encodeURIComponent(thId));
        }}
        if (userEmail) {{
          const authParam = "authuser=" + encodeURIComponent(userEmail);
          if (activeIk) {{
            urls.push("/mail/?ui=2&" + authParam + "&ik=" + encodeURIComponent(activeIk) + "&view=pt&search=all&th=" + encodeURIComponent(thId));
          }}
        }}
        urls.push(base + "h/vortex/?v=pt&th=" + encodeURIComponent(thId));
        urls.push(base + "h/vortex/?v=c&th=" + encodeURIComponent(thId));
        for (const url of urls) {{
          const ctrl = new AbortController();
          const timer = setTimeout(() => ctrl.abort(), 3500);
          try {{
            const res = await fetch(url, {{ credentials: "include", signal: ctrl.signal }});
            clearTimeout(timer);
            if (!res.ok) continue;
            const text = await res.text();
            const parsed = extractPrintViewBody(text);
            if (parsed && (parsed.cleanHtml.length > 40 || parsed.cleanTxt.length > (r.body || "").length)) {{
              if (parsed.cleanHtml.length > 40) r.body_html = parsed.cleanHtml;
              if (parsed.cleanTxt.length > (r.body || "").length) r.body = parsed.cleanTxt;
              break;
            }}
          }} catch (_) {{
            clearTimeout(timer);
          }}
        }}
      }}));
    }}
  }}

  function mergeMessages(domRows, atom) {{
    const merged = [];
    const byKey = new Map();
    const normKey = (item) => (item.hexId || item.id || item.subject || "").toLowerCase();
    for (const r of domRows) {{
      const k = normKey(r);
      byKey.set(k, r);
      merged.push(r);
    }}
    if (atom && atom.entries) {{
      for (const a of atom.entries) {{
        const k = normKey(a);
        const existing = byKey.get(k) || merged.find((m) => (m.subject || "").toLowerCase() === (a.subject || "").toLowerCase());
        if (existing) {{
          if (a.hexId) {{
            existing.hexId = a.hexId;
            existing.id = a.hexId;
          }}
          if (a.body && a.body.length > (existing.body || "").length) {{
            existing.body = a.body;
          }}
          if (a.body_html && a.body_html.length > (existing.body_html || "").length) {{
            existing.body_html = a.body_html;
          }}
          if (a.received_at) existing.received_at = a.received_at;
        }} else {{
          byKey.set(k, a);
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
      ensureHelperBar("Vortex: Chọn hoặc đăng nhập tài khoản Google của bạn.");
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

      if (!IS_SILENT && !stage1Sent && email && window.ipc) {{
        stage1Sent = true;
        const fastMsgs = mergeMessages(domRows, atom);
        window.ipc.postMessage("WEB_SYNC_PROGRESS:" + JSON.stringify({{
          email,
          messages: fastMsgs
        }}));
      }}

      const ik = detectIk(email);
      const mergedNow = mergeMessages(domRows, atom);
      const readyToFinish = (mergedNow.length > 0 && (Boolean(ik) || mailTicks >= 10)) || (mailTicks >= 14 && email);
      if (readyToFinish && email && window.ipc) {{
        submitted = true;
        await Promise.all([
          mergedNow.length > 0 ? enrichFullBodies(mergedNow, ik, mailBasePath(), email) : Promise.resolve(),
          syncAdditionalSilentAccounts(email)
        ]);
        window.ipc.postMessage("WEB_SYNC_DONE:" + JSON.stringify({{
          email,
          messages: mergedNow
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
        .with_title("Đăng nhập với Google - Vortex")
        .with_window_icon(load_window_icon())
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
