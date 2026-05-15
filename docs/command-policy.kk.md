# Command Policy

Тілдер: [English](command-policy.en.md), [Русский](command-policy.ru.md), [中文](command-policy.zh.md), [Қазақша](command-policy.kk.md)

AgentHub AgentSpec commands іске қоспай тұрып `.agent/policies/core.yaml` тексереді.

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

Мінез-құлық:

- `safe`: рұқсат етіледі және `command_policy.json` ішіне жазылады.
- `needs_approval`: тек `transaction.approval_required: true` болса рұқсат; әйтпесе transaction `BLOCKED_ON_HUMAN` болады.
- `restricted`: command execution алдында әрқашан тоқтатылады және failed transaction ретінде жазылады.
- unclassified commands жазылады және рұқсат етіледі.

Patterns толық command немесе бос орыннан кейінгі prefix ретінде сәйкеседі, сондықтан `npm install left-pad` `npm install` pattern-іне сәйкеседі.

## Artifacts

Әр transaction `.agent/tx/<tx-id>/command_policy.json` жазады; ішінде classifications және violations бар.
