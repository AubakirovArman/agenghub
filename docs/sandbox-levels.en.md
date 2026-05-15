# Sandbox Levels

Languages: [English](sandbox-levels.en.md), [Русский](sandbox-levels.ru.md), [中文](sandbox-levels.zh.md), [Қазақша](sandbox-levels.kk.md)

AgentHub evaluates `execution.sandbox.level` before command execution and writes `sandbox.json`.

## Levels

- `0`: local controlled execution with worktree isolation, process groups, timeouts, and command policy.
- `1`: local sandbox mode with sanitized command environment, sandbox TMPDIR, and `AGENTHUB_SANDBOX_LEVEL=1`.
- `2`: remote runner mode; dispatches commands to a configured remote runner or blocks with `remote_runner_required`.
- `3`: enterprise runner mode; dispatches to a remote runner labeled `enterprise` or `isolated`, otherwise blocks.

## Example

```yaml
execution:
  sandbox:
    level: 1
  commands:
    - test "$AGENTHUB_SANDBOX_LEVEL" = 1
```

Run:

```bash
agenthub run examples/sandbox-level-task.yaml
```

Remote dispatch details are in [Remote Runner Execution](remote-runner.en.md).
