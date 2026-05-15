# AgentHub Workspaces

Тілдер: [English](workspaces.en.md), [Русский](workspaces.ru.md), [中文](workspaces.zh.md), [Қазақша](workspaces.kk.md)

## Мақсаты

Phase 11 бір transaction manager non-code tasks орындай алатынын дәлелдейді. AgentHub git-worktree backed `code.git`, `content.git`, `data.git`, `infra.git`, `media.git`, `research.git` profiles қолдайды.

## ContentWorkspace

```yaml
workspace:
  type: content.git
  isolation: git_worktree

verify:
  profile: content_quality
```

`content_quality` configured commands орындайды, кейін `content/` ішіндегі markdown/text artifacts бар және бос емес екенін тексереді. Memory kind: `content_change`.

Іске қосу:

```bash
agenthub run examples/content-task.yaml
```

## DataWorkspace

```yaml
workspace:
  type: data.git
  isolation: git_worktree

verify:
  profile: data_quality
```

`data_quality` configured commands орындайды, кейін `data/` ішіндегі JSON artifacts валидтейді. Memory kind: `data_change`.

Іске қосу:

```bash
agenthub run examples/data-task.yaml
```

## InfraWorkspace

```yaml
workspace:
  type: infra.git
  isolation: git_worktree

verify:
  profile: infra_plan
```

`infra_plan` configured commands орындайды, кейін `infra/` ішіндегі infra artifacts валидтейді: бос емес YAML/YML/Terraform files және parseable YAML plans. Memory kind: `infra_change`.

Іске қосу:

```bash
agenthub run examples/infra-task.yaml
```

## MediaWorkspace

```yaml
workspace:
  type: media.git
  isolation: git_worktree

verify:
  profile: media_render
```

`media_render` configured commands орындайды, кейін `media/` ішіндегі media artifacts бар, бос емес және JSON/YAML manifests қолданылса valid екенін тексереді. Memory kind: `media_change`.

Іске қосу:

```bash
agenthub run examples/media-task.yaml
```

## ResearchWorkspace

```yaml
workspace:
  type: research.git
  isolation: git_worktree

verify:
  profile: research_report
```

`research_report` `research/` ішіндегі source collection, cited claims, research graph, final report және critic notes тексереді. Memory kind: `research_change`.

Іске қосу:

```bash
agenthub run examples/research-task.yaml
```

## Domain Memory Schemas

Tracked schemas:

- `.agent/schemas/content.yaml`
- `.agent/schemas/data.yaml`
- `.agent/schemas/infra.yaml`
- `.agent/schemas/media.yaml`
- `.agent/schemas/research.yaml`

Олар committed memory және reports үшін domain-specific memory object types мен fields анықтайды.
