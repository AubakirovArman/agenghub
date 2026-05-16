# V4.16 Live Run Watch

Status: done

Implemented:

- Added `live_run` as the shared interactive execution wrapper.
- Added transaction execution with caller-provided tx ids so watchers can start before the journal exists.
- Added cancellable `tx_watch` support for early transaction startup failures.
- `agenthub run` now shows live journal progress in interactive terminals.
- Shell task execution uses the same live journal progress path.
- Added `agenthub run --no-watch` for quiet scripted runs.
- README, product CLI docs, wiki source docs, changelog, and PRD tracker document the behavior in four languages.

Verification:

- `cargo test shell::`
- `cargo test tx_watch::tests`
- `cargo test handlers::run_commands::tests`
- `scripts/check-module-size.sh 200`
