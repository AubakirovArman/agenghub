# Rollback Handlers

Языки: [English](rollback-handlers.en.md), [Русский](rollback-handlers.ru.md), [中文](rollback-handlers.zh.md), [Қазақша](rollback-handlers.kk.md)

AgentHub v2 записывает concrete rollback handler choices для failed transactions. Когда транзакция откатывается, AgentHub пишет `.agent/tx/<tx-id>/rollback.json` и дублирует file rollback statuses в `effects.jsonl`.

## Выбор handler

- `file_restore`: стандартный file rollback через isolated worktree cleanup path.
- `package_manifest_restore`: `package.json`, lockfiles, `Cargo.toml` и `Cargo.lock`.
- `terraform_state_restore`: `.tfstate` и `.tfstate.backup` files.
- `manual_approval_required`: `.env` files и environment/secret-like files.

## Использование

```bash
agenthub tx effects tx-...
cat .agent/tx/tx-.../rollback.json
```

`rollback.json` — structured rollback artifact. Он перечисляет каждый changed path, выбранный handler, rollback status и reason, если handler требует особой осторожности.
