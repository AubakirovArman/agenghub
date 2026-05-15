# Sandbox Levels

语言: [English](sandbox-levels.en.md), [Русский](sandbox-levels.ru.md), [中文](sandbox-levels.zh.md), [Қазақша](sandbox-levels.kk.md)

AgentHub 会在 command execution 前检查 `execution.sandbox.level`，并写入 `sandbox.json`。

## Levels

- `0`: local controlled execution，包含 worktree isolation、process groups、timeouts 和 command policy。
- `1`: local sandbox mode，使用清理后的 command environment、sandbox TMPDIR 和 `AGENTHUB_SANDBOX_LEVEL=1`。
- `2`: 需要 strong isolation；transaction 会阻塞，直到 container、namespace、microVM 或 remote runner 可用。
- `3`: 需要 enterprise runner；transaction 会阻塞，直到配置 enterprise isolated runner。

## Example

```yaml
execution:
  sandbox:
    level: 1
  commands:
    - test "$AGENTHUB_SANDBOX_LEVEL" = 1
```

运行：

```bash
agenthub run examples/sandbox-level-task.yaml
```
