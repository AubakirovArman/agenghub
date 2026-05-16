# V4.18 Context Mentions

Done for the chat-first context input layer.

## Scope

- Added `@tx`, `@tx:latest`, `@tx:<id>`, and `@last-tx` transaction context summaries.
- Added `@memory` and `@memory:<query>` project-memory summaries with failed-attempt warnings.
- Kept existing `@file`, `@folder`, and `@last` behavior compatible.
- Updated `/context` mention hints.
- Split shell dispatch and mention code to preserve the near-200-line module rule.
- Updated README, interactive shell docs, and wiki seed in English, Russian, Chinese, and Kazakh.

## Checks

- `cargo fmt -- --check`
- `cargo test shell::mention_summary_tests --quiet`
- `cargo test shell::commands --quiet`
- `cargo test shell:: --quiet`
- `scripts/check-module-size.sh 200`
- `scripts/release-readiness.sh`
