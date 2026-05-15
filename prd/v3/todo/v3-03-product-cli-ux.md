# PRD v3 Task 03 — Product CLI UX

Status: Todo

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
