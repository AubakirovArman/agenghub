# PRD v2 Task 14 — Specialized Domain Runtimes

Status: Todo

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
