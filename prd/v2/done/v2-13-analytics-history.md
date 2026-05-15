# PRD v2 Task 13 — Analytics History

Status: Done

## Goal

Move from per-transaction reports to persisted trend intelligence that can be viewed locally and exported to external analytics tools.

## Acceptance

- Persist metrics history across transactions under `.agent/metrics/`.
- Record success rate, rollback rate, repair rate, human-block frequency, average time to commit, and dangerous diff rate.
- Record model, topology, verifier, skill, and task-type metrics when those artifacts exist.
- Add JSONL and CSV exports for analytics history.
- Make dashboard/report output trend-ready instead of only showing the latest transaction.
- Keep metrics append-only or explicitly snapshot-versioned so history survives process restarts.
- Add tests for metric recording, trend aggregation, export files, and missing-artifact compatibility.
- README and docs are updated in English, Russian, Chinese, and Kazakh for user-facing behavior.
- Module-size check stays under 200 lines per Rust/JS implementation file.

## Completed

- Added append-only `.agent/metrics/analytics_history.jsonl` records for completed transaction end states.
- Added `.agent/metrics/analytics_summary.json` with success, rollback, repair, human-block, dangerous-diff, and average time-to-commit metrics.
- Added `.agent/metrics/analytics_history.csv` export for spreadsheet and BI consumption.
- Added grouped metrics by task type, topology, model, verifier profile, and skill when those artifacts exist.
- Added transaction integration so successful, rolled-back, repaired, and human-blocked runs contribute to persisted history.
- Added dashboard `metrics.history` payload and Metrics panel rendering for history rates.
- Added Analytics History docs in English, Russian, Chinese, and Kazakh and updated README/metrics docs.

## Evidence

- Implementation commit: `db4b596 Complete analytics history task`
- `cargo fmt -- --check`
- `scripts/check-module-size.sh 200`
- `git diff --check`
- `cargo test analytics`
- `cargo test web_dashboard::tests`
- `cargo test successful_transaction_commits_and_promotes_memory`
- `cargo clippy -- -D warnings`
- `cargo test`
- `npm run check` in `editors/vscode`
