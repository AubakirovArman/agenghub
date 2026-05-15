# Open Task 09 — DB Migration Verifier

Status: Done

Closing evidence: implementation commit `1270c38`; verified with `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo test db_migration`, `scripts/check-module-size.sh 200`, `git diff --check`, and `npm run check` in `editors/vscode/`.

Source: `prd/audit/open/README.md`, `prd.md`

## Goal

Specialized db_migration verifier profile.

## Acceptance

- [x] Implementation exists: `db_migration` verifier profile with migration, schema diff, dry-run, rollback, and seed artifact checks.
- [x] README and docs are updated in English, Russian, Chinese, and Kazakh.
- [x] Tests and smoke checks cover domain verifier behavior and transaction execution.
- [x] Module-size check stays under 200 lines per Rust/JS implementation file.
- [x] Task moved to `prd/done/` with closing evidence.
