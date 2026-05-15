# Adaptive Orchestration

Тілдер: [English](adaptive-orchestration.en.md), [Русский](adaptive-orchestration.ru.md), [中文](adaptive-orchestration.zh.md), [Қазақша](adaptive-orchestration.kk.md)

Adaptive orchestration — opt-in routing layer. Ол task классификациясын жасайды, risk бағалайды, effective topology таңдайды, шешімді жазады және кейінгі analytics үшін project-level scoreboard жүргізеді.

## Қосу

```yaml
topology:
  kind: single_executor
  routing:
    adaptive: true
```

Егер `adaptive` жоқ немесе `false` болса, AgentHub YAML ішіндегі configured topology мәнін өзгеріссіз қолданады. Бастапқы файл `plan.yaml` болып көшіріледі; adaptive selection кейінгі runtime plan `effective_plan.yaml` ішіне жазылады.

## Classification

Classifier task id, task type, title, workspace, verifier profile, allow-scope size және command count қарайды.

| Task class | Typical topology |
|---|---|
| `simple_edit` | `single_executor` |
| `feature`, `refactor` | `manager_worker` |
| `bugfix`, `infra`, `unknown` | `planner_executor` |
| `research` | `swarm_research` |
| `content` | `generator_critic` |
| `high_risk` | `executor_reviewer_repair` |

High-risk keywords: auth, secrets, payments, security және migrations. Егер adaptive `executor_reviewer_repair` таңдаса, бірақ reviewer command жоқ болса, AgentHub topology executable болуы үшін no-op `true` қосады; нақты reviewer gate үшін review commands explicit берілгені дұрыс.

## Artifacts

- `.agent/tx/<tx-id>/adaptive.json`: classifier inputs, task class, risk, signals, original topology, selected topology, model label және explanation.
- `.agent/tx/<tx-id>/effective_plan.yaml`: adaptive topology selection кейінгі AgentSpec.
- `.agent/tx/<tx-id>/report.md`: `Adaptive Orchestration` section.
- `.agent/metrics/orchestration_scoreboard.json`: task class/topology/model бойынша aggregate runs, success, repair, rollback, human-block, cost және latency.

## Мысал

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

Мұндай feature task әдетте `manager_worker` таңдайды; report себебін түсіндіреді және classifier signals көрсетеді.
