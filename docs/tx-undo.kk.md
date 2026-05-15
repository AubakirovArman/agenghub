# Transaction Undo

`agenthub undo last` соңғы committed AgentHub transaction-ды `git revert` арқылы қайтарады.

```bash
agenthub undo last
agenthub undo tx-20260515123000-abcd1234
```

Жергілікті shell ішінде:

```text
agenthub:plan> undo
agenthub:plan> undo tx-20260515123000-abcd1234
```

1.0 дейін undo әдейі шектеулі:

- тек committed transactions undo болады;
- project ішінде unrelated uncommitted changes болмауы керек;
- AgentHub commit-ті `AgentHub <tx-id>:` commit subject арқылы табады;
- `.agent/tx/<tx-id>/undo.json` жазады;
- original transaction journal ішіне `UNDO_REVERTED` қосады.

Егер `git revert` conflict берсе, Git тоқтап conflict көрсетеді. Жаңа transactions іске қоспай тұрып Git state-ті қолмен түзету керек.
