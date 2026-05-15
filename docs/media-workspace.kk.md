# MediaWorkspace

Тілдер: [English](media-workspace.en.md), [Русский](media-workspace.ru.md), [中文](media-workspace.zh.md), [Қазақша](media-workspace.kk.md)

`media.git` — prompt, script, voice, render және video/audio asset workflows үшін AgentHub profile. Ол code, content, data және infra workspaces қолданатын transaction kernel арқылы жұмыс істейді.

## AgentSpec

```yaml
workspace:
  type: media.git
  isolation: git_worktree

verify:
  profile: media_render
```

Мысалды іске қосу:

```bash
agenthub run examples/media-task.yaml
```

## Verification

`media_render` алдымен `verify.commands` орындайды, содан кейін `media/` artifacts тексереді:

- supported files: `md`, `txt`, `json`, `yaml`, `yml`, common image, audio және video extensions;
- artifacts бар және бос емес болуы керек;
- JSON/YAML manifests болса, олар parse болуы керек.

## Memory

Сәтті media transactions `.agent/memory/committed.jsonl` ішіне `media_change` records promote жасайды. Default schema `.agent/schemas/media.yaml` ішінде орналасқан және scenes, shots, prompt templates, assets, voice tracks, render settings, video style және platform requirements қамтиды.
