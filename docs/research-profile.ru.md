# Research Profile

Языки: [English](research-profile.en.md), [Русский](research-profile.ru.md), [中文](research-profile.zh.md), [Қазақша](research-profile.kk.md)

`research.git` — профиль AgentHub для cited research workflows. Он объединяет существующую topology `swarm_research`, domain verifier и research memory schema.

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

Запуск примера:

```bash
agenthub run examples/research-task.yaml
```

## Required Artifacts

`research_report` валидирует файлы в `research/`:

- `sources.json`: array sources с полями `id`;
- `claims.json`: array claims с `citations`, которые указывают на source ids;
- `graph.json`: research graph с `nodes` и optional `edges`;
- `report.md`: final cited report;
- `critic.md`: critic или review notes.

## Memory

Успешные research transactions продвигают `research_change` records. `.agent/schemas/research.yaml` описывает sources, citations, claims, graph nodes/edges, critic notes и reports.
