# Vortex Inbox

Lightweight, local-first background agent that merges email accounts, RSS feeds, and messaging relays into one unified stream. When a message arrives, the fast path extracts verification codes (OTPs) in under 2 milliseconds and surfaces a one-click copy banner, while the background path classifies messages, scores phishing risk, extracts entities, and applies user rules.

## Demo

![Vortex Inbox demo](docs/demo.gif)

## Device tiers and budgets

Vortex detects hardware resources at startup (`RAM`, `CPU cores`, `arch`, `battery/AC`, `headless`) and adapts automatically. Override in `vortex.toml` with `profile = "auto" | "lite" | "standard" | "full"`.

| Tier | Target hardware | Binary budget | Idle RSS budget | Runtime defaults |
| :--- | :--- | :--- | :--- | :--- |
| `lite` | Raspberry Pi, NAS, small VPS (`<= 1 GB` RAM or 1 core) | `< 6 MB` | `< 15 MB` | Single-thread runtime, CLI + native notifications, IMAP IDLE, 7-day body retention, no FTS prefix index, AI off |
| `standard` | Laptop or desktop | `< 6 MB` | `< 25 MB` | Multi-thread runtime, local web UI (`< 150 KB`), FTS5 prefix search, 30-day body retention, clipboard auto-clear |
| `full` | Workstation (`>= 16 GB` RAM, `>= 8` cores on AC) | `< 12 MB` | `< 50 MB` | Everything in `standard` plus optional local AI (`Ollama` / `llama.cpp`), local vector semantic search, 90-day retention |

## Installation by tier

### Build from source (all tiers)

```bash
cargo build --release -p vortex-cli
```

The stripped binary is produced at `target/release/vortex` (`target/release/vortex.exe` on Windows).

### Connect a real account

Credentials are stored in your OS keyring (Windows Credential Manager, macOS Keychain, or Linux Secret Service) and never written to config files:

```bash
# Connect Gmail via IMAP (stores app password in OS keyring)
vortex account add you@gmail.com imap.gmail.com 993 "your-app-password"

# Sync immediately and inspect inbox or active OTP codes
vortex sync
vortex codes
vortex inbox
```

### Run the background daemon and web UI (Standard / Full)

```bash
vortex daemon --bind 127.0.0.1:7788
```

Open `http://127.0.0.1:7788` to connect accounts, copy live OTP codes, search via FTS5, or run quick actions (`j`/`k`, `e` archive, `r` read, `p` pin, `c` copy OTP, `Ctrl+K` command palette).

### Install as an OS startup service (Headless / NAS / Desktop)

```bash
# Writes systemd user unit on Linux, launchd plist on macOS, or startup script on Windows
vortex install-service ~/.config/systemd/user
```

## Documentation

- [Threat Model](THREAT_MODEL.md)
- [Connector Authoring Guide](docs/CONNECTORS.md)
- [AI Privacy Guide](docs/AI_PRIVACY.md)
- [Contributing & Budget Gates](CONTRIBUTING.md)

## License

MIT
