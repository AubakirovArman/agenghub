# Research Profile

Тілдер: [English](research-profile.en.md), [Русский](research-profile.ru.md), [中文](research-profile.zh.md), [Қазақша](research-profile.kk.md)

`research.git` — cited research workflows үшін AgentHub profile. Ол бар `swarm_research` topology, domain verifier және research memory schema біріктіреді.

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

Мысалды іске қосу:

```bash
agenthub run examples/research-task.yaml
```

## Required Artifacts

`research_report` `research/` ішіндегі файлдарды validates:

- `sources.json`: `id` fields бар sources array;
- `claims.json`: source ids көрсететін `citations` бар claims array;
- `graph.json`: `nodes` және optional `edges` бар research graph;
- `report.md`: final cited report;
- `critic.md`: critic немесе review notes.

## Memory

Сәтті research transactions `research_change` records promote жасайды. `.agent/schemas/research.yaml` sources, citations, claims, graph nodes/edges, critic notes және reports сипаттайды.
