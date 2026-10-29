use crate::model::{ItemCategory, SourceKind};
use crate::tier::ProfileMode;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    #[serde(default = "default_profile")]
    pub profile: String,
    #[serde(default = "default_db_path")]
    pub database_path: String,
    #[serde(default)]
    pub encrypt_database: bool,
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,
    #[serde(default)]
    pub clipboard_auto_copy: bool,
    #[serde(default = "default_clipboard_clear_secs")]
    pub clipboard_clear_secs: u64,
    #[serde(default = "default_post_expiry_action")]
    pub post_expiry_action: String,
    #[serde(default)]
    pub accounts: Vec<AccountConfig>,
    #[serde(default)]
    pub rules: Vec<UserRuleConfig>,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub hub: HubConfig,
}

fn default_profile() -> String {
    "auto".into()
}
fn default_db_path() -> String {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA")
        && !local_app_data.trim().is_empty()
    {
        let dir = Path::new(&local_app_data).join("VortexInbox");
        let _ = fs::create_dir_all(&dir);
        return dir.join("vortex.db").to_string_lossy().into_owned();
    }
    "vortex.db".into()
}
fn default_bind_addr() -> String {
    "127.0.0.1:7788".into()
}
fn default_clipboard_clear_secs() -> u64 {
    45
}
fn default_post_expiry_action() -> String {
    "mark_read".into()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            profile: default_profile(),
            database_path: default_db_path(),
            encrypt_database: false,
            bind_addr: default_bind_addr(),
            clipboard_auto_copy: false,
            clipboard_clear_secs: default_clipboard_clear_secs(),
            post_expiry_action: default_post_expiry_action(),
            accounts: Vec::new(),
            rules: Vec::new(),
            ai: AiConfig::default(),
            hub: HubConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountConfig {
    pub id: String,
    pub source: SourceKind,
    pub email_or_handle: String,
    pub host: String,
    pub port: u16,
    #[serde(default = "default_true")]
    pub use_tls: bool,
    #[serde(default = "default_auth_method")]
    pub auth_method: String,
    #[serde(default)]
    pub oauth_client_id: Option<String>,
    #[serde(default)]
    pub gmail_writeback_labels: bool,
    #[serde(default)]
    pub ai_enabled: bool,
    #[serde(default)]
    pub endpoint_url: Option<String>,
}

fn default_true() -> bool {
    true
}
fn default_auth_method() -> String {
    "password".into()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserRuleConfig {
    pub name: String,
    pub match_sender: Option<String>,
    pub match_keyword: Option<String>,
    pub match_category: Option<ItemCategory>,
    pub apply_label: Option<String>,
    #[serde(default)]
    pub pin: bool,
    pub snooze_minutes: Option<u32>,
    pub auto_archive_days: Option<u32>,
    #[serde(default)]
    pub explicit_delete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub auto_hide: bool,
    #[serde(default = "default_ai_provider")]
    pub provider: String,
    #[serde(default = "default_ai_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_small_model")]
    pub small_model: String,
    #[serde(default = "default_large_model")]
    pub large_model: String,
    #[serde(default)]
    pub allow_sensitive_external: bool,
}

fn default_ai_provider() -> String {
    "ollama".into()
}
fn default_ai_endpoint() -> String {
    "http://127.0.0.1:11434".into()
}
fn default_small_model() -> String {
    "qwen2.5:1.5b".into()
}
fn default_large_model() -> String {
    "qwen2.5:7b".into()
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            auto_hide: false,
            provider: default_ai_provider(),
            endpoint: default_ai_endpoint(),
            small_model: default_small_model(),
            large_model: default_large_model(),
            allow_sensitive_external: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HubConfig {
    #[serde(default = "default_hub_mode")]
    pub mode: String,
    #[serde(default = "default_hub_addr")]
    pub listen_addr: String,
    #[serde(default)]
    pub hub_url: Option<String>,
}

fn default_hub_mode() -> String {
    "off".into()
}
fn default_hub_addr() -> String {
    "127.0.0.1:7789".into()
}

impl Default for HubConfig {
    fn default() -> Self {
        Self {
            mode: default_hub_mode(),
            listen_addr: default_hub_addr(),
            hub_url: None,
        }
    }
}

impl AppConfig {
    pub fn profile_mode(&self) -> ProfileMode {
        ProfileMode::from_str_name(&self.profile)
    }

    pub fn load_or_default(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(path).map_err(|e| format!("failed reading config: {e}"))?;
        Self::parse_and_validate(&raw)
    }

    pub fn parse_and_validate(raw_toml: &str) -> Result<Self, String> {
        // Security gate: reject config files that attempt to embed raw secrets or passwords.
        for line in raw_toml.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('[') {
                continue;
            }
            if let Some((raw_key, _)) = trimmed.split_once('=') {
                let key = raw_key
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_ascii_lowercase();
                if matches!(
                    key.as_str(),
                    "password"
                        | "api_key"
                        | "apikey"
                        | "secret"
                        | "token"
                        | "bot_token"
                        | "client_secret"
                        | "refresh_token"
                        | "access_token"
                ) {
                    return Err(
                        "secrets are forbidden in config files; store credentials in the OS keyring"
                            .into(),
                    );
                }
            }
        }
        let mut cfg: Self =
            toml::from_str(raw_toml).map_err(|e| format!("invalid TOML config: {e}"))?;
        cfg.clipboard_clear_secs = cfg.clipboard_clear_secs.clamp(30, 60);
        Ok(cfg)
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let serialized =
            toml::to_string_pretty(self).map_err(|e| format!("failed encoding config: {e}"))?;
        fs::write(path, serialized).map_err(|e| format!("failed writing config: {e}"))
    }
}
