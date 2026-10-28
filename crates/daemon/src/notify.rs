use sha2::{Digest, Sha256};
use std::process::Command;
use vortex_core::model::{OtpPopupEvent, hex_encode};

pub struct ClipboardGuard {
    last_copied_hash: Option<String>,
    clear_at_epoch: Option<i64>,
    clear_delay_secs: u64,
}

impl ClipboardGuard {
    pub fn new(clear_delay_secs: u64) -> Self {
        Self {
            last_copied_hash: None,
            clear_at_epoch: None,
            clear_delay_secs: clear_delay_secs.clamp(30, 60),
        }
    }

    pub fn record_copy(&mut self, secret_code: &str, now_epoch: i64) {
        self.last_copied_hash = Some(hash_clipboard_value(secret_code));
        self.clear_at_epoch = Some(now_epoch + self.clear_delay_secs as i64);
    }

    pub fn copy_to_os_clipboard(
        &mut self,
        secret_code: &str,
        now_epoch: i64,
    ) -> Result<(), String> {
        set_os_clipboard(secret_code)?;
        self.record_copy(secret_code, now_epoch);
        Ok(())
    }

    /// Returns true if the clipboard was due to be cleared and still matched the copied OTP hash.
    pub fn should_clear_now(&mut self, current_clipboard: &str, now_epoch: i64) -> bool {
        let Some(deadline) = self.clear_at_epoch else {
            return false;
        };
        if now_epoch < deadline {
            return false;
        }
        let matches = self
            .last_copied_hash
            .as_deref()
            .is_some_and(|expected| expected == hash_clipboard_value(current_clipboard));
        self.last_copied_hash = None;
        self.clear_at_epoch = None;
        matches
    }
}

fn hash_clipboard_value(val: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"vortex-clipboard:");
    hasher.update(val.trim().as_bytes());
    hex_encode(&hasher.finalize())
}

pub fn set_os_clipboard(text: &str) -> Result<(), String> {
    if cfg!(target_os = "windows") {
        let status = Command::new("powershell")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                &format!("Set-Clipboard -Value '{}'", text.replace('\'', "''")),
            ])
            .status()
            .map_err(|e| format!("clipboard command failed: {e}"))?;
        if !status.success() {
            return Err("powershell Set-Clipboard exited non-zero".into());
        }
    } else if cfg!(target_os = "macos") {
        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
    } else if cfg!(target_os = "linux") {
        let _ = Command::new("wl-copy").arg(text).status();
    }
    Ok(())
}

pub fn emit_os_otp_notification(event: &OtpPopupEvent) -> Result<(), String> {
    let title = format!("Vortex OTP: {} ({})", event.code, event.service);
    let body = if let Some(ref warn) = event.phishing_warning {
        format!("Sender: {} | {warn}", event.sender)
    } else {
        format!("Sender: {}", event.sender)
    };

    if cfg!(target_os = "linux") {
        let _ = Command::new("notify-send").args([&title, &body]).status();
    } else if cfg!(target_os = "macos") {
        let script = format!(
            "display notification \"{}\" with title \"{}\"",
            body.replace('"', "\\\""),
            title.replace('"', "\\\"")
        );
        let _ = Command::new("osascript").args(["-e", &script]).status();
    }
    Ok(())
}
