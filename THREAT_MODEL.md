# Threat model

## Security boundaries

Vortex Inbox runs locally on the user's machine or private home hub. The project operates no cloud servers, collects no analytics or telemetry, and makes network connections only to endpoints explicitly configured by the user (IMAP servers, OAuth token endpoints, Telegram/Slack/Matrix/Discord APIs, RSS feeds, or user-configured AI endpoints).

## Credential storage

- Account passwords, OAuth refresh tokens, bot tokens, AI API keys, and multi-device pairing secrets are stored exclusively in the operating system keyring (`vortex-inbox` service via Windows Credential Manager, macOS Keychain, or Linux Secret Service).
- `AppConfig::parse_and_validate` rejects any `vortex.toml` file containing `password =`, `api_key =`, or `secret =`.
- `ExtractedCode` and `OtpPopupEvent` implement custom `Debug` formatting that outputs `[REDACTED_OTP]` so verification codes are never written to logs.

## Untrusted message content

All incoming email, RSS, SMS, and chat content is treated as untrusted input:

1. **HTML and script sanitization (`vortex_imap::mime::sanitize_html_safe`)**:
   - Removes `<script>`, `<style>`, `<iframe>`, `<object>`, and `<embed>` elements.
   - Removes inline `on*` event attributes and `javascript:` or `data:text/html` URLs.
   - Blocks remote tracking images (`http://` and `https://` `<img src>`) by default.
   - Removes hidden elements (`display:none`, `visibility:hidden`, `opacity:0`, `font-size:0`, white-on-white text) and invisible Unicode characters (`U+200B..U+200D`, `U+202A..U+202E`, `U+FEFF`).

2. **Phishing guard (`vortex_classify::evaluate_phishing`)**:
   - Flags punycode lookalike domains (`xn--`).
   - Detects display-name brand impersonation when the sender domain does not match official domains.
   - Flags verification, security, and billing messages where link domains do not match the sender's root domain.
   - Never auto-opens verification or unsubscribe links.

3. **Prompt injection defenses (`vortex_classify::ai`)**:
   - AI is disabled by default (`ai.enabled = false`).
   - Before any AI call, `redact_item_for_ai` removes OTP codes, payment card numbers, secret assignments, and quoted reply history.
   - External AI providers (`openai`, `anthropic`) are blocked on sensitive categories (`code_otp`, `security_alert`, `billing`) unless `allow_sensitive_external = true`.
   - Message data is enclosed inside `<untrusted_email_data>` tags with system instructions isolated.
   - Model responses are parsed against strict JSON schemas (`AiClassificationProposal`), and `requires_user_confirmation` is forced to `true`.

## Local database encryption and multi-device hub

- When `encrypt_database = true`, `SqliteStore` encrypts `body_text`, `body_html`, `codes_json`, and `recent_codes.code_enc` at rest using a stream cipher keyed by `db:master_key` in the OS keyring.
- In multi-device Hub mode, only the hub connects to mail servers; thin clients receive end-to-end encrypted payloads (`encrypt_e2e_payload` with per-envelope nonce and SHA-256 MAC verification) and can be revoked individually (`revoke_paired_device`).
