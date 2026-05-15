# PRD v3 Task 03 - Product CLI UX

Status: Done

## Goal

Add user-facing CLI commands that make AgentHub feel like an installable product instead of only a runtime library.

## Acceptance

- Add `agenthub doctor`.
- Add `agenthub version`.
- Add provider commands: list, status, setup, test.
- Add config commands: show and set.
- `doctor` checks git, project initialization, policy, supported provider binaries, and OS basics.
- Provider setup/status/test degrade to actionable messages when tools are missing.
- Add tests for doctor/provider/config output.
- Update README/docs in English, Russian, Chinese, and Kazakh.
- Module-size check stays under 200 lines per Rust/JS implementation file.

## Completed

- Added `agenthub doctor`.
- Added `agenthub version`.
- Added provider commands:
  - `agenthub providers list`
  - `agenthub providers status`
  - `agenthub providers setup <provider>`
  - `agenthub providers test <provider>`
- Added config commands:
  - `agenthub config show`
  - `agenthub config set <key> <value>`
- Added `.agent/config.yaml` key/value config support.
- Added product CLI smoke coverage to `scripts/smoke-test.sh`.
- Added tests for doctor/provider/config output.
- Added product CLI docs in English, Russian, Chinese, and Kazakh.
- Updated all four README files and changelog.

## Evidence

- Implementation commit: `a731a30 Add product CLI commands`
- `doctor` checks OS/architecture, Git, Git repository status, `.agent` initialization, policy files, and provider binaries.
- Provider commands treat missing Codex/Gemini/Kimi CLIs as actionable missing-provider messages instead of panics.
- `config show` falls back to `default_provider command` when no config exists.

## Validation

- `git diff --check`
- `scripts/check-module-size.sh 200`
- `cargo fmt -- --check`
- `cargo clippy --locked -- -D warnings`
- `cargo test --locked product_cli`
- `cargo test --locked`
- `npm --prefix editors/vscode run check`
- `target/debug/agenthub version`
- `target/debug/agenthub providers list`
- `target/debug/agenthub providers test command`
- `target/debug/agenthub --project /tmp/... config show`
- `target/debug/agenthub --project /tmp/... config set default_provider command`
- `scripts/smoke-test.sh`
