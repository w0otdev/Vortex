# Authoring a connector

Every message source in Vortex Inbox implements the [`Connector`](../crates/core/src/model.rs) trait in `vortex-core` and produces unified [`Item`](../crates/core/src/model.rs) values. Adding a new connector never requires modifying the OTP detector, classifier, SQLite store, or web UI.

## Trait contract

```rust
pub trait Connector: Send {
    fn id(&self) -> &str;
    fn account(&self) -> &str;
    fn connect(&mut self) -> Result<(), String>;
    fn sync(&mut self, since_epoch: Option<i64>) -> Result<Vec<Item>, String>;
    fn watch(&mut self, timeout_ms: u64) -> Result<Vec<Item>, String>;
    fn execute_action(&mut self, external_id: &str, action: &QuickAction) -> Result<(), String>;
}
```

## Rules for all connectors

1. Use official, Terms-of-Service-compliant APIs only. Never scrape web sessions or embed personal user tokens in source code.
2. Fetch credentials from [`SecretStore`](../crates/core/src/secrets.rs) (`vortex_core::secrets::SecretStore`) keyed by `account.id`.
3. Respect the tier poll floor: never poll faster than 30 seconds on AC power or 60 seconds on battery. Prefer push or long-polling (`IMAP IDLE`, Telegram `getUpdates` timeout) whenever the protocol supports it.
4. Sanitize any HTML body via `vortex_imap::mime::sanitize_html_safe` before storing.
