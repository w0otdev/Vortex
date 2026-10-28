use crate::DeviceTier;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileMode {
    Auto,
    Lite,
    Standard,
    Full,
}

impl ProfileMode {
    pub fn from_str_name(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "lite" => Self::Lite,
            "standard" => Self::Standard,
            "full" => Self::Full,
            _ => Self::Auto,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Lite => "lite",
            Self::Standard => "standard",
            Self::Full => "full",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwareProbe {
    pub ram_mb: u64,
    pub cpu_cores: usize,
    pub arch: String,
    pub on_battery: bool,
    pub headless: bool,
}

impl HardwareProbe {
    pub fn detect() -> Self {
        let cpu_cores = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        let arch = env::consts::ARCH.to_string();
        let ram_mb = detect_ram_mb().unwrap_or(4096);
        let on_battery = detect_on_battery();
        let headless = detect_headless();
        Self {
            ram_mb,
            cpu_cores,
            arch,
            on_battery,
            headless,
        }
    }

    pub fn resolve_tier(&self, profile: ProfileMode) -> DeviceTier {
        match profile {
            ProfileMode::Lite => DeviceTier::Lite,
            ProfileMode::Standard => DeviceTier::Standard,
            ProfileMode::Full => DeviceTier::Full,
            ProfileMode::Auto => {
                let is_old_arm = self.arch == "arm" || self.arch.starts_with("armv7");
                if self.ram_mb <= 1024 || self.cpu_cores <= 1 || is_old_arm {
                    DeviceTier::Lite
                } else if self.ram_mb >= 16384 && self.cpu_cores >= 8 && !self.on_battery {
                    DeviceTier::Full
                } else {
                    DeviceTier::Standard
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TierRuntimePolicy {
    pub tier: DeviceTier,
    pub worker_threads: usize,
    pub web_ui_enabled: bool,
    pub tray_enabled: bool,
    pub fts_prefix_indexes: bool,
    pub keep_bodies_days: u32,
    pub min_poll_interval_secs: u64,
    pub idle_only: bool,
    pub ai_allowed_now: bool,
    pub semantic_search_enabled: bool,
    pub degraded_reasons: Vec<String>,
}

impl TierRuntimePolicy {
    pub fn compute(probe: &HardwareProbe, profile: ProfileMode, ai_configured: bool) -> Self {
        let tier = probe.resolve_tier(profile);
        let mut degraded_reasons = Vec::new();
        let min_poll_interval_secs = if probe.on_battery {
            degraded_reasons.push("on battery: poll interval doubled to >=60s, AI paused".into());
            60
        } else {
            30
        };

        match tier {
            DeviceTier::Lite => {
                degraded_reasons
                    .push("lite tier: single-thread runtime, web UI off, FTS prefix off".into());
                Self {
                    tier,
                    worker_threads: 1,
                    web_ui_enabled: false,
                    tray_enabled: false,
                    fts_prefix_indexes: false,
                    keep_bodies_days: 7,
                    min_poll_interval_secs,
                    idle_only: true,
                    ai_allowed_now: false,
                    semantic_search_enabled: false,
                    degraded_reasons,
                }
            }
            DeviceTier::Standard => Self {
                tier,
                worker_threads: probe.cpu_cores.clamp(2, 4),
                web_ui_enabled: !probe.headless,
                tray_enabled: !probe.headless,
                fts_prefix_indexes: true,
                keep_bodies_days: 30,
                min_poll_interval_secs,
                idle_only: false,
                ai_allowed_now: ai_configured && !probe.on_battery,
                semantic_search_enabled: false,
                degraded_reasons,
            },
            DeviceTier::Full => Self {
                tier,
                worker_threads: probe.cpu_cores.clamp(2, 8),
                web_ui_enabled: !probe.headless,
                tray_enabled: !probe.headless,
                fts_prefix_indexes: true,
                keep_bodies_days: 90,
                min_poll_interval_secs,
                idle_only: false,
                ai_allowed_now: ai_configured && !probe.on_battery,
                semantic_search_enabled: ai_configured && !probe.on_battery,
                degraded_reasons,
            },
        }
    }
}

fn detect_ram_mb() -> Option<u64> {
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if let Some(rest) = line.strip_prefix("MemTotal:") {
                let kb: u64 = rest.split_whitespace().next()?.parse().ok()?;
                return Some(kb / 1024);
            }
        }
    }
    if let Ok(override_mb) = env::var("VORTEX_RAM_MB")
        && let Ok(mb) = override_mb.parse::<u64>()
    {
        return Some(mb);
    }
    Some(8192)
}

fn detect_on_battery() -> bool {
    if let Ok(val) = env::var("VORTEX_ON_BATTERY") {
        return val == "1" || val.eq_ignore_ascii_case("true");
    }
    if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
        for entry in entries.flatten() {
            let status_path = entry.path().join("status");
            if let Ok(status) = fs::read_to_string(status_path)
                && status.trim().eq_ignore_ascii_case("Discharging")
            {
                return true;
            }
        }
    }
    false
}

fn detect_headless() -> bool {
    if let Ok(val) = env::var("VORTEX_HEADLESS") {
        return val == "1" || val.eq_ignore_ascii_case("true");
    }
    if cfg!(target_os = "linux") {
        let has_display =
            env::var_os("DISPLAY").is_some() || env::var_os("WAYLAND_DISPLAY").is_some();
        return !has_display;
    }
    false
}
