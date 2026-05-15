# Command Policy

语言: [English](command-policy.en.md), [Русский](command-policy.ru.md), [中文](command-policy.zh.md), [Қазақша](command-policy.kk.md)

AgentHub 会在运行 AgentSpec commands 前检查 `.agent/policies/core.yaml`。

## Policy Lists

```yaml
commands:
  safe:
    - cargo test
    - npm test
  needs_approval:
    - npm install
    - docker compose up
  restricted:
    - rm -rf
    - sudo
```

行为：

- `safe`: 允许执行，并记录到 `command_policy.json`。
- `needs_approval`: 只有 `transaction.approval_required: true` 时允许；否则 transaction 变为 `BLOCKED_ON_HUMAN`。
- `restricted`: 总是在 command execution 前拒绝，并记录为 failed transaction。
- unclassified commands 会被记录并允许。

Patterns 匹配完整 command 或后接空格的 prefix，所以 `npm install left-pad` 会匹配 `npm install`。

## Artifacts

每个 transaction 都会写入 `.agent/tx/<tx-id>/command_policy.json`，其中包含 classifications 和 violations。
