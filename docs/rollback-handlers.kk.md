# Rollback Handlers

Тілдер: [English](rollback-handlers.en.md), [Русский](rollback-handlers.ru.md), [中文](rollback-handlers.zh.md), [Қазақша](rollback-handlers.kk.md)

AgentHub v2 failed transactions үшін concrete rollback handler choices жазады. Transaction rollback болғанда, AgentHub `.agent/tx/<tx-id>/rollback.json` жазады және file rollback statuses мәндерін `effects.jsonl` ішінде қайталайды.

## Handler таңдау

- `file_restore`: isolated worktree cleanup path арқылы default file rollback.
- `package_manifest_restore`: `package.json`, lockfiles, `Cargo.toml` және `Cargo.lock`.
- `terraform_state_restore`: `.tfstate` және `.tfstate.backup` files.
- `manual_approval_required`: `.env` files және environment/secret-like files.

## Қолдану

```bash
agenthub tx effects tx-...
cat .agent/tx/tx-.../rollback.json
```

`rollback.json` — structured rollback artifact. Ол әр changed path, selected handler, rollback status және handler extra care талап етсе reason көрсетеді.
