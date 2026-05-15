# Sandbox Levels

语言: [English](sandbox-levels.en.md), [Русский](sandbox-levels.ru.md), [中文](sandbox-levels.zh.md), [Қазақша](sandbox-levels.kk.md)

AgentHub 会在 command execution 前检查 `execution.sandbox.level`，并写入 `sandbox.json`。

## Levels

- `0`: local controlled execution，包含 worktree isolation、process groups、timeouts 和 command policy。
- `1`: local sandbox mode，使用清理后的 command environment、sandbox TMPDIR 和 `AGENTHUB_SANDBOX_LEVEL=1`。
- `2`: remote runner mode；把 commands dispatch 到配置好的 remote runner，否则以 `remote_runner_required` 阻塞。
- `3`: enterprise runner mode；dispatch 到带有 `enterprise` 或 `isolated` label 的 remote runner，否则阻塞。

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

Remote dispatch 详情见 [Remote Runner Execution](remote-runner.zh.md)。
