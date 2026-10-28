use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Email,
    Telegram,
    Rss,
    WinNotification,
    AndroidSms,
    Slack,
    Matrix,
    Discord,
}

impl SourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Telegram => "telegram",
            Self::Rss => "rss",
            Self::WinNotification => "win_notification",
            Self::AndroidSms => "android_sms",
            Self::Slack => "slack",
            Self::Matrix => "matrix",
            Self::Discord => "discord",
        }
    }

    pub fn from_str_name(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "telegram" => Self::Telegram,
            "rss" => Self::Rss,
            "win_notification" => Self::WinNotification,
            "android_sms" => Self::AndroidSms,
            "slack" => Self::Slack,
            "matrix" => Self::Matrix,
            "discord" => Self::Discord,
            _ => Self::Email,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemState {
    Unread,
    Read,
    Archived,
    Snoozed,
    Trash,
    Spam,
}

impl ItemState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unread => "unread",
            Self::Read => "read",
            Self::Archived => "archived",
            Self::Snoozed => "snoozed",
            Self::Trash => "trash",
            Self::Spam => "spam",
        }
    }

    pub fn from_str_name(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "read" => Self::Read,
            "archived" => Self::Archived,
            "snoozed" => Self::Snoozed,
            "trash" => Self::Trash,
            "spam" => Self::Spam,
            _ => Self::Unread,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemCategory {
    CodeOtp,
    Billing,
    Shipping,
    SecurityAlert,
    Social,
    Work,
    PromoNewsletter,
    Other,
}

impl ItemCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeOtp => "code_otp",
            Self::Billing => "billing",
            Self::Shipping => "shipping",
            Self::SecurityAlert => "security_alert",
            Self::Social => "social",
            Self::Work => "work",
            Self::PromoNewsletter => "promo_newsletter",
            Self::Other => "other",
        }
    }

    pub fn from_str_name(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "code_otp" => Self::CodeOtp,
            "billing" => Self::Billing,
            "shipping" => Self::Shipping,
            "security_alert" => Self::SecurityAlert,
            "social" => Self::Social,
            "work" => Self::Work,
            "promo_newsletter" => Self::PromoNewsletter,
            _ => Self::Other,
        }
    }

    pub const fn is_sensitive(self) -> bool {
        matches!(self, Self::CodeOtp | Self::SecurityAlert | Self::Billing)
    }

    pub const fn gmail_sublabel(self) -> &'static str {
        match self {
            Self::CodeOtp => "Vortex/Code",
            Self::Billing => "Vortex/Billing",
            Self::Shipping => "Vortex/Shipping",
            Self::SecurityAlert => "Vortex/Security",
            Self::Social => "Vortex/Social",
            Self::Work => "Vortex/Work",
            Self::PromoNewsletter => "Vortex/Promotions",
            Self::Other => "Vortex/Other",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedCode {
    pub code: String,
    pub service: String,
    pub expires_at: i64,
    pub confidence: u8,
    pub verification_link: Option<String>,
}

// Never write OTP codes into debug logs.
impl fmt::Debug for ExtractedCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtractedCode")
            .field("code", &"[REDACTED_OTP]")
            .field("service", &self.service)
            .field("expires_at", &self.expires_at)
            .field("confidence", &self.confidence)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedLink {
    pub url: String,
    pub domain: String,
    pub is_verification: bool,
    pub is_unsubscribe: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedEntity {
    pub kind: String,
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub source: SourceKind,
    pub account: String,
    pub external_id: String,
    pub sender: String,
    pub sender_domain: String,
    pub recipient: String,
    pub subject: String,
    pub body_text: String,
    pub body_html: String,
    pub received_at: i64,
    pub state: ItemState,
    pub snoozed_until: Option<i64>,
    pub pinned: bool,
    pub labels: Vec<String>,
    pub category: ItemCategory,
    pub extracted_codes: Vec<ExtractedCode>,
    pub extracted_links: Vec<ExtractedLink>,
    pub extracted_entities: Vec<ExtractedEntity>,
    pub phishing_score: u8,
    pub phishing_warning: Option<String>,
    pub unsubscribe_link: Option<String>,
    pub summary: Option<String>,
    pub priority_score: u8,
    pub gmail_labels: Vec<String>,
    pub list_unsubscribe_header: Option<String>,
    pub precedence_header: Option<String>,
}

impl Item {
    pub fn new(
        source: SourceKind,
        account: impl Into<String>,
        external_id: impl Into<String>,
        sender: impl Into<String>,
        subject: impl Into<String>,
        body_text: impl Into<String>,
        received_at: i64,
    ) -> Self {
        let sender_str = sender.into();
        let sender_domain = extract_email_domain(&sender_str);
        Self {
            id: 0,
            source,
            account: account.into(),
            external_id: external_id.into(),
            sender: sender_str,
            sender_domain,
            recipient: String::new(),
            subject: subject.into(),
            body_text: body_text.into(),
            body_html: String::new(),
            received_at,
            state: ItemState::Unread,
            snoozed_until: None,
            pinned: false,
            labels: Vec::new(),
            category: ItemCategory::Other,
            extracted_codes: Vec::new(),
            extracted_links: Vec::new(),
            extracted_entities: Vec::new(),
            phishing_score: 0,
            phishing_warning: None,
            unsubscribe_link: None,
            summary: None,
            priority_score: 50,
            gmail_labels: Vec::new(),
            list_unsubscribe_header: None,
            precedence_header: None,
        }
    }

    pub fn content_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.sender.as_bytes());
        hasher.update(b"\n");
        hasher.update(self.subject.as_bytes());
        hasher.update(b"\n");
        hasher.update(self.body_text.as_bytes());
        hex_encode(&hasher.finalize())
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtpPopupEvent {
    pub item_id: i64,
    pub message_id: String,
    pub code_hash: String,
    pub code: String,
    pub sender: String,
    #[serde(default)]
    pub account: String,
    #[serde(default)]
    pub recipient: String,
    pub service: String,
    pub received_at: i64,
    pub expires_at: i64,
    pub verification_link: Option<String>,
    pub phishing_warning: Option<String>,
    pub detected_latency_ms: u64,
}

impl fmt::Debug for OtpPopupEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OtpPopupEvent")
            .field("item_id", &self.item_id)
            .field("message_id", &self.message_id)
            .field("code_hash", &self.code_hash)
            .field("code", &"[REDACTED_OTP]")
            .field("service", &self.service)
            .field("expires_at", &self.expires_at)
            .field("detected_latency_ms", &self.detected_latency_ms)
            .finish()
    }
}

impl OtpPopupEvent {
    pub fn compute_dedup_hash(message_id: &str, code: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(message_id.trim().as_bytes());
        hasher.update(b":");
        hasher.update(code.trim().as_bytes());
        hex_encode(&hasher.finalize())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum QuickAction {
    Archive,
    Delete,
    MarkRead,
    MarkUnread,
    Pin,
    Unpin,
    Snooze { until_epoch: i64 },
    ReportSpam,
    Unsubscribe,
    OpenVerificationLink,
    FlagNotACode { pattern: String },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemFilter {
    pub source: Option<SourceKind>,
    pub account: Option<String>,
    pub category: Option<ItemCategory>,
    pub state: Option<ItemState>,
    pub unread_only: bool,
    pub pinned_only: bool,
    pub limit: usize,
}

pub trait Connector: Send {
    fn id(&self) -> &str;
    fn account(&self) -> &str;
    fn connect(&mut self) -> Result<(), String>;
    fn sync(&mut self, since_epoch: Option<i64>) -> Result<Vec<Item>, String>;
    fn watch(&mut self, timeout_ms: u64) -> Result<Vec<Item>, String>;
    fn execute_action(&mut self, external_id: &str, action: &QuickAction) -> Result<(), String>;
}

pub fn extract_email_domain(sender: &str) -> String {
    let trimmed = sender.trim();
    let addr = if let Some((_, after_lt)) = trimmed.rsplit_once('<') {
        after_lt.trim_end_matches('>').trim()
    } else {
        trimmed
    };
    addr.rsplit_once('@')
        .map(|(_, d)| {
            d.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '-')
                .to_ascii_lowercase()
        })
        .unwrap_or_default()
}

pub fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(&mut out, "{b:02x}");
    }
    out
}
