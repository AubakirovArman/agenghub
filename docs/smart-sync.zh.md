# Smart Sync

语言: [English](smart-sync.en.md), [Русский](smart-sync.ru.md), [中文](smart-sync.zh.md), [Қазақша](smart-sync.kk.md)

Smart Sync 用 file-level overlap detection 取代旧的 HEAD-only block。当 project `HEAD` 在事务期间变化时，AgentHub 会比较 main changed files 和 transaction changed files。

## 决策

- `clean`: project `HEAD` 没有移动。
- `rebase_required`: project `HEAD` 已移动，但 file sets 不重叠。AgentHub 运行 `git rebase --autostash`，重新运行 diff guard 和 verifier，然后 commit。
- `blocked_overlap`: project `HEAD` 已移动，双方至少修改了一个相同 path。AgentHub 会 block on human，而不是 blind merge。

## Artifacts

```text
.agent/tx/<tx-id>/baseline.json
.agent/tx/<tx-id>/sync.json
.agent/tx/<tx-id>/report.md
```

`baseline.json` 保存 transaction `base_head`、来自 `scope.allow` 的 scoped file hashes，以及从 context maps 选择的 relevant file hashes。

`sync.json` 保存 base/current heads、main changed files、transaction changed files、overlaps，以及是否需要 verifier rerun。

示例:

```json
{
  "decision": "rebase_required",
  "overlapping_files": [],
  "verifier_rerun_required": true
}
```
