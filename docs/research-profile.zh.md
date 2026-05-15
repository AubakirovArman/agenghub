# Research Profile

语言: [English](research-profile.en.md), [Русский](research-profile.ru.md), [中文](research-profile.zh.md), [Қазақша](research-profile.kk.md)

`research.git` 是 AgentHub 用于 cited research workflows 的 profile。它结合了现有 `swarm_research` topology、domain verifier 和 research memory schema。

## AgentSpec

```yaml
workspace:
  type: research.git
  isolation: git_worktree

topology:
  kind: swarm_research
  swarm_size: 2

verify:
  profile: research_report
```

运行示例：

```bash
agenthub run examples/research-task.yaml
```

## Required Artifacts

`research_report` 会验证 `research/` 下的文件：

- `sources.json`: 带 `id` fields 的 sources array；
- `claims.json`: 带 `citations` 的 claims array，citations 指向 source ids；
- `graph.json`: 包含 `nodes` 和 optional `edges` 的 research graph；
- `report.md`: final cited report；
- `critic.md`: critic 或 review notes。

## Memory

成功的 research transactions 会提升 `research_change` records。`.agent/schemas/research.yaml` 定义 sources、citations、claims、graph nodes/edges、critic notes 和 reports。
