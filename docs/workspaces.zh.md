# AgentHub Workspaces

语言: [English](workspaces.en.md), [Русский](workspaces.ru.md), [中文](workspaces.zh.md), [Қазақша](workspaces.kk.md)

## 目的

Phase 11 证明同一个 transaction manager 可以执行 non-code tasks。AgentHub 支持基于 git worktree 的 `code.git`、`content.git`、`data.git`、`infra.git`、`media.git` profiles。

## ContentWorkspace

```yaml
workspace:
  type: content.git
  isolation: git_worktree

verify:
  profile: content_quality
```

`content_quality` 先运行 configured commands，然后检查 `content/` 下存在非空 markdown/text artifacts。Memory kind: `content_change`。

运行：

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

`data_quality` 先运行 configured commands，然后验证 `data/` 下的 JSON artifacts。Memory kind: `data_change`。

运行：

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

`infra_plan` 先运行 configured commands，然后验证 `infra/` 下的 infra artifacts，包括非空 YAML/YML/Terraform files 和可解析 YAML plans。Memory kind: `infra_change`。

运行：

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

`media_render` 先运行 configured commands，然后检查 `media/` 下的 media artifacts 是否存在、非空，并在使用 JSON/YAML manifests 时验证其格式。Memory kind: `media_change`。

运行：

```bash
agenthub run examples/media-task.yaml
```

## Domain Memory Schemas

Tracked schemas 位于：

- `.agent/schemas/content.yaml`
- `.agent/schemas/data.yaml`
- `.agent/schemas/infra.yaml`
- `.agent/schemas/media.yaml`

它们定义 domain-specific memory object types 和 fields，用于 committed memory 和 reports。
