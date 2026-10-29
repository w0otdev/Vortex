use super::*;
use serde::Serialize;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use vortex_classify::ai::{list_ollama_models, pull_ollama_model, validate_ai_config};
use vortex_core::config::AiConfig;

#[derive(Serialize)]
struct ModelOption {
    id: &'static str,
    name: &'static str,
    size_bytes: u64,
    source: &'static str,
}

// Published Q4_K_M download sizes; runtime memory and Ollama installation are additional.
const MODELS: &[ModelOption] = &[
    ModelOption {
        id: "qwen2.5:0.5b",
        name: "Qwen2.5 0.5B",
        size_bytes: 398_000_000,
        source: "https://ollama.com/library/qwen2.5:0.5b",
    },
    ModelOption {
        id: "qwen2.5:1.5b",
        name: "Qwen2.5 1.5B",
        size_bytes: 986_000_000,
        source: "https://ollama.com/library/qwen2.5:1.5b",
    },
];

#[derive(Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Phase {
    Downloading,
    Canceling,
    Canceled,
    Done,
    Failed,
}

#[derive(Clone, Serialize)]
struct Download {
    id: u64,
    model: String,
    endpoint: String,
    status: Phase,
    completed: u64,
    total: u64,
    message: String,
    error: Option<String>,
    #[serde(skip)]
    cancel: Arc<AtomicBool>,
}

#[derive(Default)]
pub(super) struct Downloads {
    sequence: u64,
    current: Option<Download>,
}

impl Downloads {
    pub(super) fn status(&self) -> Value {
        self.current
            .as_ref()
            .map(|d| json!(d))
            .unwrap_or_else(|| json!({"status":"idle"}))
    }

    pub(super) fn cancel(&mut self, id: u64) -> Result<Value, String> {
        let job = self
            .current
            .as_mut()
            .filter(|d| d.id == id)
            .ok_or("Download not found")?;
        if matches!(job.status, Phase::Downloading | Phase::Canceling) {
            job.cancel.store(true, Ordering::Relaxed);
            job.status = Phase::Canceling;
        }
        Ok(self.status())
    }
}

fn validate_endpoint(endpoint: &str) -> Result<(), String> {
    validate_ai_config(&AiConfig {
        provider: "ollama".into(),
        endpoint: endpoint.into(),
        ..Default::default()
    })
}

pub(super) fn catalog(endpoint: &str, system_ram_mb: u64) -> Result<Value, String> {
    validate_endpoint(endpoint)?;
    let (installed, error) = match list_ollama_models(endpoint) {
        Ok(names) => (names, None),
        Err(first_err) => {
            if is_default_ollama_endpoint(endpoint)
                && let Some(bin) = find_ollama_binary()
            {
                spawn_ollama_serve(&bin);
                let mut names_ok = None;
                for _ in 0..6 {
                    std::thread::sleep(std::time::Duration::from_millis(250));
                    if let Ok(names) = list_ollama_models(endpoint) {
                        names_ok = Some(names);
                        break;
                    }
                }
                if let Some(names) = names_ok {
                    (names, None)
                } else {
                    (Vec::new(), Some(first_err))
                }
            } else {
                (Vec::new(), Some(first_err))
            }
        }
    };
    let models: Vec<Value> = MODELS
        .iter()
        .map(|model| {
            let mut value = json!(model);
            value["installed"] = json!(installed.iter().any(|name| name == model.id));
            value
        })
        .collect();
    Ok(
        json!({"models":models,"available":error.is_none(),"error":error,"system_ram_mb":system_ram_mb}),
    )
}

fn is_default_ollama_endpoint(endpoint: &str) -> bool {
    endpoint.trim().trim_end_matches('/').ends_with(":11434")
}

fn find_ollama_binary() -> Option<std::path::PathBuf> {
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    let mut candidates = Vec::new();
    #[cfg(target_os = "windows")]
    {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            candidates.push(
                PathBuf::from(local_app_data)
                    .join("Programs")
                    .join("Ollama")
                    .join("ollama.exe"),
            );
        }
        if let Ok(prog_files) = std::env::var("ProgramFiles") {
            candidates.push(PathBuf::from(prog_files).join("Ollama").join("ollama.exe"));
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        candidates.push(PathBuf::from("/opt/homebrew/bin/ollama"));
        candidates.push(PathBuf::from("/usr/local/bin/ollama"));
        candidates.push(PathBuf::from("/usr/bin/ollama"));
    }
    for path in candidates {
        if path.is_file() {
            return Some(path);
        }
    }
    let mut cmd = Command::new("ollama");
    cmd.arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    if cmd.status().is_ok_and(|s| s.success()) {
        return Some(PathBuf::from("ollama"));
    }
    None
}

fn spawn_ollama_serve(bin: &std::path::Path) {
    use std::process::{Command, Stdio};
    let mut cmd = Command::new(bin);
    cmd.arg("serve")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    let _ = cmd.spawn();
}

fn ensure_ollama_ready(
    endpoint: &str,
    cancel: &AtomicBool,
    mut on_status: impl FnMut(&str),
) -> Result<(), String> {
    if !is_default_ollama_endpoint(endpoint) || list_ollama_models(endpoint).is_ok() {
        return Ok(());
    }
    let mut bin = find_ollama_binary();
    #[cfg(target_os = "windows")]
    if bin.is_none() && !cancel.load(Ordering::Relaxed) {
        use std::os::windows::process::CommandExt;
        use std::process::{Command, Stdio};
        on_status("Installing Ollama...");
        let _ = Command::new("winget")
            .args([
                "install",
                "-e",
                "--id",
                "Ollama.Ollama",
                "--silent",
                "--accept-package-agreements",
                "--accept-source-agreements",
                "--disable-interactivity",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(0x0800_0000)
            .status();
        bin = find_ollama_binary();
        if bin.is_none() && !cancel.load(Ordering::Relaxed) {
            let installer = std::env::temp_dir().join("OllamaSetup.exe");
            let dl_ok = Command::new("curl.exe")
                .args([
                    "-fsSL",
                    "-o",
                    &installer.to_string_lossy(),
                    "https://ollama.com/download/OllamaSetup.exe",
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .creation_flags(0x0800_0000)
                .status()
                .is_ok_and(|s| s.success());
            if dl_ok && !cancel.load(Ordering::Relaxed) {
                let _ = Command::new(&installer)
                    .args(["/SP-", "/VERYSILENT", "/SUPPRESSMSGBOXES", "/NORESTART"])
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .creation_flags(0x0800_0000)
                    .status();
                let _ = std::fs::remove_file(&installer);
                bin = find_ollama_binary();
            }
        }
    }
    if let Some(bin_path) = bin {
        on_status("Starting Ollama...");
        spawn_ollama_serve(&bin_path);
        for _ in 0..50 {
            if cancel.load(Ordering::Relaxed) {
                return Err("Model download canceled".into());
            }
            if list_ollama_models(endpoint).is_ok() {
                return Ok(());
            }
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
    }
    Ok(())
}

pub(super) fn start_download(
    state: &Arc<Mutex<DaemonState>>,
    endpoint: &str,
    model: &str,
) -> Result<Value, String> {
    validate_endpoint(endpoint)?;
    if !MODELS.iter().any(|option| option.id == model) {
        return Err("Choose a supported local model from the download list".into());
    }
    let mut guard = state.lock().map_err(|_| "state lock poisoned")?;
    let downloads = &mut guard.model_downloads;
    if downloads
        .current
        .as_ref()
        .is_some_and(|d| matches!(d.status, Phase::Downloading | Phase::Canceling))
    {
        return Err("A model download is already running".into());
    }
    downloads.sequence = downloads.sequence.wrapping_add(1);
    let id = downloads.sequence;
    let cancel = Arc::new(AtomicBool::new(false));
    downloads.current = Some(Download {
        id,
        model: model.into(),
        endpoint: endpoint.into(),
        status: Phase::Downloading,
        completed: 0,
        total: 0,
        message: String::new(),
        error: None,
        cancel: Arc::clone(&cancel),
    });
    let endpoint = endpoint.to_string();
    let model = model.to_string();
    let weak = Arc::downgrade(state);
    if let Err(error) = std::thread::Builder::new()
        .name("vortex-model-download".into())
        .spawn(move || {
            let set_msg = |msg: &str| {
                if let Some(state) = weak.upgrade()
                    && let Ok(mut guard) = state.lock()
                    && let Some(job) = guard
                        .model_downloads
                        .current
                        .as_mut()
                        .filter(|d| d.id == id)
                {
                    job.message = msg.chars().filter(|c| !c.is_control()).take(160).collect();
                }
            };
            let outcome = if cancel.load(Ordering::Relaxed) {
                Err("Model download canceled".to_string())
            } else if let Err(err) = ensure_ollama_ready(&endpoint, &cancel, set_msg) {
                Err(err)
            } else {
                pull_ollama_model(&endpoint, &model, |progress| {
                    if cancel.load(Ordering::Relaxed) {
                        return false;
                    }
                    let Some(state) = weak.upgrade() else {
                        return false;
                    };
                    let Ok(mut guard) = state.lock() else {
                        return false;
                    };
                    let Some(job) = guard
                        .model_downloads
                        .current
                        .as_mut()
                        .filter(|d| d.id == id)
                    else {
                        return false;
                    };
                    job.message = progress
                        .status
                        .chars()
                        .filter(|c| !c.is_control())
                        .take(160)
                        .collect();
                    if progress.total > 0 {
                        job.total = progress.total;
                        job.completed = progress.completed.min(progress.total);
                    }
                    !job.cancel.load(Ordering::Relaxed)
                })
            };
            if let Some(state) = weak.upgrade()
                && let Ok(mut guard) = state.lock()
                && let Some(job) = guard
                    .model_downloads
                    .current
                    .as_mut()
                    .filter(|d| d.id == id)
            {
                if cancel.load(Ordering::Relaxed) {
                    job.status = Phase::Canceled;
                } else if let Err(error) = outcome {
                    job.status = Phase::Failed;
                    job.error = Some(error.chars().take(500).collect());
                } else {
                    job.status = Phase::Done;
                }
            }
        })
    {
        if let Some(job) = downloads.current.as_mut() {
            job.status = Phase::Failed;
            job.error = Some(format!("Could not start model download: {error}"));
        }
        return Err(format!("Could not start model download: {error}"));
    }
    Ok(downloads.status())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Instant;
    use vortex_core::tier::ProfileMode;

    fn state() -> Arc<Mutex<DaemonState>> {
        let config = AppConfig::default();
        let probe = HardwareProbe {
            ram_mb: 8192,
            cpu_cores: 4,
            arch: "x86_64".into(),
            on_battery: false,
            headless: true,
        };
        let policy = TierRuntimePolicy::compute(&probe, ProfileMode::Standard, false);
        Arc::new(Mutex::new(DaemonState {
            store: SqliteStore::open_in_memory(true, None).unwrap(),
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
            model_downloads: Downloads::default(),
        }))
    }

    fn fake_pull() -> (
        String,
        mpsc::Receiver<()>,
        mpsc::Sender<()>,
        std::thread::JoinHandle<()>,
    ) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let (arrived_tx, arrived_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let server = std::thread::spawn(move || {
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
            reader.read_exact(&mut vec![0; length]).unwrap();
            stream.write_all(b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n{\"status\":\"pulling layer\",\"completed\":25,\"total\":100}\n").unwrap();
            arrived_tx.send(()).unwrap();
            release_rx.recv_timeout(Duration::from_secs(3)).unwrap();
            let _ = stream.write_all(b"{\"status\":\"success\"}\n");
        });
        (endpoint, arrived_rx, release_tx, server)
    }

    fn wait_for(state: &Arc<Mutex<DaemonState>>, expected: &str) {
        let start = Instant::now();
        loop {
            if state.lock().unwrap().model_downloads.status()["status"] == expected {
                return;
            }
            assert!(
                start.elapsed() < Duration::from_secs(3),
                "Download did not reach {expected}"
            );
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn downloads_are_single_optional_jobs_with_progress_and_cancel() {
        let state = state();
        assert!(start_download(&state, "http://127.0.0.1:9", "unlisted:huge").is_err());
        assert_eq!(
            state.lock().unwrap().model_downloads.status()["status"],
            "idle"
        );
        let (endpoint, arrived, release, server) = fake_pull();
        let first = start_download(&state, &endpoint, MODELS[0].id).unwrap();
        arrived.recv_timeout(Duration::from_secs(3)).unwrap();
        {
            let guard = state
                .try_lock()
                .expect("Model download must release inbox mutex");
            assert!(!guard.config.ai.enabled);
            assert!(!guard.ai_worker_running);
            assert!(guard.store.get_setting("ai").unwrap().is_none());
        }
        assert!(start_download(&state, &endpoint, MODELS[1].id).is_err());
        release.send(()).unwrap();
        server.join().unwrap();
        wait_for(&state, "done");
        let status = state.lock().unwrap().model_downloads.status();
        assert_eq!(status["completed"], 25);
        assert_eq!(status["total"], 100);

        let (endpoint, arrived, release, server) = fake_pull();
        let second = start_download(&state, &endpoint, MODELS[1].id).unwrap();
        arrived.recv_timeout(Duration::from_secs(3)).unwrap();
        {
            let mut guard = state.lock().unwrap();
            assert!(
                guard
                    .model_downloads
                    .cancel(first["id"].as_u64().unwrap())
                    .is_err()
            );
            guard
                .model_downloads
                .cancel(second["id"].as_u64().unwrap())
                .unwrap();
        }
        release.send(()).unwrap();
        server.join().unwrap();
        wait_for(&state, "canceled");
        assert!(!state.lock().unwrap().config.ai.enabled);
    }

    #[test]
    fn catalog_rejects_remote_endpoints_and_download_selection_is_fixed() {
        assert!(catalog("http://example.com:11434", 8192).is_err());
        assert!(catalog("http://localhost.evil:11434", 8192).is_err());
        assert_eq!(MODELS.len(), 2);
        assert!(
            MODELS
                .iter()
                .all(|m| m.id.starts_with("qwen2.5:") && m.size_bytes < 1_000_000_000)
        );
    }

    #[test]
    fn stale_cancel_cannot_stop_a_different_download() {
        let cancel = Arc::new(AtomicBool::new(false));
        let mut downloads = Downloads {
            sequence: 2,
            current: Some(Download {
                id: 2,
                model: MODELS[0].id.into(),
                endpoint: "http://127.0.0.1:11434".into(),
                status: Phase::Downloading,
                completed: 3,
                total: 10,
                message: String::new(),
                error: None,
                cancel: Arc::clone(&cancel),
            }),
        };
        assert!(downloads.cancel(1).is_err());
        assert!(!cancel.load(Ordering::Relaxed));
        assert_eq!(downloads.cancel(2).unwrap()["status"], "canceling");
        assert!(cancel.load(Ordering::Relaxed));
        assert!(!downloads.status().to_string().contains("cancel\":"));
    }
}
