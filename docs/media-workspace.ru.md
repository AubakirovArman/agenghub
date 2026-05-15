# MediaWorkspace

Языки: [English](media-workspace.en.md), [Русский](media-workspace.ru.md), [中文](media-workspace.zh.md), [Қазақша](media-workspace.kk.md)

`media.git` — профиль AgentHub для prompt, script, voice, render и video/audio asset workflows. Он использует тот же transaction kernel, что code, content, data и infra workspaces.

## AgentSpec

```yaml
workspace:
  type: media.git
  isolation: git_worktree

verify:
  profile: media_render
```

Запуск примера:

```bash
agenthub run examples/media-task.yaml
```

## Verification

`media_render` запускает `verify.commands`, затем проверяет artifacts в `media/`:

- поддерживаемые файлы: `md`, `txt`, `json`, `yaml`, `yml`, common image, audio и video extensions;
- artifacts должны существовать и быть непустыми;
- JSON/YAML manifests должны парситься, если они есть.

## Memory

Успешные media transactions продвигают `media_change` records в `.agent/memory/committed.jsonl`. Default schema находится в `.agent/schemas/media.yaml` и включает scenes, shots, prompt templates, assets, voice tracks, render settings, video style и platform requirements.
