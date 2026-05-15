# Adaptive Orchestration

Языки: [English](adaptive-orchestration.en.md), [Русский](adaptive-orchestration.ru.md), [中文](adaptive-orchestration.zh.md), [Қазақша](adaptive-orchestration.kk.md)

Adaptive orchestration — opt-in слой routing. Он классифицирует task, оценивает risk, выбирает effective topology, записывает решение и ведёт project-level scoreboard для будущей analytics.

## Включение

```yaml
topology:
  kind: single_executor
  routing:
    adaptive: true
```

Если `adaptive` отсутствует или равен `false`, AgentHub сохраняет configured topology ровно как в YAML. Исходный файл копируется в `plan.yaml`; runtime plan после adaptive selection пишется в `effective_plan.yaml`.

## Classification

Classifier смотрит на task id, task type, title, workspace, verifier profile, размер allow-scope и command count.

| Task class | Typical topology |
|---|---|
| `simple_edit` | `single_executor` |
| `feature`, `refactor` | `manager_worker` |
| `bugfix`, `infra`, `unknown` | `planner_executor` |
| `research` | `swarm_research` |
| `content` | `generator_critic` |
| `high_risk` | `executor_reviewer_repair` |

High-risk keywords: auth, secrets, payments, security и migrations. Если adaptive выбирает `executor_reviewer_repair`, но reviewer command не задана, AgentHub добавляет no-op `true`, чтобы topology можно было выполнить; для реального reviewer gate лучше явно задавать review commands.

## Artifacts

- `.agent/tx/<tx-id>/adaptive.json`: classifier inputs, task class, risk, signals, original topology, selected topology, model label и explanation.
- `.agent/tx/<tx-id>/effective_plan.yaml`: AgentSpec после adaptive topology selection.
- `.agent/tx/<tx-id>/report.md`: секция `Adaptive Orchestration`.
- `.agent/metrics/orchestration_scoreboard.json`: aggregate runs, success, repair, rollback, human-block, cost и latency по task class/topology/model.

## Пример

```yaml
task:
  id: adaptive_feature_demo
  type: code.command
  title: Add adaptive feature output
topology:
  kind: single_executor
  routing:
    adaptive: true
workspace:
  type: code.git
  isolation: git_worktree
```

Такая feature task обычно выбирает `manager_worker`; report объясняет причину и показывает classifier signals.
