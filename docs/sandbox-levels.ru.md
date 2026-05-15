# Sandbox Levels

Языки: [English](sandbox-levels.en.md), [Русский](sandbox-levels.ru.md), [中文](sandbox-levels.zh.md), [Қазақша](sandbox-levels.kk.md)

AgentHub проверяет `execution.sandbox.level` перед command execution и пишет `sandbox.json`.

## Levels

- `0`: local controlled execution с worktree isolation, process groups, timeouts и command policy.
- `1`: local sandbox mode с очищенным command environment, sandbox TMPDIR и `AGENTHUB_SANDBOX_LEVEL=1`.
- `2`: remote runner mode; dispatch commands в настроенный remote runner или блокировка `remote_runner_required`.
- `3`: enterprise runner mode; dispatch в remote runner с label `enterprise` или `isolated`, иначе блокировка.

## Example

```yaml
execution:
  sandbox:
    level: 1
  commands:
    - test "$AGENTHUB_SANDBOX_LEVEL" = 1
```

Запуск:

```bash
agenthub run examples/sandbox-level-task.yaml
```

Подробности remote dispatch: [Remote Runner Execution](remote-runner.ru.md).
