# Transaction Undo

`agenthub undo last` 使用 `git revert` 回滚最新的 committed AgentHub transaction。

```bash
agenthub undo last
agenthub undo tx-20260515123000-abcd1234
```

在本地 shell 中：

```text
agenthub:plan> undo
agenthub:plan> undo tx-20260515123000-abcd1234
```

1.0 之前 undo 有意保持有限：

- 只能 undo committed transactions；
- project 不能有 unrelated uncommitted changes；
- AgentHub 通过 `AgentHub <tx-id>:` commit subject 查找 commit；
- 写入 `.agent/tx/<tx-id>/undo.json`；
- 向原 transaction journal 追加 `UNDO_REVERTED`。

如果 `git revert` 发生冲突，Git 会停止并显示 conflict。先手动恢复 Git state，再运行新的 transactions。
