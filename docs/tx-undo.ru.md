# Undo транзакции

`agenthub undo last` откатывает последнюю committed AgentHub transaction через `git revert`.

```bash
agenthub undo last
agenthub undo tx-20260515123000-abcd1234
```

Внутри локальной оболочки:

```text
agenthub:plan> undo
agenthub:plan> undo tx-20260515123000-abcd1234
```

До 1.0 undo намеренно ограничен:

- откатывать можно только committed transactions;
- в проекте не должно быть unrelated uncommitted changes;
- AgentHub ищет commit по subject `AgentHub <tx-id>:`;
- пишет `.agent/tx/<tx-id>/undo.json`;
- добавляет `UNDO_REVERTED` в journal исходной transaction.

Если `git revert` получает конфликт, Git останавливается и показывает conflict. Сначала нужно вручную привести Git state в порядок, потом запускать новые transactions.
