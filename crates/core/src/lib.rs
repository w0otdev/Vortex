pub mod config;
pub mod model;
pub mod secrets;
pub mod store;
pub mod tier;

pub const LITE_MAX_BINARY_BYTES: u64 = 6 * 1024 * 1024;
pub const LITE_MAX_IDLE_RSS_BYTES: u64 = 15 * 1024 * 1024;
pub const STANDARD_MAX_IDLE_RSS_BYTES: u64 = 25 * 1024 * 1024;
pub const FULL_MAX_IDLE_RSS_BYTES: u64 = 50 * 1024 * 1024;
pub const WEB_UI_MAX_ASSET_BYTES: u64 = 150 * 1024;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceTier {
    Lite,
    Standard,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TierBudget {
    pub tier: DeviceTier,
    pub max_binary_bytes: u64,
    pub max_idle_rss_bytes: u64,
    pub max_web_ui_bytes: u64,
}

impl DeviceTier {
    pub const fn budget(self) -> TierBudget {
        match self {
            Self::Lite => TierBudget {
                tier: self,
                max_binary_bytes: LITE_MAX_BINARY_BYTES,
                max_idle_rss_bytes: LITE_MAX_IDLE_RSS_BYTES,
                max_web_ui_bytes: 0,
            },
            Self::Standard => TierBudget {
                tier: self,
                max_binary_bytes: LITE_MAX_BINARY_BYTES,
                max_idle_rss_bytes: STANDARD_MAX_IDLE_RSS_BYTES,
                max_web_ui_bytes: WEB_UI_MAX_ASSET_BYTES,
            },
            Self::Full => TierBudget {
                tier: self,
                max_binary_bytes: LITE_MAX_BINARY_BYTES * 2,
                max_idle_rss_bytes: FULL_MAX_IDLE_RSS_BYTES,
                max_web_ui_bytes: WEB_UI_MAX_ASSET_BYTES,
            },
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "lite" => Some(Self::Lite),
            "standard" => Some(Self::Standard),
            "full" => Some(Self::Full),
            _ => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lite => "lite",
            Self::Standard => "standard",
            Self::Full => "full",
        }
    }
}

impl TierBudget {
    pub fn check(
        &self,
        binary_bytes: u64,
        rss_bytes: u64,
        web_ui_bytes: u64,
    ) -> Result<(), String> {
        if binary_bytes > self.max_binary_bytes {
            return Err(format!(
                "tier {} binary size {} B exceeds budget {} B",
                self.tier.as_str(),
                binary_bytes,
                self.max_binary_bytes
            ));
        }
        if rss_bytes > self.max_idle_rss_bytes {
            return Err(format!(
                "tier {} idle RSS {} B exceeds budget {} B",
                self.tier.as_str(),
                rss_bytes,
                self.max_idle_rss_bytes
            ));
        }
        if web_ui_bytes > self.max_web_ui_bytes {
            return Err(format!(
                "tier {} web UI size {} B exceeds budget {} B",
                self.tier.as_str(),
                web_ui_bytes,
                self.max_web_ui_bytes
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::model::{ExtractedCode, Item, ItemCategory, ItemFilter, QuickAction, SourceKind};
    use crate::secrets::SecretStore;
    use crate::store::SqliteStore;
    use crate::tier::{HardwareProbe, ProfileMode, TierRuntimePolicy};

    #[test]
    fn lite_budget_enforces_limits() {
        let budget = DeviceTier::Lite.budget();
        assert!(budget.check(2 * 1024 * 1024, 8 * 1024 * 1024, 0).is_ok());
        assert!(budget.check(7 * 1024 * 1024, 8 * 1024 * 1024, 0).is_err());
        assert!(budget.check(2 * 1024 * 1024, 16 * 1024 * 1024, 0).is_err());
        assert!(
            budget
                .check(2 * 1024 * 1024, 8 * 1024 * 1024, 1024)
                .is_err()
        );
    }

    #[test]
    fn tier_auto_detect_and_battery_scaling() {
        let rpi_probe = HardwareProbe {
            ram_mb: 512,
            cpu_cores: 1,
            arch: "armv7".into(),
            on_battery: false,
            headless: true,
        };
        let lite_policy = TierRuntimePolicy::compute(&rpi_probe, ProfileMode::Auto, true);
        assert_eq!(lite_policy.tier, DeviceTier::Lite);
        assert_eq!(lite_policy.worker_threads, 1);
        assert!(!lite_policy.web_ui_enabled);
        assert!(!lite_policy.fts_prefix_indexes);
        assert_eq!(lite_policy.keep_bodies_days, 7);
        assert!(!lite_policy.ai_allowed_now);

        let laptop_battery = HardwareProbe {
            ram_mb: 8192,
            cpu_cores: 4,
            arch: "x86_64".into(),
            on_battery: true,
            headless: false,
        };
        let std_policy = TierRuntimePolicy::compute(&laptop_battery, ProfileMode::Auto, true);
        assert_eq!(std_policy.tier, DeviceTier::Standard);
        assert_eq!(std_policy.min_poll_interval_secs, 60);
        assert!(!std_policy.ai_allowed_now);
        assert_eq!(std_policy.keep_bodies_days, 30);
    }

    #[test]
    fn config_rejects_raw_secrets() {
        let bad = r#"
            profile = "standard"
            password = "my_secret_password"
        "#;
        assert!(AppConfig::parse_and_validate(bad).is_err());

        let good = r#"
            profile = "standard"
            clipboard_clear_secs = 45
        "#;
        let parsed = AppConfig::parse_and_validate(good).unwrap();
        assert_eq!(parsed.profile_mode(), ProfileMode::Standard);
    }

    #[test]
    fn sqlite_store_fts_encryption_and_pruning() {
        let store = SqliteStore::open_in_memory(true, Some("test-master-key")).unwrap();
        let mut item = Item::new(
            SourceKind::Email,
            "work@example.com",
            "msg-001",
            "GitHub <noreply@github.com>",
            "Your verification code is 849201",
            "Use 849201 to complete sign-in.",
            1_700_000_000,
        );
        item.category = ItemCategory::CodeOtp;
        item.labels = vec!["otp".into(), "github".into()];
        item.extracted_codes.push(ExtractedCode {
            code: "849201".into(),
            service: "github.com".into(),
            expires_at: 1_700_000_600,
            confidence: 98,
            verification_link: None,
        });

        let id = store.upsert_item(&item).unwrap();
        let loaded = store.get_item(id).unwrap().unwrap();
        assert_eq!(loaded.sender_domain, "github.com");
        assert_eq!(loaded.extracted_codes[0].code, "849201");
        assert!(!format!("{:?}", loaded.extracted_codes[0]).contains("849201"));

        let fts_hits = store.search_fts("verification", 10).unwrap();
        assert_eq!(fts_hits.len(), 1);
        assert_eq!(fts_hits[0].id, id);

        store.apply_action(id, &QuickAction::Pin).unwrap();
        let pinned = store
            .list_items(&ItemFilter {
                pinned_only: true,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(pinned.len(), 1);

        let pruned = store
            .prune_expired_bodies(1_700_000_000 + 8 * 86_400, 7)
            .unwrap();
        assert_eq!(pruned, 1);
        let after_prune = store.get_item(id).unwrap().unwrap();
        assert!(after_prune.body_text.is_empty());
        assert_eq!(after_prune.subject, "Your verification code is 849201");
    }

    #[test]
    fn memory_secret_store_roundtrip() {
        let secrets = SecretStore::new_memory();
        secrets.set_secret("acc1", "tok_123").unwrap();
        assert_eq!(
            secrets.get_secret("acc1").unwrap().as_deref(),
            Some("tok_123")
        );
        secrets.delete_secret("acc1").unwrap();
        assert_eq!(secrets.get_secret("acc1").unwrap(), None);
    }
}
