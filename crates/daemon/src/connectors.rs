use crate::hub::{EncryptedEnvelope, decrypt_e2e_payload};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use vortex_core::config::AccountConfig;
use vortex_core::model::{Connector, Item, QuickAction, SourceKind};
use vortex_core::secrets::SecretStore;

pub struct RssConnector {
    config: AccountConfig,
}

impl RssConnector {
    pub fn new(config: AccountConfig) -> Self {
        Self { config }
    }

    pub fn parse_feed_xml(account_id: &str, xml: &str, now_epoch: i64) -> Vec<Item> {
        let mut items = Vec::new();
        let tag_type = if xml.contains("<entry") {
            "entry"
        } else {
            "item"
        };
        let open_tag = format!("<{tag_type}");
        let close_tag = format!("</{tag_type}>");

        let mut rest = xml;
        while let Some(start) = rest.find(&open_tag) {
            let after_open = &rest[start..];
            let Some(end) = after_open.find(&close_tag) else {
                break;
            };
            let block = &after_open[..end + close_tag.len()];
            let title = extract_xml_tag(block, "title").unwrap_or_default();
            let guid = extract_xml_tag(block, "guid")
                .or_else(|| extract_xml_tag(block, "id"))
                .or_else(|| extract_xml_tag(block, "link"))
                .unwrap_or_else(|| title.clone());
            let body = extract_xml_tag(block, "description")
                .or_else(|| extract_xml_tag(block, "summary"))
                .or_else(|| extract_xml_tag(block, "content"))
                .unwrap_or_default();
            let author = extract_xml_tag(block, "author")
                .or_else(|| extract_xml_tag(block, "dc:creator"))
                .unwrap_or_else(|| account_id.to_string());

            if !guid.is_empty() || !title.is_empty() {
                let item = Item::new(
                    SourceKind::Rss,
                    account_id,
                    guid,
                    author,
                    title,
                    body,
                    now_epoch,
                );
                items.push(item);
            }
            rest = &after_open[end + close_tag.len()..];
        }
        items
    }
}

impl Connector for RssConnector {
    fn id(&self) -> &str {
        "rss"
    }

    fn account(&self) -> &str {
        &self.config.id
    }

    fn connect(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn sync(&mut self, _since_epoch: Option<i64>) -> Result<Vec<Item>, String> {
        let url = self
            .config
            .endpoint_url
            .as_deref()
            .unwrap_or(&self.config.host);
        let xml = http_get_text(url, &[])?;
        Ok(Self::parse_feed_xml(&self.config.id, &xml, now_epoch()))
    }

    fn watch(&mut self, _timeout_ms: u64) -> Result<Vec<Item>, String> {
        self.sync(None)
    }

    fn execute_action(&mut self, _external_id: &str, _action: &QuickAction) -> Result<(), String> {
        Ok(())
    }
}

pub struct TelegramBotConnector {
    config: AccountConfig,
    secrets: SecretStore,
    last_update_id: i64,
}

impl TelegramBotConnector {
    pub fn new(config: AccountConfig, secrets: SecretStore) -> Self {
        Self {
            config,
            secrets,
            last_update_id: 0,
        }
    }

    pub fn with_last_update_id(mut self, last_update_id: i64) -> Self {
        self.last_update_id = last_update_id;
        self
    }

    pub fn parse_updates_json(
        &mut self,
        raw_json: &str,
        now_epoch: i64,
    ) -> Result<Vec<Item>, String> {
        let v: serde_json::Value =
            serde_json::from_str(raw_json).map_err(|e| format!("invalid telegram json: {e}"))?;
        let Some(arr) = v.get("result").and_then(|r| r.as_array()) else {
            return Ok(Vec::new());
        };
        let mut out = Vec::new();
        for upd in arr {
            let update_id = upd.get("update_id").and_then(|u| u.as_i64()).unwrap_or(0);
            if update_id <= self.last_update_id {
                continue;
            }
            self.last_update_id = update_id;
            if let Some(msg) = upd.get("message") {
                let text = msg
                    .get("text")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_string();
                let sender = msg
                    .pointer("/from/username")
                    .or_else(|| msg.pointer("/from/first_name"))
                    .and_then(|s| s.as_str())
                    .unwrap_or("telegram_user");
                let ts = msg
                    .get("date")
                    .and_then(|d| d.as_i64())
                    .unwrap_or(now_epoch);
                let subject = text
                    .lines()
                    .next()
                    .unwrap_or("Telegram message")
                    .to_string();
                out.push(Item::new(
                    SourceKind::Telegram,
                    &self.config.id,
                    update_id.to_string(),
                    format!("{sender}@telegram.org"),
                    subject,
                    text,
                    ts,
                ));
            }
        }
        Ok(out)
    }
}

impl Connector for TelegramBotConnector {
    fn id(&self) -> &str {
        "telegram"
    }

    fn account(&self) -> &str {
        &self.config.id
    }

    fn connect(&mut self) -> Result<(), String> {
        let _ = self
            .secrets
            .get_secret(&self.config.id)?
            .ok_or_else(|| "missing telegram bot token in keyring".to_string())?;
        Ok(())
    }

    fn sync(&mut self, _since_epoch: Option<i64>) -> Result<Vec<Item>, String> {
        let token = self
            .secrets
            .get_secret(&self.config.id)?
            .ok_or_else(|| "missing telegram bot token in keyring".to_string())?;
        let offset = self.last_update_id + 1;
        let url =
            format!("https://api.telegram.org/bot{token}/getUpdates?offset={offset}&timeout=1");
        let resp = http_get_text(&url, &[])?;
        self.parse_updates_json(&resp, now_epoch())
    }

    fn watch(&mut self, _timeout_ms: u64) -> Result<Vec<Item>, String> {
        self.sync(None)
    }

    fn execute_action(&mut self, _external_id: &str, _action: &QuickAction) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidSmsRelayPayload {
    pub message_id: String,
    pub sender: String,
    pub body: String,
    pub received_at: i64,
}

pub fn ingest_android_e2e_relay(
    account_id: &str,
    shared_secret: &str,
    envelope: &EncryptedEnvelope,
) -> Result<Item, String> {
    let plain = decrypt_e2e_payload(shared_secret, envelope)?;
    let sms: AndroidSmsRelayPayload =
        serde_json::from_str(&plain).map_err(|e| format!("invalid android relay JSON: {e}"))?;
    let subject = sms.body.lines().next().unwrap_or("SMS").to_string();
    Ok(Item::new(
        SourceKind::AndroidSms,
        account_id,
        sms.message_id,
        sms.sender,
        subject,
        sms.body,
        sms.received_at,
    ))
}

pub fn parse_windows_notification_preview(
    account_id: &str,
    app_name: &str,
    title: &str,
    text: &str,
    now_epoch: i64,
) -> Item {
    let ext_id = format!("win-notif-{app_name}-{now_epoch}");
    Item::new(
        SourceKind::WinNotification,
        account_id,
        ext_id,
        app_name,
        title,
        text,
        now_epoch,
    )
}

pub fn parse_chat_bot_message(
    source: SourceKind,
    account_id: &str,
    message_id: &str,
    sender: &str,
    channel_name: &str,
    text: &str,
    received_at: i64,
) -> Item {
    Item::new(
        source,
        account_id,
        message_id,
        sender,
        format!("[{channel_name}] {}", text.lines().next().unwrap_or("")),
        text,
        received_at,
    )
}

fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let start_idx = xml.find(&open)?;
    let after_open = &xml[start_idx + open.len()..];
    let gt_idx = after_open.find('>')?;
    let content_start = &after_open[gt_idx + 1..];
    let end_idx = content_start.find(&close)?;
    let raw = content_start[..end_idx]
        .trim()
        .trim_start_matches("<![CDATA[")
        .trim_end_matches("]]>")
        .trim();
    Some(raw.to_string())
}

pub fn http_get_text(url: &str, headers: &[(String, String)]) -> Result<String, String> {
    if url.contains('\r') || url.contains('\n') {
        return Err("URL contains forbidden control characters".into());
    }
    let (use_tls, rest) = if let Some(r) = url.strip_prefix("https://") {
        (true, r)
    } else if let Some(r) = url.strip_prefix("http://") {
        (false, r)
    } else {
        return Err(format!("unsupported URL scheme: {url}"));
    };

    let (authority, path) = match rest.split_once('/') {
        Some((hp, p)) => (hp, format!("/{p}")),
        None => (rest, "/".to_string()),
    };
    let host_port = authority.rsplit_once('@').map_or(authority, |(_, hp)| hp);
    let (host, port) = match host_port.split_once(':') {
        Some((h, p)) => (
            h,
            p.parse::<u16>().unwrap_or(if use_tls { 443 } else { 80 }),
        ),
        None => (host_port, if use_tls { 443 } else { 80 }),
    };
    let host_lower = host.trim_matches(['[', ']']).to_ascii_lowercase();
    if host_lower.is_empty()
        || host_lower == "localhost"
        || host_lower == "0.0.0.0"
        || host_lower == "::1"
        || host_lower.starts_with("127.")
        || host_lower.starts_with("169.254.")
    {
        return Err(format!("disallowed loopback or link-local host: {host}"));
    }

    let tcp = TcpStream::connect((host, port))
        .map_err(|e| format!("connect {host}:{port} failed: {e}"))?;
    let _ = tcp.set_read_timeout(Some(Duration::from_secs(15)));
    let _ = tcp.set_write_timeout(Some(Duration::from_secs(10)));

    let mut req = format!(
        "GET {path} HTTP/1.0\r\nHost: {host_port}\r\nUser-Agent: VortexInbox/0.1\r\nConnection: close\r\n"
    );
    for (k, v) in headers {
        if k.contains('\r') || k.contains('\n') || v.contains('\r') || v.contains('\n') {
            return Err("HTTP header contains forbidden control characters".into());
        }
        req.push_str(&format!("{k}: {v}\r\n"));
    }
    req.push_str("\r\n");

    let mut raw = String::new();
    if use_tls {
        let tls_conn = native_tls::TlsConnector::new().map_err(|e| e.to_string())?;
        let mut tls = tls_conn.connect(host, tcp).map_err(|e| e.to_string())?;
        tls.write_all(req.as_bytes()).map_err(|e| e.to_string())?;
        tls.take(2 * 1024 * 1024)
            .read_to_string(&mut raw)
            .map_err(|e| e.to_string())?;
    } else {
        let mut s = tcp;
        s.write_all(req.as_bytes()).map_err(|e| e.to_string())?;
        s.take(2 * 1024 * 1024)
            .read_to_string(&mut raw)
            .map_err(|e| e.to_string())?;
    }

    let (headers_part, body) = raw
        .split_once("\r\n\r\n")
        .ok_or_else(|| "invalid HTTP response".to_string())?;
    let status_line = headers_part.lines().next().unwrap_or("");
    if !status_line.starts_with("HTTP/1.1 2") && !status_line.starts_with("HTTP/1.0 2") {
        return Err(format!("HTTP error response: {status_line}"));
    }
    Ok(body.to_string())
}

fn now_epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
