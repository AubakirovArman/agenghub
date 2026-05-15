# MediaWorkspace

语言: [English](media-workspace.en.md), [Русский](media-workspace.ru.md), [中文](media-workspace.zh.md), [Қазақша](media-workspace.kk.md)

`media.git` 是 AgentHub 用于 prompt、script、voice、render 和 video/audio asset workflows 的 profile。它使用与 code、content、data、infra workspaces 相同的 transaction kernel。

## AgentSpec

```yaml
workspace:
  type: media.git
  isolation: git_worktree

verify:
  profile: media_render
```

运行示例：

```bash
agenthub run examples/media-task.yaml
```

## Verification

`media_render` 先运行 `verify.commands`，然后检查 `media/` artifacts：

- 支持文件：`md`、`txt`、`json`、`yaml`、`yml`，以及常见 image、audio、video extensions；
- artifacts 必须存在且非空；
- 如果存在 JSON/YAML manifests，则必须能解析。

## Memory

成功的 media transactions 会把 `media_change` records 提升到 `.agent/memory/committed.jsonl`。默认 schema 位于 `.agent/schemas/media.yaml`，包含 scenes、shots、prompt templates、assets、voice tracks、render settings、video style 和 platform requirements。
