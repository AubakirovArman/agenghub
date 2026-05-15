# Transaction Undo

`agenthub undo last` reverts the latest committed AgentHub transaction with `git revert`.

```bash
agenthub undo last
agenthub undo tx-20260515123000-abcd1234
```

Inside the local shell:

```text
agenthub:plan> undo
agenthub:plan> undo tx-20260515123000-abcd1234
```

Undo is intentionally limited before 1.0:

- only committed transactions can be undone;
- the project must have no unrelated uncommitted changes;
- AgentHub finds the commit by the `AgentHub <tx-id>:` commit subject;
- it writes `.agent/tx/<tx-id>/undo.json`;
- it appends `UNDO_REVERTED` to the original transaction journal.

If the revert conflicts, Git stops and reports the conflict. Resolve the Git state manually before running more transactions.
