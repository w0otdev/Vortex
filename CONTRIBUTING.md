# Contributing to Vortex Inbox

## Quality requirements

Every change must pass these checks before merging:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --release -p vortex-cli
```

On Windows:

```powershell
./scripts/check-budgets.ps1
```

On Linux or macOS:

```sh
sh scripts/check-budgets.sh
```

## Resource budgets

- `unsafe_code = "forbid"` across the entire workspace.
- LITE tier: stripped release binary `< 6 MB`, idle RSS `< 15 MB`, single-thread runtime, web UI disabled by default.
- STANDARD tier: stripped release binary `< 6 MB`, idle RSS `< 25 MB`, embedded web UI assets `< 150 KB`.
- FULL tier: stripped release binary `< 12 MB`, idle RSS `< 50 MB`.
- Any new dependency exceeding `200 KB` in `cargo bloat --release -p vortex-cli` must be justified in the pull request description.
