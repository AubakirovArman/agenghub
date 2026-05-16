# V4.12 Chat-First Shell

## Status

Done for the local preview shell foundation.

## Completed

- Made `agenthub` without a subcommand the primary chat-first product surface.
- Added first-run onboarding that can prepare Git, initialize `.agent`, suggest an available provider, and restore the latest chat.
- Added persistent shell history and slash-command completion.
- Changed plain text in the shell to create a draft plan, show inline approval details, execute after approval, and print next actions.
- Added input prefixes:
  - `/` for AgentHub commands;
  - `@` for explicit file, folder, or latest-report context;
  - `!` for policy-checked shell commands with logs;
  - `#` for typed project memory notes.
- Added `/diff` and `/logs` shell commands backed by `agenthub tx diff` and `agenthub tx logs`.
- Fixed natural requests containing route-like text such as `/courses` so they are treated as requests, not filesystem paths.
- Updated README, shell docs, and product CLI docs in English, Russian, Chinese, and Kazakh.

## Evidence

- `cargo test shell::commands::tests`
- `cargo test handlers::run_commands::tests`
- Non-TTY shell smoke: `/status` and `/exit` initialize a temporary project and enter chat-first mode.

## Remaining V5 Depth

- True live progress streaming inside the shell should hook into transaction journal events.
- Provider profiles should become first-class named configurations.
- A local `agenthub serve` dashboard can make the browser view live instead of static-refresh only.
