# V4.19 Inline Approval Controls

Done for the shell plan approval UX.

## Scope

- Approval prompts now show scope allow/deny, execution and verifier commands, and a risk summary.
- Added `diff` approval action to show the planned scope before execution.
- Added `details` action for full AgentSpec YAML.
- Added `edit` action backed by `$VISUAL` or `$EDITOR`, with AgentSpec revalidation after editing.
- Kept non-TTY behavior script-friendly: approval automatically proceeds as before.
- Updated README, interactive shell docs, and wiki seed in English, Russian, Chinese, and Kazakh.

## Checks

- `cargo fmt -- --check`
- `cargo test shell:: --quiet`
- `scripts/check-module-size.sh 200`
