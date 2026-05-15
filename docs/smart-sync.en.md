# Smart Sync

Languages: [English](smart-sync.en.md), [Русский](smart-sync.ru.md), [中文](smart-sync.zh.md), [Қазақша](smart-sync.kk.md)

Smart Sync replaces the old HEAD-only block with file-level overlap detection. When project `HEAD` changes during a transaction, AgentHub compares files changed on main with files changed by the transaction.

## Decisions

- `clean`: project `HEAD` did not move.
- `rebase_required`: project `HEAD` moved, but file sets do not overlap. AgentHub runs `git rebase --autostash`, reruns diff guard and verifier, then commits.
- `blocked_overlap`: project `HEAD` moved and both sides changed at least one same path. AgentHub blocks on human instead of merging blindly.

## Artifacts

```text
.agent/tx/<tx-id>/baseline.json
.agent/tx/<tx-id>/sync.json
.agent/tx/<tx-id>/report.md
```

`baseline.json` stores the transaction `base_head`, scoped file hashes from `scope.allow`, and relevant file hashes selected from context maps.

`sync.json` stores base/current heads, main changed files, transaction changed files, overlaps, and whether verifier rerun was required.

Example:

```json
{
  "decision": "rebase_required",
  "overlapping_files": [],
  "verifier_rerun_required": true
}
```
