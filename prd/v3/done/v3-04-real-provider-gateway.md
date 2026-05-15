# PRD v3 Task 04 - Real Provider Gateway

Status: Done

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

## Completed

- Added `HttpProvider` for OpenAI-compatible `http://` endpoints at `/v1/chat/completions`.
- Added real `CliProvider` execution path with prompt-file rendering, command template execution, stdout/stderr capture, redaction, and transcript JSONL.
- Added `complete_with_retry` for provider retry/backoff and optional attempt transcript records.
- Added `openai-http` provider status/test integration through `agenthub providers`.
- Preserved planned metadata compatibility by keeping prompt optional in `LlmRequest`.
- Added local stub/fake provider tests for HTTP, CLI, and retry behavior.
- Updated README, changelog, Product CLI docs, and LLM Gateway docs in four languages.

## Evidence

- Implementation commit: `df405f6 Add real provider gateway execution`
- HTTP local stub test validates OpenAI-compatible response parsing.
- CLI provider test validates real command invocation and transcript capture.
- Retry test validates retry after a temporary provider failure.
- `agenthub providers status` now includes `openai-http` and actionable endpoint setup.

## Validation

- `cargo fmt -- --check`
- `scripts/check-module-size.sh 200`
- `cargo clippy --locked -- -D warnings`
- `cargo test --locked provider`
- `cargo test --locked`
- `npm --prefix editors/vscode run check`
- `git diff --check`
- `target/debug/agenthub providers status`
- `scripts/smoke-test.sh`

## Notes

- The HTTP provider intentionally targets local/dev OpenAI-compatible `http://` endpoints. Direct HTTPS SaaS providers remain a later hardening step.
