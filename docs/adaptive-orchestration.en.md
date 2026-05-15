# Adaptive Orchestration

Languages: [English](adaptive-orchestration.en.md), [Русский](adaptive-orchestration.ru.md), [中文](adaptive-orchestration.zh.md), [Қазақша](adaptive-orchestration.kk.md)

Adaptive orchestration is an opt-in routing layer. It classifies the task, estimates risk, chooses an effective topology, records the decision, and keeps a project-level scoreboard for later analytics.

## Enable

```yaml
topology:
  kind: single_executor
  routing:
    adaptive: true
```

If `adaptive` is absent or `false`, AgentHub keeps the configured topology exactly as written. The original file is still copied to `plan.yaml`; the runtime plan after adaptive selection is written to `effective_plan.yaml`.

## Classification

The classifier looks at task id, task type, title, workspace, verifier profile, allow-scope size, and command count.

| Task class | Typical topology |
|---|---|
| `simple_edit` | `single_executor` |
| `feature`, `refactor` | `manager_worker` |
| `bugfix`, `infra`, `unknown` | `planner_executor` |
| `research` | `swarm_research` |
| `content` | `generator_critic` |
| `high_risk` | `executor_reviewer_repair` |

High-risk keywords include auth, secrets, payments, security, and migrations. If adaptive chooses `executor_reviewer_repair` and no reviewer command exists, AgentHub inserts a no-op `true` reviewer command so the topology remains executable; explicit review commands should be provided for real review gates.

## Artifacts

- `.agent/tx/<tx-id>/adaptive.json`: classifier inputs, task class, risk, signals, original topology, selected topology, model label, and explanation.
- `.agent/tx/<tx-id>/effective_plan.yaml`: the AgentSpec after adaptive topology selection.
- `.agent/tx/<tx-id>/report.md`: `Adaptive Orchestration` section.
- `.agent/metrics/orchestration_scoreboard.json`: aggregate runs, success, repair, rollback, human-block, cost, and latency by task class/topology/model.

## Example

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

This feature task normally selects `manager_worker`; the report explains the reason and lists the classifier signals.
