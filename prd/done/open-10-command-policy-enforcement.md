# Open Task 10 — Command Policy Enforcement

Status: Done

Closing evidence: implementation commit `e448b7f`; verified with `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo test command_policy`, `scripts/check-module-size.sh 200`, `git diff --check`, and `npm run check` in `editors/vscode/`.

Source: `prd/audit/open/README.md`, `prd.md`

## Goal

Enforce safe, needs_approval, and restricted command policy lists at execution time.

## Acceptance

- [x] Implementation exists: `.agent/policies/core.yaml` safe, `needs_approval`, and restricted command lists are evaluated before execution.
- [x] README and docs are updated in English, Russian, Chinese, and Kazakh.
- [x] Tests and smoke checks cover `needs_approval` blocking and restricted command rejection.
- [x] Module-size check stays under 200 lines per Rust/JS implementation file.
- [x] Task moved to `prd/done/` with closing evidence.
