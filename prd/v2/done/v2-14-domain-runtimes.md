# PRD v2 Task 14 — Specialized Domain Runtimes

Status: Done

## Goal

Move domain support from generic workspace profiles toward concrete runtime packs with real verifier, effect ledger, memory schema, and report/dashboard artifacts.

## Acceptance

- Add runtime pack metadata for code, infra, data, media, and research domains.
- Add at least one concrete code runtime pack such as Rust, Next.js, FastAPI, or Python package.
- Add at least one concrete infra/data/media/research runtime pack where existing verifiers can provide meaningful evidence.
- Each runtime pack declares supported workspace profiles, verifier profiles, effects, artifacts, and memory schemas.
- Transaction artifacts include selected runtime pack metadata when a pack applies.
- Reports and dashboard payloads expose domain runtime pack artifacts.
- Missing tools degrade to structured warnings rather than panics.
- Add tests for pack selection, artifact generation, missing-tool compatibility, and transaction integration.
- README and docs are updated in English, Russian, Chinese, and Kazakh for user-facing behavior.
- Module-size check stays under 200 lines per Rust/JS implementation file.

## Completed

- Added a Domain Runtime catalog for `code.rust`, `infra.terraform`, `data.python`, `media.render`, and `research.citations`.
- Added pack metadata for supported workspaces, verifier profiles, expected effects, artifacts, memory schemas, and required tools.
- Added runtime pack selection by workspace, verifier profile, and project files.
- Added structured missing-tool warnings that do not panic transactions.
- Added `.agent/tx/<tx-id>/domain_runtime.json` transaction artifacts.
- Added Domain Runtime report section and dashboard payload field `transactions[].domain_runtime`.
- Added tests for pack selection, artifact generation, missing-tool compatibility, dashboard payload, and transaction integration.
- Added Domain Runtimes docs in English, Russian, Chinese, and Kazakh and updated README/web-dashboard docs.

## Evidence

- Implementation commit: `433a29f Complete domain runtimes task`
- `cargo fmt -- --check`
- `scripts/check-module-size.sh 200`
- `git diff --check`
- `cargo test domain_runtime`
- `cargo test web_dashboard::tests`
- `cargo test successful_transaction_commits_and_promotes_memory`
- `cargo clippy -- -D warnings`
- `cargo test`
- `npm run check` in `editors/vscode`
