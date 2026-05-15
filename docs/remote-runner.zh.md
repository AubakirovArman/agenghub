# Remote Runner Execution

语言: [English](remote-runner.en.md), [Русский](remote-runner.ru.md), [中文](remote-runner.zh.md), [Қазақша](remote-runner.kk.md)

当 `execution.sandbox.level` 为 `2` 或更高，并且配置了 enterprise remote runner 时，Remote runner execution 会调度 external agent adapter CLI calls、`execution.commands`、repair commands、review commands 和 verifier commands。

## Policy

```yaml
enterprise:
  runners:
    default: local
    remote:
      - id: strong-runner
        endpoint: ssh://runner.internal/workspaces/project
        labels:
          - strong-isolation
```

支持的 endpoints：

- `local://name`: 用于 local integration tests 和 single-host deployments 的 dispatch path。
- `ssh://host/path`: 运行 `ssh host 'cd path && sh -lc <command>'`。

## AgentSpec

```yaml
execution:
  sandbox:
    level: 2
  commands:
    - cargo test
```

结果会收集到 `adapter_invocation_*.json`、`execution.json`、`verifier.json`、`review.json` 或 `repair.json`，其中包含 `remote: true` 和 runner id。`sandbox.json` 会记录选中的 runner。
