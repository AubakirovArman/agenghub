# PRD v2 Task 04 — Smart Sync

Status: Done

## Goal

Replace the current coarse HEAD-only sync block with file-level overlap detection and a safe rebase path for independent changes.

## Acceptance

- Transaction baseline captures relevant file hashes.
- Sync check distinguishes independent main changes from overlapping transaction changes.
- Overlap blocks on human with structured report data.
- Independent changes can rebase transaction branch onto current main.
- Diff guard and verifier run again after rebase before commit.
- Report shows the sync decision and rerun verifier result.
- Tests cover independent rebase and overlapping block behavior.
- README and docs are updated in English, Russian, Chinese, and Kazakh for user-facing behavior.
- Module-size check stays under 200 lines per Rust/JS implementation file.

## Completed

- Added `SmartSyncDecision` and `.agent/tx/<tx-id>/sync.json` for structured sync decisions.
- Added `.agent/tx/<tx-id>/baseline.json` with `base_head`, scoped file hashes, and relevant context-map file hashes.
- Added file-level main/transaction changed-file comparison with overlap blocking.
- Added safe rebase for independent main changes through `git rebase --autostash`.
- Rerun diff guard and verifier after smart-sync rebase before commit.
- Added sync decision output to transaction reports.
- Added integration tests for independent rebase and overlapping block behavior.
- Updated README and smart-sync docs in English, Russian, Chinese, and Kazakh.

## Evidence

- Implementation commit: pending.
- Checks: pending.
