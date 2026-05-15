# PRD v2 Task 09 — AAL v2 Semantics

Status: Done

## Goal

Move AAL from parser-only DSL toward a versioned language with semantic diagnostics, imports, compatibility checks, and live-validation friendly output.

## Acceptance

- Add `aal "0.2"` version handling while preserving existing AAL examples.
- Add import declarations for skills/rules with semantic validation stubs.
- Add semantic diagnostics for unknown skills, unknown verifier profiles, workspace/skill incompatibility, policy conflicts, and route smoke preconditions.
- Add structured diagnostics output suitable for editor/LSP use.
- Add formatter or normalized rendering for parsed AAL.
- Add tests for parser compatibility, semantic errors, and valid v0.2 examples.
- README and docs are updated in English, Russian, Chinese, and Kazakh for user-facing behavior.
- Module-size check stays under 200 lines per Rust/JS implementation file.

## Completed

- Added AAL v0.2 preamble support with `aal "0.2"` while preserving existing v0.1-style examples.
- Added `import skill` and `import rules` parsing with version metadata for semantic tooling.
- Split parser responsibilities into smaller modules for statements, preamble handling, semantic validation, and formatting.
- Added structured `AalDiagnostic` output with stable `code`, `severity`, `line`, `message`, optional `help`, and JSON serialization.
- Added semantic diagnostics for unsupported versions, unknown skill namespaces, unknown verifier profiles, workspace/skill mismatches, exact allow/deny overlaps, and runtime smoke preconditions.
- Added normalized AAL rendering through `parsed.normalized`, `format_aal`, and `format_aal_file`.
- Updated `agenthub aal parse` to stop before YAML output when semantic errors are present.
- Added parser compatibility, valid v0.2, normalized rendering, and structured semantic diagnostic tests.
- Updated README and AAL docs in English, Russian, Chinese, and Kazakh.

## Evidence

- Implementation commit: `7d2aaf1`.
- Checks: `cargo fmt -- --check`; `scripts/check-module-size.sh 200`; `git diff --check`; `cargo test aal::tests`; `cargo clippy -- -D warnings`; `cargo test`; `npm run check` in `editors/vscode`.
