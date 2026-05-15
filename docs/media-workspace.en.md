# MediaWorkspace

Languages: [English](media-workspace.en.md), [Русский](media-workspace.ru.md), [中文](media-workspace.zh.md), [Қазақша](media-workspace.kk.md)

`media.git` is the AgentHub profile for prompt, script, voice, render, and video/audio asset workflows. It uses the same transaction kernel as code, content, data, and infra workspaces.

## AgentSpec

```yaml
workspace:
  type: media.git
  isolation: git_worktree

verify:
  profile: media_render
```

Run the example:

```bash
agenthub run examples/media-task.yaml
```

## Verification

`media_render` runs `verify.commands`, then checks `media/` artifacts:

- supported files: `md`, `txt`, `json`, `yaml`, `yml`, common image, audio, and video extensions;
- artifacts must be present and non-empty;
- JSON/YAML manifests must parse when present.

## Memory

Successful media transactions promote `media_change` records into `.agent/memory/committed.jsonl`. The default schema lives at `.agent/schemas/media.yaml` and includes scenes, shots, prompt templates, assets, voice tracks, render settings, video style, and platform requirements.
