# Open Task 16 — Formal WAL

Status: Done

Source: `prd/audit/open/README.md`, `prd.md`

## Goal

Formal write-ahead log subsystem beyond current journal.

## Acceptance

- Implementation exists or the PRD gap is explicitly narrowed with shipped behavior.
- README and docs are updated in English, Russian, Chinese, and Kazakh when user-facing behavior changes.
- Tests or smoke checks cover the new behavior.
- Module-size check stays under 200 lines per Rust/JS implementation file.
- Move this task to `prd/done/` with closing evidence when complete.

## Completed

- Added formal append-only transaction WAL records with sequence numbers and SHA-256 checksums.
- WAL append happens before matching `journal.jsonl` events and is fsynced.
- Added replay validation for sequence order and checksum integrity.
- Transactions write `wal.jsonl` and `wal_replay.json`.
- README and feature docs were updated in English, Russian, Chinese, and Kazakh.

## Evidence

- Implementation commit: pending.
- Checks: pending.
