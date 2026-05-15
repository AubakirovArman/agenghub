# Sandbox Levels

Тілдер: [English](sandbox-levels.en.md), [Русский](sandbox-levels.ru.md), [中文](sandbox-levels.zh.md), [Қазақша](sandbox-levels.kk.md)

AgentHub command execution алдында `execution.sandbox.level` тексереді және `sandbox.json` жазады.

## Levels

- `0`: worktree isolation, process groups, timeouts және command policy бар local controlled execution.
- `1`: тазартылған command environment, sandbox TMPDIR және `AGENTHUB_SANDBOX_LEVEL=1` бар local sandbox mode.
- `2`: strong isolation қажет; container, namespace, microVM немесе remote runner дайын болғанша transaction блокталады.
- `3`: enterprise runner қажет; enterprise isolated runner бапталғанша transaction блокталады.

## Example

```yaml
execution:
  sandbox:
    level: 1
  commands:
    - test "$AGENTHUB_SANDBOX_LEVEL" = 1
```

Іске қосу:

```bash
agenthub run examples/sandbox-level-task.yaml
```
