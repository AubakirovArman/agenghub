# PRD v3 Task 04 — Real Provider Gateway

Status: Todo

## Goal

Move LLM Gateway from provider plans/traces toward real provider execution for local developer preview.

## Acceptance

- Add an OpenAI-compatible HTTP provider path.
- Add real CLI provider invocation with transcript capture.
- Add provider retry/backoff around real calls.
- Add provider status/test integration with CLI commands.
- Keep dry-run and planned metadata compatibility.
- Add tests using local stub/fake provider endpoints.
- Update README/docs in English, Russian, Chinese, and Kazakh.
- Module-size check stays under 200 lines per Rust/JS implementation file.
