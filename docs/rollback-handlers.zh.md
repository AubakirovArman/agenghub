# Rollback Handlers

语言: [English](rollback-handlers.en.md), [Русский](rollback-handlers.ru.md), [中文](rollback-handlers.zh.md), [Қазақша](rollback-handlers.kk.md)

AgentHub v2 会为 failed transactions 记录 concrete rollback handler choices。事务回滚时，AgentHub 写入 `.agent/tx/<tx-id>/rollback.json`，并在 `effects.jsonl` 中同步 file rollback statuses。

## Handler 选择

- `file_restore`: 默认 file rollback，通过 isolated worktree cleanup path 完成。
- `package_manifest_restore`: `package.json`、lockfiles、`Cargo.toml` 和 `Cargo.lock`。
- `terraform_state_restore`: `.tfstate` 和 `.tfstate.backup` files。
- `manual_approval_required`: `.env` files 和 environment/secret-like files。

## 使用

```bash
agenthub tx effects tx-...
cat .agent/tx/tx-.../rollback.json
```

`rollback.json` 是 structured rollback artifact。它列出每个 changed path、所选 handler、rollback status，以及需要额外谨慎时的 reason。
