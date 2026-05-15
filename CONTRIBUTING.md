# Contributing

AgentHub is currently moving toward an installable local developer preview. Keep changes scoped, transactional, and documented.

By submitting a contribution, you agree that it is licensed under Apache License 2.0.

## Local Checks

Run the same checks expected by CI:

```bash
cargo fmt -- --check
cargo build --locked
cargo clippy --locked -- -D warnings
cargo test --locked
scripts/check-module-size.sh 200
npm --prefix editors/vscode run check
scripts/smoke-test.sh
```

## Code Style

- Prefer existing modules and patterns before adding abstractions.
- Keep Rust source files near or below 200 lines when practical.
- Add focused tests for changed behavior.
- Update README/docs in English, Russian, Chinese, and Kazakh when user-facing behavior changes.

## Pull Requests

Include the problem, the implementation summary, verification commands, and any remaining risks. Do not include secrets, private traces, or generated transaction artifacts unless they are required test fixtures.
