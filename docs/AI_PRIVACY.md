# AI privacy guide

AI features in Vortex Inbox are opt-in and disabled by default (`ai.enabled = false`). The fast OTP detection path never waits for or depends on AI.

## Redaction pipeline

Before any message is formatted for an AI provider, [`redact_item_for_ai`](../crates/classify/src/ai.rs) strips:

- Extracted OTP and verification codes (`[REDACTED_OTP]` / `[REDACTED_CODE]`)
- Payment card numbers of 13 to 19 digits (`[REDACTED_CARD]`)
- Password, secret, token, and API key assignments (`[REDACTED_SECRET]`)
- Quoted reply history (`>` lines and `On ... wrote:` blocks)
- Hidden HTML elements and zero-width Unicode characters

## Sensitive category block

When an external provider (`openai` or `anthropic`) is configured, messages classified as `code_otp`, `security_alert`, or `billing` are blocked from leaving the device unless the user explicitly sets `allow_sensitive_external = true` in `vortex.toml`.

## Exact payload preview and call log

- Before sending any data, call `GET /api/ai/preview?item_id=<id>` (or click **Xem dữ liệu gửi AI / AI data preview** in the web UI) to inspect the exact system prompt, redacted body, provider endpoint, and estimated token count.
- Every completed AI request is recorded in the local SQLite `ai_call_log` table with timestamp, provider, model, task, estimated tokens, and redacted payload preview.
- Responses are cached by SHA-256 content hash in `ai_cache` so identical messages are never sent twice.
