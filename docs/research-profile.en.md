# Research Profile

Languages: [English](research-profile.en.md), [Русский](research-profile.ru.md), [中文](research-profile.zh.md), [Қазақша](research-profile.kk.md)

`research.git` is the AgentHub profile for cited research workflows. It combines the existing `swarm_research` topology with a domain verifier and research memory schema.

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

Run the example:

```bash
agenthub run examples/research-task.yaml
```

## Required Artifacts

`research_report` validates files under `research/`:

- `sources.json`: array of sources with `id` fields;
- `claims.json`: array of claims with `citations` pointing to source ids;
- `graph.json`: research graph with `nodes` and optional `edges`;
- `report.md`: final cited report;
- `critic.md`: critic or review notes.

## Memory

Successful research transactions promote `research_change` records. `.agent/schemas/research.yaml` defines sources, citations, claims, graph nodes/edges, critic notes, and reports.
