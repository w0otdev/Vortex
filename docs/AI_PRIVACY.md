# AI privacy guide

The AI assistant is optional and disabled by default (`ai.enabled = false`). Click **AI**, choose a provider and model, select the mailboxes that may use AI, and save. Enabling AI without selecting a mailbox sends no mail. Turning it off stops new requests; an already running request may finish at the provider, but its result is ignored.

## Setup and storage

- **Ollama / llama.cpp:** use an existing service on a loopback address such as `http://127.0.0.1:11434`. Vortex does not install a model, download weights, or embed an inference runtime.
- **OpenAI / Anthropic:** require an HTTPS base endpoint, a model available to your account, and an API key. Keys are saved in the OS keyring, never returned by the settings API or written to TOML.
- Settings saved in the app, including mailbox consent, persist in local SQLite. Saved AI settings take precedence over the `[ai]` section in `vortex.toml` on subsequent starts.
- With database encryption enabled, newly written summaries, AI cache responses, and call-log payload excerpts use the existing local field encryption. Previously stored plaintext entries are not rewritten automatically.

## Behavior and resource use

New arrivals in opted-in mailboxes are analyzed automatically. For existing mail, select a message and click **Summarize with AI**. Enabling the assistant does not backfill your entire mailbox. Summaries appear in the reader; **Auto-sort** uses their priority scores while retaining OTP and unread priority.

One optional background worker reads message IDs from an indexed SQLite queue. Provider I/O runs outside the daemon state lock, so OTP detection and inbox API requests do not wait for the model. AI pauses under the existing battery and Lite-tier policies. There are no new package dependencies; the initial UI asset budget remains 150 KiB and feedback audio loads on demand.

Interactive summary requests take priority over queued automatic analysis after the current provider request finishes.

Responses are cached by message content, provider, endpoint, model, task, and prompt contract. Completed and failed jobs do not retry on every sync; requesting a summary again explicitly retries. Changed message content gets a new job. Manual read, archive, delete, and pin actions remain authoritative when a response arrives.

Cache entries and call logs expire during sync under the same 7/30/90-day body-retention policy for the current device tier. Completion records remain attached to messages, so cache expiry alone does not trigger another provider request.

## Auto-hide and recovery

**Auto-hide promotions** is a separate opt-in setting. A message can be hidden only when both the existing classifier and model identify it as a promotion/newsletter, model confidence is at least 90%, and there are no detected codes, actionable entities, pin, snooze, or high phishing score. Rule-detected OTP, security, billing, shipping, and work messages retain their protected categories.

Hiding archives the message locally; it does not delete it or move it at the mail provider. Open **Hidden by AI** and choose **Restore message** to recover its previous read/unread state. Restored messages are permanently exempt from automatic hiding. Hidden messages remain recoverable when the assistant is disabled.

## Redaction pipeline

Before a message is formatted for a provider, [`redact_item_for_ai`](../crates/classify/src/ai.rs) builds a bounded excerpt from sender, subject, and plain text. Its redaction rules cover:

- Extracted OTP and verification codes (`[REDACTED_OTP]` / `[REDACTED_CODE]`)
- Payment card numbers of 13 to 19 digits (`[REDACTED_CARD]`)
- Password, secret, token, and API key assignments (`[REDACTED_SECRET]`)
- Quoted reply history (`>` lines and `On ... wrote:` blocks)

The transmitted excerpt is limited to 2,400 UTF-8 bytes. Mail content is marked as untrusted data in the prompt, and model output must pass a bounded JSON schema check. Redaction is pattern based; sender names and ordinary message content remain visible to the chosen provider.

## Sensitive category block

External providers (`openai` or `anthropic`) cannot receive messages classified as `code_otp`, `security_alert`, or `billing` unless `allow_sensitive_external` was explicitly enabled in the AI configuration. This exception is disabled by default and has no toggle in the assistant dialog. Local providers require loopback endpoints.

## Exact payload preview and call log

- `GET /api/ai/preview?item_id=<id>` shows the system prompt, redacted excerpt, endpoint, estimated token count, and blocking reason before a manual request.
- Accepted successful provider responses are recorded in local `ai_call_log` with provider, model, task, estimated tokens, and redacted excerpt. Canceled, failed, and cached requests do not create a successful-call record.
- `GET /api/ai/result?item_id=<id>` reports job status and errors. Provider failures leave mail available with its normal local classification.

## Validation limits

Automated tests use local fake provider servers and temporary SQLite databases. Live provider credentials, model quality, and end-to-end RSS/latency budgets require measurement on the target machine; the app does not infer those measurements from the asset-size check.
