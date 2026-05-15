# PRD v2 Task 03 — Resume, Retry, Resolve

Status: Todo

## Goal

Make blocked and failed transactions actionable instead of read-only historical reports.

## Acceptance

- `agenthub tx resolve <tx-id> --note ...` records a human resolution note.
- `agenthub tx resume <tx-id>` can continue supported blocked states.
- `agenthub tx retry <tx-id> --from <state>` creates a controlled retry plan.
- Resume/retry/resolve events are written to journal, WAL, and effect ledger.
- Failed external effects do not promote committed memory during resume/retry flows.
- Tests cover resolve metadata and at least one supported resume or retry path.
- README and docs are updated in English, Russian, Chinese, and Kazakh for user-facing behavior.
- Module-size check stays under 200 lines per Rust/JS implementation file.
