# PRD v2 Task 11 — Verifier Integrations v2

Status: Todo

## Goal

Move verifier output from command/domain checks toward a structured verifier integration layer with reusable check records, trendable artifacts, and memory feedback.

## Acceptance

- Add a structured verifier check schema that can represent code, infra, data, content, media, and research checks.
- Convert existing command, runtime smoke, and domain verifier outputs into a unified structured JSON artifact.
- Add verifier fingerprints for failed checks and feed them into failed-attempt memory or typed warning memory.
- Add verifier trend artifact data suitable for dashboard/analytics consumption.
- Keep existing verifier command behavior and domain profiles compatible.
- Make verifier integrations plugin-compatible where local plugin verifier metadata already exists.
- Report includes a structured verifier summary and trend/fingerprint references.
- Add tests for structured verifier JSON, failure fingerprints, plugin-compatible verifier metadata, report output, and compatibility.
- README and docs are updated in English, Russian, Chinese, and Kazakh for user-facing behavior.
- Module-size check stays under 200 lines per Rust/JS implementation file.
