# Adaptive Orchestration

语言: [English](adaptive-orchestration.en.md), [Русский](adaptive-orchestration.ru.md), [中文](adaptive-orchestration.zh.md), [Қазақша](adaptive-orchestration.kk.md)

Adaptive orchestration 是一个 opt-in routing 层。它会分类 task、估算 risk、选择 effective topology、记录决策，并维护 project-level scoreboard，供后续 analytics 使用。

## 启用

```yaml
topology:
  kind: single_executor
  routing:
    adaptive: true
```

如果没有 `adaptive`，或它是 `false`，AgentHub 会完全保留 YAML 中配置的 topology。原始文件仍复制为 `plan.yaml`；adaptive selection 后的 runtime plan 会写入 `effective_plan.yaml`。

## Classification

Classifier 会查看 task id、task type、title、workspace、verifier profile、allow-scope 大小和 command count。

| Task class | Typical topology |
|---|---|
| `simple_edit` | `single_executor` |
| `feature`, `refactor` | `manager_worker` |
| `bugfix`, `infra`, `unknown` | `planner_executor` |
| `research` | `swarm_research` |
| `content` | `generator_critic` |
| `high_risk` | `executor_reviewer_repair` |

High-risk keywords 包括 auth、secrets、payments、security 和 migrations。如果 adaptive 选择 `executor_reviewer_repair`，但没有 reviewer command，AgentHub 会插入 no-op `true`，保证 topology 可执行；真实 reviewer gate 应该显式提供 review commands。

## Artifacts

- `.agent/tx/<tx-id>/adaptive.json`: classifier inputs、task class、risk、signals、original topology、selected topology、model label 和 explanation。
- `.agent/tx/<tx-id>/effective_plan.yaml`: adaptive topology selection 后的 AgentSpec。
- `.agent/tx/<tx-id>/report.md`: `Adaptive Orchestration` section。
- `.agent/metrics/orchestration_scoreboard.json`: 按 task class/topology/model 聚合 runs、success、repair、rollback、human-block、cost 和 latency。

## 示例

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

这个 feature task 通常会选择 `manager_worker`；report 会解释原因并列出 classifier signals。
