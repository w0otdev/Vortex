use crate::config::{AccountConfig, UserRuleConfig};
use crate::model::{
    ExtractedCode, ExtractedEntity, ExtractedLink, Item, ItemCategory, ItemFilter, ItemState,
    OtpPopupEvent, QuickAction, SourceKind, constant_time_eq, generate_random_bytes_32,
};
use base64::Engine;
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

const BACKPRESSURE_WRITE_THRESHOLD_MS: u64 = 150;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCallLogEntry {
    pub id: i64,
    pub created_at: i64,
    pub provider: String,
    pub model: String,
    pub task: String,
    pub estimated_tokens: usize,
    pub payload_preview: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairedDevice {
    pub device_id: String,
    pub device_name: String,
    pub shared_secret_hash: String,
    pub paired_at: i64,
    pub revoked: bool,
}

pub struct SqliteStore {
    conn: Connection,
    encryption_key: Option<[u8; 32]>,
    last_write_ms: AtomicU64,
}

impl SqliteStore {
    pub fn open(
        path: &Path,
        fts_prefix_indexes: bool,
        encryption_secret: Option<&str>,
    ) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|e| format!("sqlite open failed: {e}"))?;
        Self::init(conn, fts_prefix_indexes, encryption_secret)
    }

    pub fn open_in_memory(
        fts_prefix_indexes: bool,
        encryption_secret: Option<&str>,
    ) -> Result<Self, String> {
        let conn =
            Connection::open_in_memory().map_err(|e| format!("sqlite memory open failed: {e}"))?;
        Self::init(conn, fts_prefix_indexes, encryption_secret)
    }

    fn init(
        conn: Connection,
        fts_prefix_indexes: bool,
        encryption_secret: Option<&str>,
    ) -> Result<Self, String> {
        let encryption_key = encryption_secret.filter(|s| !s.is_empty()).map(|secret| {
            let mut key = [0u8; 32];
            let mut hasher = Sha256::new();
            hasher.update(b"vortex-local-encryption-v1:");
            hasher.update(secret.as_bytes());
            key.copy_from_slice(&hasher.finalize());
            for round in 0..4096u32 {
                let mut h = Sha256::new();
                h.update(b"vortex-kdf-v2:");
                h.update(round.to_le_bytes());
                h.update(key);
                h.update(secret.as_bytes());
                key.copy_from_slice(&h.finalize());
            }
            key
        });

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;
             PRAGMA busy_timeout = 5000;",
        )
        .map_err(|e| format!("sqlite pragma setup failed: {e}"))?;

        let fts_sql = if fts_prefix_indexes {
            "CREATE VIRTUAL TABLE IF NOT EXISTS items_fts USING fts5(
                sender,
                subject,
                body_text,
                labels,
                prefix='2 3'
            );"
        } else {
            "CREATE VIRTUAL TABLE IF NOT EXISTS items_fts USING fts5(
                sender,
                subject,
                body_text,
                labels
            );"
        };

        conn.execute_batch(&format!(
            "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source TEXT NOT NULL,
                account TEXT NOT NULL,
                external_id TEXT NOT NULL,
                sender TEXT NOT NULL,
                sender_domain TEXT NOT NULL,
                recipient TEXT NOT NULL,
                subject TEXT NOT NULL,
                body_text TEXT NOT NULL,
                body_html TEXT NOT NULL,
                received_at INTEGER NOT NULL,
                state TEXT NOT NULL,
                snoozed_until INTEGER,
                pinned INTEGER NOT NULL DEFAULT 0,
                labels_json TEXT NOT NULL,
                category TEXT NOT NULL,
                codes_json TEXT NOT NULL,
                links_json TEXT NOT NULL,
                entities_json TEXT NOT NULL,
                phishing_score INTEGER NOT NULL DEFAULT 0,
                phishing_warning TEXT,
                unsubscribe_link TEXT,
                summary TEXT,
                priority_score INTEGER NOT NULL DEFAULT 50,
                gmail_labels_json TEXT NOT NULL DEFAULT '[]',
                list_unsubscribe_header TEXT,
                precedence_header TEXT,
                UNIQUE(source, account, external_id)
            );
            CREATE INDEX IF NOT EXISTS idx_items_received ON items(received_at DESC);
            CREATE INDEX IF NOT EXISTS idx_items_category ON items(category, received_at DESC);
            CREATE INDEX IF NOT EXISTS idx_items_state ON items(state, received_at DESC);
            {fts_sql}
            CREATE TABLE IF NOT EXISTS recent_codes (
                code_hash TEXT PRIMARY KEY,
                item_id INTEGER NOT NULL,
                message_id TEXT NOT NULL,
                code_enc TEXT NOT NULL,
                sender TEXT NOT NULL,
                service TEXT NOT NULL,
                received_at INTEGER NOT NULL,
                expires_at INTEGER NOT NULL,
                verification_link TEXT,
                phishing_warning TEXT,
                detected_latency_ms INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS not_a_code_rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sender_domain TEXT NOT NULL,
                pattern TEXT NOT NULL,
                UNIQUE(sender_domain, pattern)
            );
            CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                config_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS user_rules (
                name TEXT PRIMARY KEY,
                rule_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS ai_cache (
                content_hash TEXT NOT NULL,
                task TEXT NOT NULL,
                response_json TEXT NOT NULL,
                updated_at INTEGER NOT NULL,
                PRIMARY KEY(content_hash, task)
            );
            CREATE TABLE IF NOT EXISTS ai_call_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                created_at INTEGER NOT NULL,
                provider TEXT NOT NULL,
                model TEXT NOT NULL,
                task TEXT NOT NULL,
                estimated_tokens INTEGER NOT NULL,
                payload_preview TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_ai_cache_updated ON ai_cache(updated_at);
            CREATE INDEX IF NOT EXISTS idx_ai_call_log_created ON ai_call_log(created_at);
            CREATE TABLE IF NOT EXISTS ai_jobs (
                item_id INTEGER PRIMARY KEY REFERENCES items(id) ON DELETE CASCADE,
                cache_key TEXT NOT NULL,
                status TEXT NOT NULL,
                error TEXT,
                manual INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS app_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS item_embeddings (
                item_id INTEGER PRIMARY KEY,
                vector_blob BLOB NOT NULL,
                FOREIGN KEY(item_id) REFERENCES items(id) ON DELETE CASCADE
            );
            CREATE TABLE IF NOT EXISTS paired_devices (
                device_id TEXT PRIMARY KEY,
                device_name TEXT NOT NULL,
                shared_secret_hash TEXT NOT NULL,
                paired_at INTEGER NOT NULL,
                revoked INTEGER NOT NULL DEFAULT 0
            );
            DELETE FROM items
            WHERE (external_id LIKE ':%' OR external_id LIKE 'gmail-row-%' OR external_id LIKE 'gmail-msg-%' OR external_id LIKE 'tag:gmail.google.com%' OR external_id LIKE 'atom-%')
              AND id NOT IN (
                  SELECT id FROM (
                      SELECT id, ROW_NUMBER() OVER (
                          PARTITION BY source, account, sender, subject
                          ORDER BY CASE WHEN external_id LIKE ':%' OR external_id LIKE 'gmail-row-%' OR external_id LIKE 'gmail-msg-%' OR external_id LIKE 'tag:gmail.google.com%' OR external_id LIKE 'atom-%' THEN 1 ELSE 0 END ASC,
                                   LENGTH(body_html) DESC,
                                   LENGTH(body_text) DESC,
                                   id ASC
                      ) AS rn
                      FROM items
                  ) WHERE rn = 1
              );"
        ))
        .map_err(|e| format!("sqlite schema migration failed: {e}"))?;

        for (table, name, definition) in [
            ("items", "ai_hidden", "TEXT"),
            ("items", "ai_keep", "INTEGER NOT NULL DEFAULT 0"),
            ("ai_jobs", "manual", "INTEGER NOT NULL DEFAULT 0"),
        ] {
            let exists: bool = conn
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM pragma_table_info(?1) WHERE name = ?2)",
                    params![table, name],
                    |row| row.get(0),
                )
                .map_err(|e| e.to_string())?;
            if !exists {
                conn.execute_batch(&format!(
                    "ALTER TABLE {table} ADD COLUMN {name} {definition}"
                ))
                .map_err(|e| format!("AI metadata migration failed: {e}"))?;
            }
        }
        conn.execute_batch("DROP INDEX IF EXISTS idx_ai_jobs_status;
            CREATE INDEX IF NOT EXISTS idx_ai_jobs_priority ON ai_jobs(status, manual DESC, item_id DESC);")
            .map_err(|e| format!("AI queue index migration failed: {e}"))?;

        Ok(Self {
            conn,
            encryption_key,
            last_write_ms: AtomicU64::new(0),
        })
    }

    pub fn last_write_latency_ms(&self) -> u64 {
        self.last_write_ms.load(Ordering::Relaxed)
    }

    pub fn is_disk_backpressured(&self) -> bool {
        self.last_write_latency_ms() > BACKPRESSURE_WRITE_THRESHOLD_MS
    }

    fn protect_field(&self, plain: &str, _context: &str) -> String {
        let Some(key) = &self.encryption_key else {
            return plain.to_string();
        };
        if plain.is_empty() {
            return String::new();
        }
        let rand_bytes = generate_random_bytes_32(b"vortex-store-enc2-nonce", plain.len() as u64);
        let nonce = &rand_bytes[..16];
        let cipher = xor_stream_cipher(key, nonce, plain.as_bytes());
        let mac = compute_field_mac(key, nonce, &cipher);
        let mut payload = Vec::with_capacity(32 + cipher.len());
        payload.extend_from_slice(nonce);
        payload.extend_from_slice(&mac);
        payload.extend_from_slice(&cipher);
        format!(
            "enc2:{}",
            base64::engine::general_purpose::STANDARD.encode(payload)
        )
    }

    fn unprotect_field(&self, stored: &str, context: &str) -> String {
        if let Some(rest) = stored.strip_prefix("enc2:") {
            let Some(key) = &self.encryption_key else {
                return String::new();
            };
            let Ok(payload) = base64::engine::general_purpose::STANDARD.decode(rest) else {
                return String::new();
            };
            if payload.len() < 32 {
                return String::new();
            }
            let nonce = &payload[..16];
            let mac = &payload[16..32];
            let cipher = &payload[32..];
            let expected_mac = compute_field_mac(key, nonce, cipher);
            if !constant_time_eq(&expected_mac, mac) {
                return String::new();
            }
            let plain = xor_stream_cipher(key, nonce, cipher);
            return String::from_utf8(plain).unwrap_or_default();
        }
        let Some(rest) = stored.strip_prefix("enc1:") else {
            return stored.to_string();
        };
        let Some(key) = &self.encryption_key else {
            return String::new();
        };
        let Ok(cipher) = base64::engine::general_purpose::STANDARD.decode(rest) else {
            return String::new();
        };
        let plain = xor_stream_cipher(key, context.as_bytes(), &cipher);
        String::from_utf8(plain).unwrap_or_default()
    }

    pub fn get_existing_received_at(
        &self,
        source: SourceKind,
        account: &str,
        external_id: &str,
        subject: &str,
    ) -> Option<i64> {
        self.conn
            .query_row(
                "SELECT received_at FROM items
                 WHERE source = ?1
                   AND account = ?2
                   AND (external_id = ?3 OR (subject = ?4 AND external_id LIKE 'tag:gmail.google.com%'))
                 ORDER BY received_at DESC
                 LIMIT 1",
                params![source.as_str(), account, external_id, subject],
                |row| row.get(0),
            )
            .optional()
            .unwrap_or(None)
    }

    pub fn clear_recent_codes(&self) -> Result<(), String> {
        self.conn
            .execute(
                "UPDATE recent_codes SET expires_at = 0 WHERE expires_at > 0",
                [],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn upsert_item(&self, item: &Item) -> Result<i64, String> {
        let start = Instant::now();
        let labels_json = serde_json::to_string(&item.labels).unwrap_or_else(|_| "[]".into());
        let codes_raw =
            serde_json::to_string(&item.extracted_codes).unwrap_or_else(|_| "[]".into());
        let links_json =
            serde_json::to_string(&item.extracted_links).unwrap_or_else(|_| "[]".into());
        let entities_json =
            serde_json::to_string(&item.extracted_entities).unwrap_or_else(|_| "[]".into());
        let gmail_labels_json =
            serde_json::to_string(&item.gmail_labels).unwrap_or_else(|_| "[]".into());

        let tx = self
            .conn
            .unchecked_transaction()
            .map_err(|e| format!("begin transaction failed: {e}"))?;

        if !item.external_id.starts_with("tag:gmail.google.com")
            && !item.external_id.starts_with("atom-")
            && !item.external_id.starts_with("gmail-row-")
            && !item.external_id.starts_with("gmail-msg-")
            && !item.external_id.starts_with(':')
        {
            let exact_exists: Option<i64> = tx
                .query_row(
                    "SELECT id FROM items WHERE source = ?1 AND account = ?2 AND external_id = ?3",
                    params![item.source.as_str(), item.account, item.external_id],
                    |row| row.get(0),
                )
                .optional()
                .unwrap_or(None);
            if exact_exists.is_none() {
                let _ = tx.execute(
                    "UPDATE items
                     SET external_id = ?4
                     WHERE id = (
                         SELECT id FROM items
                         WHERE source = ?1
                           AND account = ?2
                           AND subject = ?3
                           AND (external_id LIKE 'tag:gmail.google.com%' OR external_id LIKE 'atom-%' OR external_id LIKE 'gmail-row-%' OR external_id LIKE 'gmail-msg-%' OR external_id LIKE ':%')
                         ORDER BY LENGTH(body_html) DESC, LENGTH(body_text) DESC, received_at DESC
                         LIMIT 1
                     )",
                    params![item.source.as_str(), item.account, item.subject, item.external_id],
                );
            }
            let _ = tx.execute(
                "DELETE FROM items
                 WHERE source = ?1
                   AND account = ?2
                   AND subject = ?3
                   AND external_id != ?4
                   AND (external_id LIKE 'tag:gmail.google.com%' OR external_id LIKE 'atom-%' OR external_id LIKE 'gmail-row-%' OR external_id LIKE 'gmail-msg-%' OR external_id LIKE ':%')",
                params![item.source.as_str(), item.account, item.subject, item.external_id],
            );
        }

        let effective_ext_id = item.external_id.clone();
        let preserve_analysis = tx
            .query_row(
                "SELECT sender, subject, body_text FROM items
                 WHERE source = ?1 AND account = ?2 AND external_id = ?3
                   AND EXISTS(SELECT 1 FROM ai_jobs WHERE item_id = items.id AND status = 'done')",
                params![item.source.as_str(), item.account, effective_ext_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )
            .optional()
            .map_err(|e| e.to_string())?
            .is_some_and(|(sender, subject, body)| {
                sender == item.sender
                    && subject == item.subject
                    && self.unprotect_field(&body, &effective_ext_id) == item.body_text
            });
        let body_text_stored = self.protect_field(&item.body_text, &effective_ext_id);
        let body_html_stored = self.protect_field(&item.body_html, &effective_ext_id);
        let codes_stored = if item.extracted_codes.is_empty() {
            "[]".to_string()
        } else {
            self.protect_field(&codes_raw, &effective_ext_id)
        };

        tx.execute(
            "INSERT INTO items (
                    source, account, external_id, sender, sender_domain, recipient, subject,
                    body_text, body_html, received_at, state, snoozed_until, pinned, labels_json,
                    category, codes_json, links_json, entities_json, phishing_score,
                    phishing_warning, unsubscribe_link, summary, priority_score,
                    gmail_labels_json, list_unsubscribe_header, precedence_header
                ) VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7,
                    ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                    ?15, ?16, ?17, ?18, ?19,
                    ?20, ?21, ?22, ?23,
                    ?24, ?25, ?26
                )
                ON CONFLICT(source, account, external_id) DO UPDATE SET
                    sender = excluded.sender,
                    subject = excluded.subject,
                    body_text = CASE
                        WHEN LENGTH(excluded.body_text) >= LENGTH(items.body_text) AND excluded.body_text != '' THEN excluded.body_text
                        WHEN items.body_text = '' THEN excluded.body_text
                        ELSE items.body_text
                    END,
                    body_html = CASE
                        WHEN LENGTH(excluded.body_html) >= LENGTH(items.body_html) AND excluded.body_html != '' THEN excluded.body_html
                        WHEN items.body_html = '' THEN excluded.body_html
                        ELSE items.body_html
                    END,
                    state = CASE
                        WHEN items.state IN ('trash', 'archived', 'spam', 'read') THEN items.state
                        ELSE excluded.state
                    END,
                    snoozed_until = COALESCE(items.snoozed_until, excluded.snoozed_until),
                    pinned = CASE
                        WHEN items.pinned = 1 THEN 1
                        ELSE excluded.pinned
                    END,
                    labels_json = CASE
                        WHEN LENGTH(items.body_text) > LENGTH(excluded.body_text) AND items.category != 'other' THEN items.labels_json
                        ELSE excluded.labels_json
                    END,
                    category = CASE
                        WHEN ?27 THEN items.category
                        WHEN items.category != 'other' AND (excluded.category = 'other' OR LENGTH(items.body_text) > LENGTH(excluded.body_text)) THEN items.category
                        ELSE excluded.category
                    END,
                    codes_json = CASE
                        WHEN items.codes_json != '[]' AND LENGTH(items.codes_json) >= LENGTH(excluded.codes_json) THEN items.codes_json
                        ELSE excluded.codes_json
                    END,
                    links_json = CASE
                        WHEN items.links_json != '[]' AND LENGTH(items.links_json) >= LENGTH(excluded.links_json) THEN items.links_json
                        ELSE excluded.links_json
                    END,
                    entities_json = CASE
                        WHEN items.entities_json != '[]' AND LENGTH(items.entities_json) >= LENGTH(excluded.entities_json) THEN items.entities_json
                        ELSE excluded.entities_json
                    END,
                    phishing_score = MAX(items.phishing_score, excluded.phishing_score),
                    phishing_warning = COALESCE(excluded.phishing_warning, items.phishing_warning),
                    unsubscribe_link = COALESCE(excluded.unsubscribe_link, items.unsubscribe_link),
                    summary = CASE
                        WHEN ?27 THEN items.summary
                        WHEN LENGTH(items.body_text) > LENGTH(excluded.body_text) AND items.summary IS NOT NULL THEN items.summary
                        ELSE excluded.summary
                    END,
                    priority_score = CASE WHEN ?27 THEN items.priority_score ELSE MAX(items.priority_score, excluded.priority_score) END,
                    gmail_labels_json = excluded.gmail_labels_json",
            params![
                item.source.as_str(),
                item.account,
                effective_ext_id,
                item.sender,
                item.sender_domain,
                item.recipient,
                item.subject,
                body_text_stored,
                body_html_stored,
                item.received_at,
                item.state.as_str(),
                item.snoozed_until,
                i32::from(item.pinned),
                labels_json,
                item.category.as_str(),
                codes_stored,
                links_json,
                entities_json,
                item.phishing_score,
                item.phishing_warning,
                item.unsubscribe_link,
                item.summary.as_ref().map(|summary| self.protect_field(summary, "item-summary")),
                item.priority_score,
                gmail_labels_json,
                item.list_unsubscribe_header,
                item.precedence_header,
                preserve_analysis,
            ],
        )
        .map_err(|e| format!("upsert_item failed: {e}"))?;

        let row_id: i64 = tx
            .query_row(
                "SELECT id FROM items WHERE source = ?1 AND account = ?2 AND external_id = ?3",
                params![item.source.as_str(), item.account, effective_ext_id],
                |row| row.get(0),
            )
            .map_err(|e| format!("query item id failed: {e}"))?;

        let _ = tx.execute("DELETE FROM items_fts WHERE rowid = ?1", params![row_id]);
        let fts_body = if self.encryption_key.is_some() {
            ""
        } else {
            item.body_text.as_str()
        };
        tx.execute(
            "INSERT INTO items_fts(rowid, sender, subject, body_text, labels)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                row_id,
                item.sender,
                item.subject,
                fts_body,
                item.labels.join(" ")
            ],
        )
        .map_err(|e| format!("fts index update failed: {e}"))?;

        tx.commit()
            .map_err(|e| format!("commit upsert_item failed: {e}"))?;

        self.last_write_ms
            .store(start.elapsed().as_millis() as u64, Ordering::Relaxed);
        Ok(row_id)
    }

    pub fn get_item(&self, id: i64) -> Result<Option<Item>, String> {
        self.conn
            .query_row(
                "SELECT id, source, account, external_id, sender, sender_domain, recipient,
                        subject, body_text, body_html, received_at, state, snoozed_until,
                        pinned, labels_json, category, codes_json, links_json, entities_json,
                        phishing_score, phishing_warning, unsubscribe_link, summary,
                        priority_score, gmail_labels_json, list_unsubscribe_header,
                        precedence_header
                 FROM items WHERE id = ?1",
                params![id],
                |row| self.map_item_row(row),
            )
            .optional()
            .map_err(|e| format!("get_item failed: {e}"))
    }

    pub fn list_items(&self, filter: &ItemFilter) -> Result<Vec<Item>, String> {
        let limit = if filter.limit == 0 { 100 } else { filter.limit };
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, source, account, external_id, sender, sender_domain, recipient,
                        subject, body_text, body_html, received_at, state, snoozed_until,
                        pinned, labels_json, category, codes_json, links_json, entities_json,
                        phishing_score, phishing_warning, unsubscribe_link, summary,
                        priority_score, gmail_labels_json, list_unsubscribe_header,
                        precedence_header
                 FROM items
                 WHERE (?1 IS NULL OR source = ?1)
                   AND (?2 IS NULL OR account = ?2)
                   AND (?3 IS NULL OR category = ?3)
                   AND ((?4 IS NULL AND state NOT IN ('trash', 'archived', 'spam')) OR state = ?4)
                   AND (?5 = 0 OR state = 'unread')
                   AND (?6 = 0 OR pinned = 1)
                 ORDER BY pinned DESC, received_at DESC
                 LIMIT ?7",
            )
            .map_err(|e| format!("list_items prepare failed: {e}"))?;

        let rows = stmt
            .query_map(
                params![
                    filter.source.map(SourceKind::as_str),
                    filter.account.as_deref(),
                    filter.category.map(ItemCategory::as_str),
                    filter.state.map(ItemState::as_str),
                    i32::from(filter.unread_only),
                    i32::from(filter.pinned_only),
                    limit as i64,
                ],
                |row| self.map_item_row(row),
            )
            .map_err(|e| format!("list_items query failed: {e}"))?;

        let mut items = Vec::new();
        for r in rows {
            items.push(r.map_err(|e| format!("row decode failed: {e}"))?);
        }
        Ok(items)
    }

    pub fn status_digest(&self) -> Result<(usize, usize), String> {
        self.conn
            .query_row(
                "SELECT COUNT(*), COALESCE(SUM(LENGTH(body_html) + LENGTH(body_text)), 0)
                 FROM items
                 WHERE state NOT IN ('trash', 'archived', 'spam')",
                [],
                |row| {
                    let count: i64 = row.get(0)?;
                    let total_len: i64 = row.get(1)?;
                    Ok((count.max(0) as usize, total_len.max(0) as usize))
                },
            )
            .map_err(|e| format!("status_digest failed: {e}"))
    }

    pub fn search_fts(&self, query: &str, limit: usize) -> Result<Vec<Item>, String> {
        let sanitized = sanitize_fts_query(query);
        if sanitized.is_empty() {
            return Ok(Vec::new());
        }
        let max_rows = if limit == 0 { 50 } else { limit };
        let mut stmt = self
            .conn
            .prepare(
                "SELECT i.id, i.source, i.account, i.external_id, i.sender, i.sender_domain,
                        i.recipient, i.subject, i.body_text, i.body_html, i.received_at,
                        i.state, i.snoozed_until, i.pinned, i.labels_json, i.category,
                        i.codes_json, i.links_json, i.entities_json, i.phishing_score,
                        i.phishing_warning, i.unsubscribe_link, i.summary, i.priority_score,
                        i.gmail_labels_json, i.list_unsubscribe_header, i.precedence_header
                 FROM items_fts f
                 JOIN items i ON i.id = f.rowid
                 WHERE items_fts MATCH ?1
                   AND i.state NOT IN ('trash', 'archived', 'spam')
                 ORDER BY rank, i.received_at DESC
                 LIMIT ?2",
            )
            .map_err(|e| format!("search_fts prepare failed: {e}"))?;

        let rows = stmt
            .query_map(params![sanitized, max_rows as i64], |row| {
                self.map_item_row(row)
            })
            .map_err(|e| format!("search_fts query failed: {e}"))?;

        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| format!("fts row decode failed: {e}"))?);
        }
        Ok(out)
    }

    pub fn apply_action(&self, item_id: i64, action: &QuickAction) -> Result<(), String> {
        match action {
            QuickAction::Archive => {
                self.conn
                    .execute(
                        "UPDATE items SET state = 'archived' WHERE id = ?1",
                        params![item_id],
                    )
                    .map_err(|e| e.to_string())?;
                let _ = self.conn.execute(
                    "UPDATE recent_codes SET expires_at = 0 WHERE item_id = ?1",
                    params![item_id],
                );
            }
            QuickAction::Delete => {
                self.conn
                    .execute(
                        "UPDATE items SET state = 'trash' WHERE id = ?1",
                        params![item_id],
                    )
                    .map_err(|e| e.to_string())?;
                let _ = self.conn.execute(
                    "UPDATE recent_codes SET expires_at = 0 WHERE item_id = ?1",
                    params![item_id],
                );
            }
            QuickAction::MarkRead => {
                self.conn
                    .execute(
                        "UPDATE items SET state = 'read' WHERE id = ?1",
                        params![item_id],
                    )
                    .map_err(|e| e.to_string())?;
            }
            QuickAction::MarkUnread => {
                self.conn
                    .execute(
                        "UPDATE items SET state = 'unread' WHERE id = ?1",
                        params![item_id],
                    )
                    .map_err(|e| e.to_string())?;
            }
            QuickAction::Pin => {
                self.conn
                    .execute(
                        "UPDATE items SET pinned = 1 WHERE id = ?1",
                        params![item_id],
                    )
                    .map_err(|e| e.to_string())?;
            }
            QuickAction::Unpin => {
                self.conn
                    .execute(
                        "UPDATE items SET pinned = 0 WHERE id = ?1",
                        params![item_id],
                    )
                    .map_err(|e| e.to_string())?;
            }
            QuickAction::Snooze { until_epoch } => {
                self.conn
                    .execute(
                        "UPDATE items SET state = 'snoozed', snoozed_until = ?2 WHERE id = ?1",
                        params![item_id, until_epoch],
                    )
                    .map_err(|e| e.to_string())?;
            }
            QuickAction::ReportSpam => {
                self.conn
                    .execute(
                        "UPDATE items SET state = 'spam' WHERE id = ?1",
                        params![item_id],
                    )
                    .map_err(|e| e.to_string())?;
            }
            QuickAction::FlagNotACode { pattern } => {
                if let Some(item) = self.get_item(item_id)? {
                    self.add_not_a_code_rule(&item.sender_domain, pattern)?;
                    self.conn
                        .execute(
                            "UPDATE items SET codes_json = '[]', category = CASE WHEN category = 'code_otp' THEN 'other' ELSE category END WHERE id = ?1",
                            params![item_id],
                        )
                        .map_err(|e| e.to_string())?;
                }
            }
            QuickAction::Unsubscribe | QuickAction::OpenVerificationLink => {}
        }
        if matches!(
            action,
            QuickAction::Archive
                | QuickAction::Delete
                | QuickAction::MarkRead
                | QuickAction::MarkUnread
                | QuickAction::Snooze { .. }
                | QuickAction::ReportSpam
        ) {
            self.conn.execute(
                "UPDATE items SET ai_hidden = NULL, ai_keep = 1 WHERE id = ?1 AND ai_hidden IS NOT NULL",
                params![item_id],
            ).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn record_recent_code(&self, event: &OtpPopupEvent) -> Result<bool, String> {
        let code_enc = self.protect_field(&event.code, &event.code_hash);
        let inserted = self
            .conn
            .execute(
                "INSERT OR IGNORE INTO recent_codes (
                    code_hash, item_id, message_id, code_enc, sender, service,
                    received_at, expires_at, verification_link, phishing_warning,
                    detected_latency_ms
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    event.code_hash,
                    event.item_id,
                    event.message_id,
                    code_enc,
                    event.sender,
                    event.service,
                    event.received_at,
                    event.expires_at,
                    event.verification_link,
                    event.phishing_warning,
                    event.detected_latency_ms as i64,
                ],
            )
            .map_err(|e| format!("record_recent_code failed: {e}"))?;
        Ok(inserted > 0)
    }

    pub fn list_recent_codes(&self, now_epoch: i64) -> Result<Vec<OtpPopupEvent>, String> {
        let total_tracked: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM recent_codes", [], |r| r.get(0))
            .unwrap_or(0);
        if total_tracked == 0 {
            let mut seed_stmt = self
                .conn
                .prepare(
                    "SELECT id, account, external_id, sender, COALESCE(recipient, ''), subject, codes_json, received_at, state
                     FROM items
                     WHERE category = 'code_otp'
                       AND state NOT IN ('trash', 'archived', 'spam')
                     ORDER BY received_at DESC
                     LIMIT 25",
                )
                .map_err(|e| e.to_string())?;
            type CodeSeed = (
                i64,
                String,
                String,
                String,
                String,
                String,
                String,
                i64,
                String,
            );
            let seed_rows: Vec<CodeSeed> = seed_stmt
                .query_map([], |r| {
                    Ok((
                        r.get(0)?,
                        r.get(1)?,
                        r.get(2)?,
                        r.get(3)?,
                        r.get(4)?,
                        r.get(5)?,
                        r.get(6)?,
                        r.get(7)?,
                        r.get(8)?,
                    ))
                })
                .map_err(|e| e.to_string())?
                .flatten()
                .collect();
            let mut activated_latest = false;
            for (
                item_id,
                account,
                ext_id,
                sender,
                recipient,
                subject,
                codes_stored,
                recv_at,
                state_str,
            ) in seed_rows
            {
                let raw = self.unprotect_field(&codes_stored, &ext_id);
                let extracted: Vec<crate::model::ExtractedCode> =
                    serde_json::from_str(&raw).unwrap_or_default();
                if let Some(first) = extracted.first() {
                    if first.expires_at > 0 && first.expires_at <= now_epoch {
                        continue;
                    }
                    let hash = OtpPopupEvent::compute_dedup_hash(
                        &format!("{account}:{subject}"),
                        &first.code,
                    );
                    let exp = if first.expires_at > now_epoch {
                        first.expires_at
                    } else if !activated_latest && state_str == "unread" {
                        activated_latest = true;
                        now_epoch + 600
                    } else {
                        0
                    };
                    let ev = OtpPopupEvent {
                        item_id,
                        message_id: ext_id,
                        code_hash: hash,
                        code: first.code.clone(),
                        sender,
                        account,
                        recipient,
                        service: first.service.clone(),
                        received_at: recv_at,
                        expires_at: exp,
                        verification_link: first.verification_link.clone(),
                        phishing_warning: None,
                        detected_latency_ms: 1,
                    };
                    let _ = self.record_recent_code(&ev);
                }
            }
        }

        let mut stmt = self
            .conn
            .prepare(
                "SELECT c.code_hash, c.item_id, c.message_id, c.code_enc, c.sender, c.service,
                        c.received_at, c.expires_at, c.verification_link, c.phishing_warning,
                        c.detected_latency_ms, COALESCE(i.account, ''), COALESCE(i.recipient, '')
                 FROM recent_codes c LEFT JOIN items i ON i.id = c.item_id
                 WHERE c.expires_at > ?1
                   AND COALESCE(i.state, '') NOT IN ('trash', 'archived', 'spam')
                 ORDER BY c.received_at DESC
                 LIMIT 25",
            )
            .map_err(|e| format!("list_recent_codes prepare failed: {e}"))?;

        let rows = stmt
            .query_map(params![now_epoch], |row| {
                let code_hash: String = row.get(0)?;
                let code_enc: String = row.get(3)?;
                let code = self.unprotect_field(&code_enc, &code_hash);
                Ok(OtpPopupEvent {
                    code_hash,
                    item_id: row.get(1)?,
                    message_id: row.get(2)?,
                    code,
                    sender: row.get(4)?,
                    account: row.get(11)?,
                    recipient: row.get(12)?,
                    service: row.get(5)?,
                    received_at: row.get(6)?,
                    expires_at: row.get(7)?,
                    verification_link: row.get(8)?,
                    phishing_warning: row.get(9)?,
                    detected_latency_ms: row.get::<_, i64>(10)? as u64,
                })
            })
            .map_err(|e| format!("list_recent_codes query failed: {e}"))?;

        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| e.to_string())?);
        }
        Ok(out)
    }

    pub fn prune_expired_bodies(&self, now_epoch: i64, keep_days: u32) -> Result<usize, String> {
        let cutoff = now_epoch - i64::from(keep_days) * 86_400;
        for sql in [
            "DELETE FROM ai_cache WHERE updated_at < ?1",
            "DELETE FROM ai_call_log WHERE created_at < ?1",
        ] {
            self.conn
                .execute(sql, params![cutoff])
                .map_err(|e| format!("AI retention cleanup failed: {e}"))?;
        }
        let _ = self.conn.execute(
            "UPDATE items_fts
             SET body_text = ''
             WHERE rowid IN (
                 SELECT id FROM items
                 WHERE received_at < ?1 AND (body_text != '' OR body_html != '')
             )",
            params![cutoff],
        );
        let updated = self
            .conn
            .execute(
                "UPDATE items
                 SET body_text = '', body_html = ''
                 WHERE received_at < ?1 AND (body_text != '' OR body_html != '')",
                params![cutoff],
            )
            .map_err(|e| format!("prune_expired_bodies failed: {e}"))?;
        let _ = self.conn.execute(
            "DELETE FROM recent_codes WHERE received_at < ?1",
            params![now_epoch - 7 * 86_400],
        );
        Ok(updated)
    }

    pub fn expire_old_code_items(&self, now_epoch: i64, archive: bool) -> Result<usize, String> {
        let target_state = if archive { "archived" } else { "read" };
        let count = self
            .conn
            .execute(
                "UPDATE items
                 SET state = ?2
                 WHERE category = 'code_otp'
                   AND state = 'unread'
                   AND id IN (
                       SELECT item_id FROM recent_codes WHERE expires_at <= ?1
                   )",
                params![now_epoch, target_state],
            )
            .map_err(|e| format!("expire_old_code_items failed: {e}"))?;
        Ok(count)
    }

    pub fn add_not_a_code_rule(&self, sender_domain: &str, pattern: &str) -> Result<(), String> {
        self.conn
            .execute(
                "INSERT OR IGNORE INTO not_a_code_rules (sender_domain, pattern)
                 VALUES (?1, ?2)",
                params![sender_domain.to_ascii_lowercase(), pattern.trim()],
            )
            .map_err(|e| format!("add_not_a_code_rule failed: {e}"))?;
        Ok(())
    }

    pub fn is_flagged_not_a_code(
        &self,
        sender_domain: &str,
        candidate: &str,
    ) -> Result<bool, String> {
        let count: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM not_a_code_rules
                 WHERE (sender_domain = ?1 OR sender_domain = '*') AND pattern = ?2",
                params![sender_domain.to_ascii_lowercase(), candidate.trim()],
                |row| row.get(0),
            )
            .map_err(|e| format!("is_flagged_not_a_code failed: {e}"))?;
        Ok(count > 0)
    }

    pub fn list_not_a_code_patterns(&self, sender_domain: &str) -> Result<Vec<String>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT pattern FROM not_a_code_rules
                 WHERE sender_domain = ?1 OR sender_domain = '*'",
            )
            .map_err(|e| format!("list_not_a_code_patterns prepare failed: {e}"))?;
        let rows = stmt
            .query_map(params![sender_domain.to_ascii_lowercase()], |row| {
                row.get::<_, String>(0)
            })
            .map_err(|e| format!("list_not_a_code_patterns query failed: {e}"))?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| e.to_string())?);
        }
        Ok(out)
    }

    pub fn max_numeric_external_id(
        &self,
        source: SourceKind,
        account: &str,
    ) -> Result<u32, String> {
        let incomplete_count: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*)
                 FROM items
                 WHERE source = ?1
                   AND account = ?2
                   AND (body_html = '' OR LENGTH(body_text) < 260)",
                params![source.as_str(), account],
                |row| row.get(0),
            )
            .unwrap_or(0);
        if incomplete_count > 0 {
            return Ok(0);
        }
        let max_uid: Option<i64> = self
            .conn
            .query_row(
                "SELECT MAX(CAST(external_id AS INTEGER))
                 FROM items
                 WHERE source = ?1
                   AND account = ?2
                   AND external_id GLOB '[0-9]*'
                   AND NOT external_id GLOB '*[^0-9]*'",
                params![source.as_str(), account],
                |row| row.get(0),
            )
            .map_err(|e| format!("max_numeric_external_id failed: {e}"))?;
        Ok(max_uid.unwrap_or(0).max(0) as u32)
    }

    pub fn save_account(&self, account: &AccountConfig) -> Result<(), String> {
        let json = serde_json::to_string(account).map_err(|e| e.to_string())?;
        self.conn
            .execute(
                "INSERT INTO accounts (id, config_json) VALUES (?1, ?2)
                 ON CONFLICT(id) DO UPDATE SET config_json = excluded.config_json",
                params![account.id, json],
            )
            .map_err(|e| format!("save_account failed: {e}"))?;
        Ok(())
    }

    pub fn list_accounts(&self) -> Result<Vec<AccountConfig>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT config_json FROM accounts ORDER BY id ASC")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for r in rows {
            let json = r.map_err(|e| e.to_string())?;
            if let Ok(cfg) = serde_json::from_str::<AccountConfig>(&json) {
                out.push(cfg);
            }
        }
        Ok(out)
    }

    pub fn remove_account(&self, id: &str) -> Result<bool, String> {
        let _ = self.conn.execute(
            "DELETE FROM recent_codes WHERE item_id IN (SELECT id FROM items WHERE account = ?1)",
            params![id],
        );
        let _ = self.conn.execute(
            "DELETE FROM items_fts WHERE rowid IN (SELECT id FROM items WHERE account = ?1)",
            params![id],
        );
        let _ = self
            .conn
            .execute("DELETE FROM items WHERE account = ?1", params![id]);
        let n = self
            .conn
            .execute("DELETE FROM accounts WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(n > 0)
    }

    pub fn save_user_rule(&self, rule: &UserRuleConfig) -> Result<(), String> {
        let json = serde_json::to_string(rule).map_err(|e| e.to_string())?;
        self.conn
            .execute(
                "INSERT INTO user_rules (name, rule_json) VALUES (?1, ?2)
                 ON CONFLICT(name) DO UPDATE SET rule_json = excluded.rule_json",
                params![rule.name, json],
            )
            .map_err(|e| format!("save_user_rule failed: {e}"))?;
        Ok(())
    }

    pub fn list_user_rules(&self) -> Result<Vec<UserRuleConfig>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT rule_json FROM user_rules ORDER BY name ASC")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for r in rows {
            let json = r.map_err(|e| e.to_string())?;
            if let Ok(rule) = serde_json::from_str::<UserRuleConfig>(&json) {
                out.push(rule);
            }
        }
        Ok(out)
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>, String> {
        self.conn
            .query_row(
                "SELECT value FROM app_settings WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| e.to_string())
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), String> {
        self.conn
            .execute(
                "INSERT INTO app_settings(key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                params![key, value],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn save_ai_settings(&self, json: &str, accounts: &[AccountConfig]) -> Result<(), String> {
        let tx = self
            .conn
            .unchecked_transaction()
            .map_err(|e| e.to_string())?;
        self.set_setting("ai", json)?;
        for account in accounts {
            self.save_account(account)?;
        }
        tx.commit().map_err(|e| e.to_string())
    }

    pub fn queue_ai_item(&self, item_id: i64, cache_key: &str, force: bool) -> Result<(), String> {
        self.conn.execute(
            "INSERT INTO ai_jobs(item_id, cache_key, status, manual) VALUES (?1, ?2, 'pending', ?3)
             ON CONFLICT(item_id) DO UPDATE SET cache_key = excluded.cache_key, status = 'pending', error = NULL, manual = excluded.manual
             WHERE ai_jobs.cache_key != excluded.cache_key OR ?3", params![item_id, cache_key, force],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn next_ai_job(&self) -> Result<Option<(i64, String)>, String> {
        self.conn.query_row(
            "SELECT item_id, cache_key FROM ai_jobs WHERE status = 'pending' ORDER BY manual DESC, item_id DESC LIMIT 1",
            [], |row| Ok((row.get(0)?, row.get(1)?)),
        ).optional().map_err(|e| e.to_string())
    }

    pub fn ai_job_is_current(&self, item_id: i64, cache_key: &str) -> Result<bool, String> {
        self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM ai_jobs WHERE item_id = ?1 AND cache_key = ?2 AND status = 'pending')",
            params![item_id, cache_key], |row| row.get(0),
        ).map_err(|e| e.to_string())
    }

    pub fn finish_ai_job(
        &self,
        item_id: i64,
        cache_key: &str,
        error: Option<&str>,
    ) -> Result<bool, String> {
        let error = error.map(|e| e.chars().take(500).collect::<String>());
        self.conn.execute(
            "UPDATE ai_jobs SET status = ?3, error = ?4, manual = 0 WHERE item_id = ?1 AND cache_key = ?2 AND status = 'pending'",
            params![item_id, cache_key, if error.is_some() { "failed" } else { "done" }, error],
        ).map(|n| n > 0).map_err(|e| e.to_string())
    }

    pub fn ai_job_status(&self, item_id: i64) -> Result<Option<(String, Option<String>)>, String> {
        self.conn
            .query_row(
                "SELECT status, error FROM ai_jobs WHERE item_id = ?1",
                params![item_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(|e| e.to_string())
    }

    pub fn ai_auto_hide_allowed(&self, item_id: i64) -> Result<bool, String> {
        self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM items WHERE id = ?1 AND ai_keep = 0 AND pinned = 0 AND state IN ('read', 'unread'))",
            params![item_id], |row| row.get(0),
        ).map_err(|e| e.to_string())
    }

    pub fn apply_ai_analysis(
        &self,
        item_id: i64,
        summary: &str,
        category: ItemCategory,
        priority: u8,
        hide: bool,
    ) -> Result<bool, String> {
        self.conn.execute(
            "UPDATE items SET summary = ?2, category = ?3, priority_score = ?4,
                ai_hidden = CASE WHEN ?5 AND pinned = 0 AND ai_keep = 0 THEN state ELSE ai_hidden END,
                state = CASE WHEN ?5 AND pinned = 0 AND ai_keep = 0 THEN 'archived' ELSE state END
             WHERE id = ?1 AND state IN ('read', 'unread')",
            params![item_id, self.protect_field(summary, "item-summary"), category.as_str(), priority.min(100), hide],
        ).map(|n| n > 0).map_err(|e| e.to_string())
    }

    pub fn restore_ai_item(&self, item_id: i64) -> Result<bool, String> {
        self.conn
            .execute(
                "UPDATE items SET state = ai_hidden, ai_hidden = NULL, ai_keep = 1
             WHERE id = ?1 AND state = 'archived' AND ai_hidden IN ('read', 'unread')",
                params![item_id],
            )
            .map(|n| n > 0)
            .map_err(|e| e.to_string())
    }

    pub fn list_ai_hidden(&self, account: Option<&str>, limit: usize) -> Result<Vec<Item>, String> {
        let mut stmt = self.conn.prepare(
            "SELECT id, source, account, external_id, sender, sender_domain, recipient,
                    subject, body_text, body_html, received_at, state, snoozed_until,
                    pinned, labels_json, category, codes_json, links_json, entities_json,
                    phishing_score, phishing_warning, unsubscribe_link, summary,
                    priority_score, gmail_labels_json, list_unsubscribe_header, precedence_header
             FROM items WHERE ai_hidden IS NOT NULL AND state = 'archived' AND (?1 IS NULL OR account = ?1)
             ORDER BY received_at DESC LIMIT ?2",
        ).map_err(|e| e.to_string())?;
        stmt.query_map(params![account, limit.clamp(1, 500) as i64], |row| {
            self.map_item_row(row)
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
    }

    pub fn get_ai_cache(&self, content_hash: &str, task: &str) -> Result<Option<String>, String> {
        self.conn
            .query_row(
                "SELECT response_json FROM ai_cache WHERE content_hash = ?1 AND task = ?2",
                params![content_hash, task],
                |row| Ok(self.unprotect_field(&row.get::<_, String>(0)?, content_hash)),
            )
            .optional()
            .map_err(|e| e.to_string())
    }

    pub fn set_ai_cache(
        &self,
        content_hash: &str,
        task: &str,
        response_json: &str,
        now_epoch: i64,
    ) -> Result<(), String> {
        self.conn
            .execute(
                "INSERT INTO ai_cache (content_hash, task, response_json, updated_at)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(content_hash, task) DO UPDATE SET
                   response_json = excluded.response_json,
                   updated_at = excluded.updated_at",
                params![
                    content_hash,
                    task,
                    self.protect_field(response_json, content_hash),
                    now_epoch
                ],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn log_ai_call(
        &self,
        created_at: i64,
        provider: &str,
        model: &str,
        task: &str,
        estimated_tokens: usize,
        payload_preview: &str,
    ) -> Result<(), String> {
        self.conn
            .execute(
                "INSERT INTO ai_call_log (
                    created_at, provider, model, task, estimated_tokens, payload_preview
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    created_at,
                    provider,
                    model,
                    task,
                    estimated_tokens as i64,
                    self.protect_field(payload_preview, "ai-call-log")
                ],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn list_ai_calls(&self, limit: usize) -> Result<Vec<AiCallLogEntry>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, created_at, provider, model, task, estimated_tokens, payload_preview
                 FROM ai_call_log ORDER BY id DESC LIMIT ?1",
            )
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map(params![limit as i64], |row| {
                Ok(AiCallLogEntry {
                    id: row.get(0)?,
                    created_at: row.get(1)?,
                    provider: row.get(2)?,
                    model: row.get(3)?,
                    task: row.get(4)?,
                    estimated_tokens: row.get::<_, i64>(5)? as usize,
                    payload_preview: self.unprotect_field(&row.get::<_, String>(6)?, "ai-call-log"),
                })
            })
            .map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| e.to_string())?);
        }
        Ok(out)
    }

    pub fn upsert_embedding(&self, item_id: i64, vector: &[f32]) -> Result<(), String> {
        let mut blob = Vec::with_capacity(vector.len() * 4);
        for v in vector {
            blob.extend_from_slice(&v.to_le_bytes());
        }
        self.conn
            .execute(
                "INSERT INTO item_embeddings (item_id, vector_blob) VALUES (?1, ?2)
                 ON CONFLICT(item_id) DO UPDATE SET vector_blob = excluded.vector_blob",
                params![item_id, blob],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn semantic_search(
        &self,
        query_vec: &[f32],
        limit: usize,
    ) -> Result<Vec<(Item, f32)>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT item_id, vector_blob FROM item_embeddings")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, Vec<u8>>(1)?))
            })
            .map_err(|e| e.to_string())?;

        let mut scored = Vec::new();
        for r in rows {
            let (item_id, blob) = r.map_err(|e| e.to_string())?;
            if blob.len() % 4 != 0 {
                continue;
            }
            let vec: Vec<f32> = blob
                .as_chunks::<4>()
                .0
                .iter()
                .map(|c| f32::from_le_bytes(*c))
                .collect();
            let sim = cosine_similarity(query_vec, &vec);
            scored.push((item_id, sim));
        }
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(limit.max(1));

        let mut out = Vec::new();
        for (item_id, sim) in scored {
            if let Some(item) = self.get_item(item_id)? {
                out.push((item, sim));
            }
        }
        Ok(out)
    }

    pub fn register_paired_device(&self, dev: &PairedDevice) -> Result<(), String> {
        self.conn
            .execute(
                "INSERT INTO paired_devices (
                    device_id, device_name, shared_secret_hash, paired_at, revoked
                 ) VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(device_id) DO UPDATE SET
                   device_name = excluded.device_name,
                   shared_secret_hash = excluded.shared_secret_hash,
                   revoked = excluded.revoked",
                params![
                    dev.device_id,
                    dev.device_name,
                    dev.shared_secret_hash,
                    dev.paired_at,
                    i32::from(dev.revoked)
                ],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn revoke_paired_device(&self, device_id: &str) -> Result<bool, String> {
        let updated = self
            .conn
            .execute(
                "UPDATE paired_devices SET revoked = 1 WHERE device_id = ?1",
                params![device_id],
            )
            .map_err(|e| e.to_string())?;
        Ok(updated > 0)
    }

    pub fn list_paired_devices(&self) -> Result<Vec<PairedDevice>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT device_id, device_name, shared_secret_hash, paired_at, revoked
                 FROM paired_devices ORDER BY paired_at DESC",
            )
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| {
                Ok(PairedDevice {
                    device_id: row.get(0)?,
                    device_name: row.get(1)?,
                    shared_secret_hash: row.get(2)?,
                    paired_at: row.get(3)?,
                    revoked: row.get::<_, i32>(4)? != 0,
                })
            })
            .map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| e.to_string())?);
        }
        Ok(out)
    }

    fn map_item_row(&self, row: &rusqlite::Row<'_>) -> rusqlite::Result<Item> {
        let external_id: String = row.get(3)?;
        let raw_body_text: String = row.get(8)?;
        let raw_body_html: String = row.get(9)?;
        let labels_json: String = row.get(14)?;
        let raw_codes_json: String = row.get(16)?;
        let links_json: String = row.get(17)?;
        let entities_json: String = row.get(18)?;
        let gmail_labels_json: String = row.get(24)?;

        let body_text = self.unprotect_field(&raw_body_text, &external_id);
        let body_html = self.unprotect_field(&raw_body_html, &external_id);
        let codes_json = self.unprotect_field(&raw_codes_json, &external_id);

        let labels: Vec<String> = serde_json::from_str(&labels_json).unwrap_or_default();
        let extracted_codes: Vec<ExtractedCode> =
            serde_json::from_str(&codes_json).unwrap_or_default();
        let extracted_links: Vec<ExtractedLink> =
            serde_json::from_str(&links_json).unwrap_or_default();
        let extracted_entities: Vec<ExtractedEntity> =
            serde_json::from_str(&entities_json).unwrap_or_default();
        let gmail_labels: Vec<String> =
            serde_json::from_str(&gmail_labels_json).unwrap_or_default();

        Ok(Item {
            id: row.get(0)?,
            source: SourceKind::from_str_name(&row.get::<_, String>(1)?),
            account: row.get(2)?,
            external_id,
            sender: row.get(4)?,
            sender_domain: row.get(5)?,
            recipient: row.get(6)?,
            subject: row.get(7)?,
            body_text,
            body_html,
            received_at: row.get(10)?,
            state: ItemState::from_str_name(&row.get::<_, String>(11)?),
            snoozed_until: row.get(12)?,
            pinned: row.get::<_, i32>(13)? != 0,
            labels,
            category: ItemCategory::from_str_name(&row.get::<_, String>(15)?),
            extracted_codes,
            extracted_links,
            extracted_entities,
            phishing_score: row.get::<_, u8>(19)?,
            phishing_warning: row.get(20)?,
            unsubscribe_link: row.get(21)?,
            summary: row
                .get::<_, Option<String>>(22)?
                .map(|summary| self.unprotect_field(&summary, "item-summary")),
            priority_score: row.get::<_, u8>(23)?,
            gmail_labels,
            list_unsubscribe_header: row.get(25)?,
            precedence_header: row.get(26)?,
        })
    }
}

fn sanitize_fts_query(q: &str) -> String {
    let tokens: Vec<String> = q
        .split_whitespace()
        .map(|w| {
            w.chars()
                .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '@')
                .collect::<String>()
        })
        .filter(|w| !w.is_empty())
        .map(|w| format!("\"{w}\""))
        .collect();
    tokens.join(" AND ")
}

fn compute_field_mac(key: &[u8; 32], nonce: &[u8], cipher: &[u8]) -> [u8; 16] {
    let mut hasher = Sha256::new();
    hasher.update(b"vortex-enc2-mac:");
    hasher.update(key);
    hasher.update(nonce);
    hasher.update((cipher.len() as u64).to_le_bytes());
    hasher.update(cipher);
    let digest = hasher.finalize();
    let mut mac = [0u8; 16];
    mac.copy_from_slice(&digest[..16]);
    mac
}

fn xor_stream_cipher(key: &[u8; 32], context: &[u8], data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    for (block_idx, chunk) in data.chunks(32).enumerate() {
        let mut hasher = Sha256::new();
        hasher.update(key);
        hasher.update(context);
        hasher.update((block_idx as u64).to_le_bytes());
        let stream = hasher.finalize();
        for (b, k) in chunk.iter().zip(stream.iter()) {
            out.push(b ^ k);
        }
    }
    out
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;
    for (x, y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }
    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom <= f32::EPSILON {
        0.0
    } else {
        dot / denom
    }
}

#[cfg(test)]
mod ai_tests {
    use super::*;

    fn message(external_id: &str) -> Item {
        Item::new(
            SourceKind::Email,
            "me@example.com",
            external_id,
            "news@example.com",
            "Weekly offers",
            "A newsletter with weekly offers.",
            1_700_000_000,
        )
    }

    #[test]
    fn ai_queue_is_durable_deduplicated_and_rejects_stale_completion() {
        let store = SqliteStore::open_in_memory(false, None).unwrap();
        let id = store.upsert_item(&message("queue")).unwrap();
        store.queue_ai_item(id, "first", false).unwrap();
        assert_eq!(store.next_ai_job().unwrap(), Some((id, "first".into())));
        store.queue_ai_item(id, "second", false).unwrap();
        assert!(!store.ai_job_is_current(id, "first").unwrap());
        assert!(!store.finish_ai_job(id, "first", None).unwrap());
        assert!(store.finish_ai_job(id, "second", None).unwrap());
        store.queue_ai_item(id, "second", false).unwrap();
        assert!(store.next_ai_job().unwrap().is_none());
        store.queue_ai_item(id, "second", true).unwrap();
        assert!(
            store
                .finish_ai_job(id, "second", Some("provider unavailable"))
                .unwrap()
        );
        assert_eq!(
            store.ai_job_status(id).unwrap(),
            Some(("failed".into(), Some("provider unavailable".into())))
        );
        store.set_setting("ai", "{\"enabled\":false}").unwrap();
        assert_eq!(
            store.get_setting("ai").unwrap().as_deref(),
            Some("{\"enabled\":false}")
        );
        store
            .conn
            .execute("DELETE FROM items WHERE id = ?1", params![id])
            .unwrap();
        assert!(store.ai_job_status(id).unwrap().is_none());
    }

    #[test]
    fn ai_settings_roll_back_account_failures() {
        let store = SqliteStore::open_in_memory(false, None).unwrap();
        store.set_setting("ai", "previous").unwrap();
        let account = |id: &str| {
            serde_json::from_value::<AccountConfig>(serde_json::json!({
                "id": id, "source": "email", "email_or_handle": id,
                "host": "imap.example.com", "port": 993, "ai_enabled": true,
            }))
            .unwrap()
        };
        store
            .conn
            .execute_batch(
                "CREATE TRIGGER reject_account BEFORE INSERT ON accounts
            WHEN NEW.id = 'blocked' BEGIN SELECT RAISE(ABORT, 'test failure'); END;",
            )
            .unwrap();
        assert!(
            store
                .save_ai_settings("next", &[account("allowed"), account("blocked")])
                .is_err()
        );
        assert_eq!(
            store.get_setting("ai").unwrap().as_deref(),
            Some("previous")
        );
        assert!(store.list_accounts().unwrap().is_empty());
        store
            .save_ai_settings("next", &[account("allowed")])
            .unwrap();
        assert_eq!(store.get_setting("ai").unwrap().as_deref(), Some("next"));
        assert!(store.list_accounts().unwrap()[0].ai_enabled);
    }

    #[test]
    fn manual_ai_requests_take_priority_over_new_arrivals() {
        let store = SqliteStore::open_in_memory(false, None).unwrap();
        let older = store.upsert_item(&message("older")).unwrap();
        let newer = store.upsert_item(&message("newer")).unwrap();
        store.queue_ai_item(older, "old", false).unwrap();
        store.queue_ai_item(newer, "new", false).unwrap();
        assert_eq!(store.next_ai_job().unwrap(), Some((newer, "new".into())));
        store.queue_ai_item(older, "old", true).unwrap();
        assert_eq!(store.next_ai_job().unwrap(), Some((older, "old".into())));
        store.finish_ai_job(older, "old", None).unwrap();
        store.queue_ai_item(older, "old", false).unwrap();
        assert_eq!(store.next_ai_job().unwrap(), Some((newer, "new".into())));
        store.queue_ai_item(older, "old", true).unwrap();
        store
            .finish_ai_job(older, "old", Some("unavailable"))
            .unwrap();
        store.queue_ai_item(older, "old", false).unwrap();
        assert_eq!(store.next_ai_job().unwrap(), Some((newer, "new".into())));
        store.queue_ai_item(older, "changed", false).unwrap();
        assert_eq!(store.next_ai_job().unwrap(), Some((newer, "new".into())));
    }

    #[test]
    fn ai_cache_and_logs_follow_body_retention_without_requeuing_done_mail() {
        let store = SqliteStore::open_in_memory(false, None).unwrap();
        let now = 1_700_000_000;
        let cutoff = now - 7 * 86_400;
        store
            .set_ai_cache("expired", "classify", "old", cutoff - 1)
            .unwrap();
        store
            .set_ai_cache("boundary", "classify", "retained", cutoff)
            .unwrap();
        store
            .set_ai_cache("recent", "classify", "new", now)
            .unwrap();
        for timestamp in [cutoff - 1, cutoff, now] {
            store
                .log_ai_call(timestamp, "ollama", "small", "classify", 10, "excerpt")
                .unwrap();
        }
        let id = store.upsert_item(&message("done")).unwrap();
        store.queue_ai_item(id, "expired", false).unwrap();
        store.finish_ai_job(id, "expired", None).unwrap();
        store.prune_expired_bodies(now, 7).unwrap();
        assert!(store.get_ai_cache("expired", "classify").unwrap().is_none());
        assert!(
            store
                .get_ai_cache("boundary", "classify")
                .unwrap()
                .is_some()
        );
        assert!(store.get_ai_cache("recent", "classify").unwrap().is_some());
        let log = store.list_ai_calls(10).unwrap();
        assert_eq!(log.len(), 2);
        assert!(log.iter().all(|entry| entry.created_at >= cutoff));
        store.queue_ai_item(id, "expired", false).unwrap();
        assert!(store.next_ai_job().unwrap().is_none());
    }

    #[test]
    fn ai_queue_survives_reopen_and_metadata_migration_is_idempotent() {
        let name = format!(
            "vortex-ai-{}-{}.db",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let path = std::env::temp_dir().join(name);
        let id = {
            let store = SqliteStore::open(&path, false, None).unwrap();
            let id = store.upsert_item(&message("persisted")).unwrap();
            store.queue_ai_item(id, "persisted-key", false).unwrap();
            store.conn.execute_batch("ALTER TABLE items DROP COLUMN ai_hidden; ALTER TABLE items DROP COLUMN ai_keep;
                DROP INDEX idx_ai_jobs_priority; ALTER TABLE ai_jobs DROP COLUMN manual;").unwrap();
            id
        };
        {
            let store = SqliteStore::open(&path, false, None).unwrap();
            assert_eq!(
                store.next_ai_job().unwrap(),
                Some((id, "persisted-key".into()))
            );
            assert!(store.ai_auto_hide_allowed(id).unwrap());
            store
                .apply_ai_analysis(id, "Offers", ItemCategory::PromoNewsletter, 3, true)
                .unwrap();
        }
        {
            let store = SqliteStore::open(&path, false, None).unwrap();
            assert_eq!(store.list_ai_hidden(None, 1).unwrap().len(), 1);
            assert!(store.restore_ai_item(id).unwrap());
        }
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn ai_hide_is_reversible_and_respects_manual_states_and_pins() {
        let store = SqliteStore::open_in_memory(false, None).unwrap();
        let original = message("hide");
        let id = store.upsert_item(&original).unwrap();
        store.queue_ai_item(id, "key", false).unwrap();
        assert!(
            store
                .apply_ai_analysis(
                    id,
                    "Weekly promotions",
                    ItemCategory::PromoNewsletter,
                    5,
                    true
                )
                .unwrap()
        );
        store.finish_ai_job(id, "key", None).unwrap();
        assert!(store.list_items(&ItemFilter::default()).unwrap().is_empty());
        assert_eq!(store.list_ai_hidden(None, 10).unwrap().len(), 1);
        store.upsert_item(&original).unwrap();
        let analyzed = store.get_item(id).unwrap().unwrap();
        assert_eq!(analyzed.summary.as_deref(), Some("Weekly promotions"));
        assert_eq!(analyzed.priority_score, 5);
        assert_eq!(analyzed.category, ItemCategory::PromoNewsletter);
        assert!(store.restore_ai_item(id).unwrap());
        assert_eq!(
            store.get_item(id).unwrap().unwrap().state,
            ItemState::Unread
        );
        assert!(!store.ai_auto_hide_allowed(id).unwrap());
        store
            .apply_ai_analysis(id, "Still visible", ItemCategory::PromoNewsletter, 5, true)
            .unwrap();
        assert_eq!(
            store.get_item(id).unwrap().unwrap().state,
            ItemState::Unread
        );

        let pinned = store.upsert_item(&message("pinned")).unwrap();
        store.apply_action(pinned, &QuickAction::Pin).unwrap();
        store
            .apply_ai_analysis(
                pinned,
                "Pinned mail",
                ItemCategory::PromoNewsletter,
                4,
                true,
            )
            .unwrap();
        assert_eq!(
            store.get_item(pinned).unwrap().unwrap().state,
            ItemState::Unread
        );
        store.apply_action(pinned, &QuickAction::Delete).unwrap();
        assert!(
            !store
                .apply_ai_analysis(pinned, "Ignore", ItemCategory::Other, 90, true)
                .unwrap()
        );
        assert_eq!(
            store.get_item(pinned).unwrap().unwrap().state,
            ItemState::Trash
        );
    }

    #[test]
    fn ai_storage_encrypts_cache_and_logs_and_preserves_encrypted_resync() {
        let store = SqliteStore::open_in_memory(false, Some("test-encryption-key")).unwrap();
        store
            .set_ai_cache("hash", "classify", "private summary", 1)
            .unwrap();
        assert_eq!(
            store.get_ai_cache("hash", "classify").unwrap().as_deref(),
            Some("private summary")
        );
        let raw: String = store
            .conn
            .query_row("SELECT response_json FROM ai_cache", [], |row| row.get(0))
            .unwrap();
        assert!(raw.starts_with("enc2:"));
        store
            .log_ai_call(1, "ollama", "small", "classify", 10, "private excerpt")
            .unwrap();
        assert_eq!(
            store.list_ai_calls(1).unwrap()[0].payload_preview,
            "private excerpt"
        );
        let raw: String = store
            .conn
            .query_row("SELECT payload_preview FROM ai_call_log", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert!(raw.starts_with("enc2:"));
        let item = message("encrypted");
        let id = store.upsert_item(&item).unwrap();
        store.queue_ai_item(id, "same", false).unwrap();
        store
            .apply_ai_analysis(id, "Generated summary", ItemCategory::Work, 17, false)
            .unwrap();
        store.finish_ai_job(id, "same", None).unwrap();
        store.upsert_item(&item).unwrap();
        assert_eq!(
            store.get_item(id).unwrap().unwrap().summary.as_deref(),
            Some("Generated summary")
        );
        assert_eq!(store.get_item(id).unwrap().unwrap().priority_score, 17);
        let raw: String = store
            .conn
            .query_row(
                "SELECT summary FROM items WHERE id = ?1",
                params![id],
                |row| row.get(0),
            )
            .unwrap();
        assert!(raw.starts_with("enc2:"));
    }
}
