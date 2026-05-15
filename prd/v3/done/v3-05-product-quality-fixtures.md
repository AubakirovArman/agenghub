# PRD v3 Task 05 - Product Quality Fixtures

Status: Done

## Goal

Add fixture projects and product smoke tests that prove AgentHub works beyond unit tests.

## Acceptance

- Add or extend fixtures for Rust, Next.js/reference web, Python data, Terraform/infra, and content.
- Add scripts for transaction rollback, smart sync, provider dry-run, dashboard, and fixture smoke tests.
- CI can run the safe fixture smoke set.
- Add docs explaining how to run fixtures locally.
- Update README/docs in English, Russian, Chinese, and Kazakh.
- Module-size check stays under 200 lines per Rust/JS implementation file.

## Completed

- Added fixture projects:
  - `fixtures/rust-basic`
  - `fixtures/python-data`
  - `fixtures/terraform-basic`
  - `fixtures/content-basic`
  - existing `examples/reference-web-app` included in fixture smoke.
- Added product smoke scripts:
  - `scripts/test-fixtures.sh`
  - `scripts/test-transaction-rollback.sh`
  - `scripts/test-smart-sync.sh`
  - `scripts/test-provider-dry-run.sh`
  - `scripts/test-dashboard.sh`
- Added Linux CI fixture smoke.
- Added product fixture docs in English, Russian, Chinese, and Kazakh.
- Updated README and changelog.

## Evidence

- Implementation commit: `5d21533 Add product quality fixtures`
- `scripts/test-fixtures.sh` runs Rust, data, infra, content, and reference web fixture transactions.
- Separate scripts cover rollback, smart sync rebase, provider dry-run artifacts, and dashboard generation.

## Validation

- `scripts/test-fixtures.sh`
- `scripts/test-transaction-rollback.sh`
- `scripts/test-smart-sync.sh`
- `scripts/test-provider-dry-run.sh`
- `scripts/test-dashboard.sh`
- `bash -n scripts/test-fixtures.sh scripts/test-transaction-rollback.sh scripts/test-smart-sync.sh scripts/test-provider-dry-run.sh scripts/test-dashboard.sh scripts/smoke-test.sh`
- `git diff --check`
- `scripts/check-module-size.sh 200`
- `cargo fmt -- --check`
- `cargo test --locked`
- `npm --prefix editors/vscode run check`
