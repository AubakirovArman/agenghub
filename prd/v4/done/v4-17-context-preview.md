# V4.17 Shell Context Preview

Status: done

Implemented:

- Added `/context` slash command.
- Context preview prints project path, default provider, current chat title, recent user messages, selected transaction report, memory counts, and mention hints.
- Slash completion and `/help` include `/context`.
- README, product CLI docs, wiki source docs, changelog, and PRD tracker document the behavior in four languages.

Verification:

- `cargo test shell::`
- `scripts/check-module-size.sh 200`
