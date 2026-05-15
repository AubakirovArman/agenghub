# PRD v3 Task 02 — Installers And Packaging

Status: Todo

## Goal

Make AgentHub installable from source and release artifacts without requiring users to understand the repo internals.

## Acceptance

- Add POSIX `scripts/install.sh`.
- Add Windows `scripts/install.ps1`.
- Add `scripts/package.sh` for local release archive creation.
- Document `cargo install --path .` and future `cargo install --git` flow.
- Document GitHub Releases archive naming.
- Update README/docs in English, Russian, Chinese, and Kazakh.
- Module-size check stays under 200 lines per Rust/JS implementation file.
