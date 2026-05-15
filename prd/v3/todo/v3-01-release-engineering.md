# PRD v3 Task 01 — Release Engineering

Status: Todo

## Goal

Add the release engineering foundation required for an installable local developer preview.

## Acceptance

- Add GitHub Actions CI for Linux, macOS, and Windows.
- CI runs `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test`, `scripts/check-module-size.sh 200`, AAL parse smoke, and no-commit transaction smoke where supported.
- Add release workflow skeleton for tagged builds.
- Add `scripts/smoke-test.sh`.
- Add `CHANGELOG.md`, `LICENSE`, `SECURITY.md`, and `CONTRIBUTING.md`.
- Update README/docs in English, Russian, Chinese, and Kazakh.
- Module-size check stays under 200 lines per Rust/JS implementation file.
