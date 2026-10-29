<p align="center">
  <img src="docs/logo.png" width="100" height="100" alt="Vortex">
</p>

<h2 align="center">Vortex</h2>

<p align="center">
  A local-first inbox and live OTP companion for Windows
</p>

<p align="center">
  <a href="https://github.com/w0otdev/Vortex/releases"><img src="https://img.shields.io/badge/release-v0.1.0-ffffff?style=for-the-badge&labelColor=eeeeee&logo=github&logoColor=111111" height="28" alt="Release"></a>
  <img src="https://img.shields.io/badge/Windows-10%20%7C%2011-ffffff?style=for-the-badge&labelColor=eeeeee&logo=windows&logoColor=111111" height="28" alt="Windows 10 / 11">
  <img src="https://img.shields.io/badge/Rust-2024-ffffff?style=for-the-badge&labelColor=eeeeee&logo=rust&logoColor=111111" height="28" alt="Rust 2024">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-ffffff?style=for-the-badge&labelColor=eeeeee" height="28" alt="License"></a>
  <a href="https://github.com/w0otdev/Vortex"><img src="https://img.shields.io/badge/ WebView2-Native-ffffff?style=for-the-badge&labelColor=eeeeee&logo=microsoftedge&logoColor=111111" height="28" alt="WebView2 Native"></a>
</p>

<p align="center">
  <a href="https://github.com/w0otdev/Vortex/releases/latest">Download</a> ·
  <a href="#features">Features</a> ·
  <a href="#usage">Usage</a> ·
  <a href="#installation">Installation</a> ·
  <a href="#privacy">Privacy</a>
</p>

<p align="center">
  <sub><a href="https://github.com/w0otdev/Vortex/releases">All releases</a></sub>
</p>

<p align="center">
  Vortex merges multiple email accounts, RSS feeds, and messaging relays into one unified desktop stream. When a message arrives, it extracts verification codes in under 2 milliseconds and surfaces a floating always-on-top OTP card for one-click copying, while categorizing mail, scoring phishing risk, and indexing everything locally with SQLite FTS5.
</p>

<p align="center">
  <b>Free and open source forever.</b>
</p>

<p align="center">
  Vortex is an independent project.<br>
  If you use it regularly, you can <a href="https://www.paypal.me/PhuocLe678"><b>support development via PayPal</b></a>.
</p>

<div id="features"></div>

<div align="center">

<table align="center">
  <tr>
    <td align="center" valign="top" width="50%">
      <b>Live OTP overlay</b><br>
      <sub>Extracts 4 to 8 digit verification codes in under 2 ms and displays a floating card at the corner of your screen for one-click copy.</sub>
    </td>
    <td align="center" valign="top" width="50%">
      <b>Multi-account &amp; bulk sync</b><br>
      <sub>Connect Gmail with one-click Google sign-in, or batch-import dozens of Gmail, Outlook, iCloud, Yahoo, and custom IMAP accounts in parallel.</sub>
    </td>
  </tr>
  <tr>
    <td align="center" valign="top" width="50%">
      <b>Smart categories &amp; auto-sort</b><br>
      <sub>Automatically routes messages into OTP, Billing, Orders, Security, Work, and Promotions, with optional priority sorting for unread codes.</sub>
    </td>
    <td align="center" valign="top" width="50%">
      <b>Phishing &amp; link guard</b><br>
      <sub>Flags suspicious senders and deceptive verification links before you click, and surfaces direct one-click verification and unsubscribe buttons.</sub>
    </td>
  </tr>
  <tr>
    <td align="center" valign="top" width="50%">
      <b>Paper-Ink &amp; Dark themes</b><br>
      <sub>Custom borderless native window with tactile 3D controls, smooth circular view transitions, and adaptive HTML email contrast normalization.</sub>
    </td>
    <td align="center" valign="top" width="50%">
      <b>Hardware-aware budgets</b><br>
      <sub>Scales automatically from headless single-core boards (under 15 MB RSS) to full desktop workstations with instant SQLite FTS5 prefix search.</sub>
    </td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="2">
      <b>Optional AI assistant</b><br>
      <sub>Opt in per mailbox for message summaries, priority sorting, and reversible auto-hiding of confirmed promotions. Connect an existing Ollama / llama.cpp service or OpenAI / Anthropic account. Disabled by default, with no bundled model or new runtime dependency.</sub>
    </td>
  </tr>
</table>

</div>

<div id="usage"></div>

## Usage

| Action | Result |
| :--- | :--- |
| Click floating OTP card | Copy verification code to clipboard and auto-clear after expiry |
| Click `×` on OTP card | Dismiss current code overlay |
| Drag top bar | Move the borderless native window |
| Double-click top bar | Toggle maximize / restore window |
| `Add mailbox` -> `Sign in with Google` | Connect a Gmail account without generating an App Password |
| `Add mailbox` -> `Bulk add accounts` | Import multiple `email\|app_password` lines and sync in parallel |
| Click category pill | Filter stream by All, OTP, Billing, Orders, Security, Work, or Promotions |
| Click `Auto-sort` | Pin active OTP codes and unread messages to the top of the stream |
| Click `AI` | Enable the optional assistant, choose a provider/model, and select allowed mailboxes |
| Click `Summarize with AI` | Analyze an existing message; new arrivals in opted-in mailboxes are analyzed automatically |
| Enable `Auto-hide promotions` | Locally archive promotions only when the classifier and model agree with high confidence |
| Open `Hidden by AI` → `Restore message` | Restore the message and exempt it from future automatic hiding |
| Click speaker / theme / language | Toggle tactile audio feedback, Light / Dark theme, or Vietnamese / English UI |

<details>
<summary><strong>CLI commands</strong></summary>

| Command | Result |
| :--- | :--- |
| `vortex` or `vortex daemon` | Launch the native desktop app and background sync daemon |
| `vortex status` | Show detected hardware tier, RAM, CPU cores, and item counts |
| `vortex account list` | List connected mailboxes |
| `vortex account add <email> <host> <port> <secret>` | Save credentials in the OS keyring and register an account |
| `vortex sync` | Trigger an immediate sync across all connected accounts |
| `vortex codes` | Print active OTP codes and remaining expiration times |
| `vortex inbox --q "<query>"` | Search local messages via SQLite FTS5 |
| `vortex action <archive\|delete\|read\|pin> <id>` | Apply a quick action to a message by ID |
| `vortex rule add <name> "<dsl>"` | Add a custom classification rule |
| `vortex install-service <dir>` | Generate an OS startup manifest (Windows, systemd, or launchd) |

</details>

### Device tiers and budgets

Vortex probes hardware resources at startup (`RAM`, `CPU cores`, `arch`, `battery/AC`, `headless`) and selects a runtime policy automatically. You can override this in `vortex.toml` with `profile = "auto" | "lite" | "standard" | "full"`.

| Tier | Target hardware | Binary budget | Idle RSS budget | Runtime defaults |
| :--- | :--- | :--- | :--- | :--- |
| `lite` | Raspberry Pi, NAS, small VPS (`<= 1 GB` RAM or 1 core) | `< 6 MB` | `< 15 MB` | Single-thread runtime, CLI + native notifications, 7-day body retention, AI off |
| `standard` | Laptop or desktop | `< 6 MB` | `< 25 MB` | Multi-thread runtime, native WebView2 UI (`< 150 KB` bundle), FTS5 prefix search, 30-day retention |
| `full` | Workstation (`>= 16 GB` RAM, `>= 8` cores on AC) | `< 12 MB` | `< 50 MB` | Everything in `standard` plus optional local AI (`Ollama` / `llama.cpp`) and 90-day retention |

<div id="installation"></div>

## Installation

Vortex runs on 64-bit Windows 10 or Windows 11 and uses the built-in Microsoft Edge WebView2 runtime for its native window and floating OTP overlay.

1. Download `vortex.exe` from [Releases](https://github.com/w0otdev/Vortex/releases/latest).
2. Run `vortex.exe` to open the desktop app.
3. Click **Add mailbox** to sign in with Google or paste your IMAP app passwords.

<details>
<summary><strong>Build from source</strong></summary>

```powershell
# Quick build and launch on Windows
.\r.ps1

# Or build the stripped release binary directly with Cargo
cargo build --release -p vortex-cli
```

The release binary is produced at `target/release/vortex.exe`.

</details>

<div id="privacy"></div>

## Privacy

Vortex runs locally on your machine and does not collect telemetry, analytics, or usage tracking.

- **OS keyring storage:** IMAP passwords and secrets are stored in Windows Credential Manager, macOS Keychain, or Linux Secret Service, and are never written in plain text to `vortex.toml`.
- **Sandboxed email rendering:** HTML emails render inside an isolated iframe with scripts stripped and remote tracking pixels blocked by default.
- **Local database:** Messages, extracted OTP hashes, and FTS5 search indexes live in a local SQLite database (`vortex.db`) on your disk.
- **Optional AI:** Only opted-in mailboxes use the configured provider. Requests use bounded, redacted excerpts; OTP processing remains independent. Saved assistant settings persist in SQLite and take precedence over TOML. See [AI Privacy](docs/AI_PRIVACY.md) for provider setup, auto-hide safeguards, and validation limits.

[Threat Model](THREAT_MODEL.md) · [AI Privacy](docs/AI_PRIVACY.md) · [Connectors](docs/CONNECTORS.md) · [Contributing](CONTRIBUTING.md)

## Other projects by w0otdev

If you like Vortex, check out other free and open-source Windows utilities by w0otdev:

- **[V-Notch](https://github.com/w0otdev/V-Notch)** - Desktop status, media controls, synced lyrics, and Dynamic Island companion for Windows.
- **[Vertex](https://github.com/w0otdev/Vertex)** - Minimalist radial wheel file converter for Windows (built with Rust).
- **[Scroll-V](https://github.com/w0otdev/Scroll-V)** - Smooth scrolling experience for all Windows applications (built with C#).

## License

Vortex is licensed under the MIT License. See [LICENSE](LICENSE) for details.

<p align="center">
  Made by <a href="https://w0otdev.id.vn">w0otdev</a> ·
  <a href="https://github.com/w0otdev/Vortex">GitHub</a> ·
  <a href="https://www.facebook.com/rain.107/">Facebook</a> ·
  <a href="https://www.paypal.me/PhuocLe678">Donate</a>
</p>
