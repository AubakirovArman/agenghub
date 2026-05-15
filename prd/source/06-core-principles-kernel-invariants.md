# 6. Core Principles / Kernel Invariants

## 6.1 Главные законы AgentHub

### Law 1 — Atomicity

```text
No verified success — no commit.
```

Если задача не прошла verifier, её изменения не попадают в основную среду.

### Law 2 — Memory Consistency

```text
No successful verifier — no memory promotion.
```

Основная память проекта обновляется только после успешной проверки.

### Law 3 — Isolation

```text
Agent actions must run in isolated workspaces.
```

Агент не должен напрямую менять рабочую директорию без транзакционного контроля.

### Law 4 — Rollbackability

```text
Every effect must be rollbackable or explicitly declared non-rollbackable.
```

Для каждого эффекта нужен rollback handler или явное human approval.

### Law 5 — Failed Experience Durability

```text
Failed attempts are remembered separately from project truth.
```

Неудачные попытки не становятся фактами проекта, но сохраняются как опыт.

### Law 6 — No Blind Merge

```text
No transaction may merge without sync check.
```

Если main изменился во время работы агента, нужно проверить конфликты и заново прогнать verifier.

### Law 7 — Scope Enforcement

```text
Agent cannot edit outside declared scope.
```

Любое изменение вне scope требует approval или hard fail.

### Law 8 — Observability First

```text
Every transaction must be explainable after completion.
```

У каждой транзакции должны быть journal, report, traces, verifier logs и cost breakdown.

### Law 9 — Least Context

```text
Agent receives minimum sufficient context, not maximum available context.
```

### Law 10 — Domain via Plugins

```text
Core runtime is domain-agnostic. Domains are defined by workspace + skill + memory schema.
```

---

