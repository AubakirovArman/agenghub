# Command Policy

Languages: [English](command-policy.en.md), [Русский](command-policy.ru.md), [中文](command-policy.zh.md), [Қазақша](command-policy.kk.md)

AgentHub enforces `.agent/policies/core.yaml` before running AgentSpec commands.

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

Behavior:

- `safe`: allowed and recorded in `command_policy.json`.
- `needs_approval`: allowed only when `transaction.approval_required: true`; otherwise the transaction becomes `BLOCKED_ON_HUMAN`.
- `restricted`: always rejected before command execution and recorded as a failed transaction.
- unclassified commands are recorded and allowed.

Patterns match exact commands or prefixes followed by a space, so `npm install left-pad` matches `npm install`.

## Artifacts

Each transaction writes `.agent/tx/<tx-id>/command_policy.json` with classifications and violations.
