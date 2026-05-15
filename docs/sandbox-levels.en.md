# Sandbox Levels

Languages: [English](sandbox-levels.en.md), [Русский](sandbox-levels.ru.md), [中文](sandbox-levels.zh.md), [Қазақша](sandbox-levels.kk.md)

AgentHub evaluates `execution.sandbox.level` before command execution and writes `sandbox.json`.

## Levels

- `0`: local controlled execution with worktree isolation, process groups, timeouts, and command policy.
- `1`: local sandbox mode with sanitized command environment, sandbox TMPDIR, and `AGENTHUB_SANDBOX_LEVEL=1`.
- `2`: strong isolation required; transaction is blocked until a container, namespace, microVM, or remote runner is available.
- `3`: enterprise runner required; transaction is blocked until an enterprise isolated runner is configured.

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
