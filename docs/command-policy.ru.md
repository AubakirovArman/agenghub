# Command Policy

Языки: [English](command-policy.en.md), [Русский](command-policy.ru.md), [中文](command-policy.zh.md), [Қазақша](command-policy.kk.md)

AgentHub проверяет `.agent/policies/core.yaml` перед запуском AgentSpec commands.

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

Поведение:

- `safe`: разрешается и записывается в `command_policy.json`.
- `needs_approval`: разрешается только при `transaction.approval_required: true`; иначе transaction получает `BLOCKED_ON_HUMAN`.
- `restricted`: всегда отклоняется до выполнения command и записывается как failed transaction.
- unclassified commands записываются и разрешаются.

Patterns совпадают с точной command или prefix + пробел, поэтому `npm install left-pad` совпадает с `npm install`.

## Artifacts

Каждая transaction пишет `.agent/tx/<tx-id>/command_policy.json` с classifications и violations.
