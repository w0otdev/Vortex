use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use vortex_core::model::{constant_time_eq, hex_encode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseAssetInfo {
    pub version: String,
    pub download_url: String,
    pub sha256_hex: String,
}

fn sanitize_path_for_shell(p: &Path) -> String {
    p.display()
        .to_string()
        .chars()
        .filter(|c| !c.is_control() && *c != '"')
        .collect()
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn generate_systemd_user_unit(binary_path: &Path, config_path: &Path) -> String {
    let bin = sanitize_path_for_shell(binary_path).replace('\\', "\\\\");
    let cfg = sanitize_path_for_shell(config_path).replace('\\', "\\\\");
    format!(
        "[Unit]\nDescription=Vortex Inbox Background Agent\nAfter=network-online.target\n\n[Service]\nType=simple\nExecStart=\"{bin}\" daemon --config \"{cfg}\"\nRestart=on-failure\nRestartSec=10\n\n[Install]\nWantedBy=default.target\n"
    )
}

pub fn generate_launchd_plist(binary_path: &Path, config_path: &Path) -> String {
    let bin = escape_xml(&sanitize_path_for_shell(binary_path));
    let cfg = escape_xml(&sanitize_path_for_shell(config_path));
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n  <key>Label</key>\n  <string>org.vortexinbox.daemon</string>\n  <key>ProgramArguments</key>\n  <array>\n    <string>{bin}</string>\n    <string>daemon</string>\n    <string>--config</string>\n    <string>{cfg}</string>\n  </array>\n  <key>RunAtLoad</key>\n  <true/>\n  <key>KeepAlive</key>\n  <true/>\n</dict>\n</plist>\n"
    )
}

pub fn generate_windows_startup_script(binary_path: &Path, config_path: &Path) -> String {
    let bin = sanitize_path_for_shell(binary_path);
    let cfg = sanitize_path_for_shell(config_path);
    format!("@echo off\r\nstart \"\" /B \"{bin}\" daemon --config \"{cfg}\"\r\n")
}

pub fn install_user_service(
    target_dir: &Path,
    binary_path: &Path,
    config_path: &Path,
) -> Result<PathBuf, String> {
    fs::create_dir_all(target_dir).map_err(|e| format!("failed creating service dir: {e}"))?;
    if cfg!(target_os = "windows") {
        let path = target_dir.join("vortex-inbox-startup.cmd");
        let content = generate_windows_startup_script(binary_path, config_path);
        fs::write(&path, content).map_err(|e| format!("failed writing startup script: {e}"))?;
        Ok(path)
    } else if cfg!(target_os = "macos") {
        let path = target_dir.join("org.vortexinbox.daemon.plist");
        let content = generate_launchd_plist(binary_path, config_path);
        fs::write(&path, content).map_err(|e| format!("failed writing launchd plist: {e}"))?;
        Ok(path)
    } else {
        let path = target_dir.join("vortex-inbox.service");
        let content = generate_systemd_user_unit(binary_path, config_path);
        fs::write(&path, content).map_err(|e| format!("failed writing systemd unit: {e}"))?;
        Ok(path)
    }
}

pub fn verify_artifact_sha256(bytes: &[u8], expected_hex: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let actual = hex_encode(&hasher.finalize());
    let expected = expected_hex.trim().to_ascii_lowercase();
    constant_time_eq(actual.as_bytes(), expected.as_bytes())
}

pub fn is_newer_version(current: &str, candidate: &str) -> bool {
    let parse_ver = |s: &str| -> (u32, u32, u32) {
        let mut parts = s
            .trim()
            .trim_start_matches('v')
            .split('.')
            .map(|p| p.parse::<u32>().unwrap_or(0));
        (
            parts.next().unwrap_or(0),
            parts.next().unwrap_or(0),
            parts.next().unwrap_or(0),
        )
    };
    parse_ver(candidate) > parse_ver(current)
}
