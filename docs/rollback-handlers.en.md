# Rollback Handlers

Languages: [English](rollback-handlers.en.md), [Русский](rollback-handlers.ru.md), [中文](rollback-handlers.zh.md), [Қазақша](rollback-handlers.kk.md)

AgentHub v2 records concrete rollback handler choices for failed transactions. When a transaction rolls back, AgentHub writes `.agent/tx/<tx-id>/rollback.json` and mirrors file rollback statuses in `effects.jsonl`.

## Handler Selection

- `file_restore`: default file rollback through the isolated worktree cleanup path.
- `package_manifest_restore`: `package.json`, lockfiles, `Cargo.toml`, and `Cargo.lock`.
- `terraform_state_restore`: `.tfstate` and `.tfstate.backup` files.
- `manual_approval_required`: `.env` files and environment/secret-like files.

## Usage

```bash
agenthub tx effects tx-...
cat .agent/tx/tx-.../rollback.json
```

`rollback.json` is the structured rollback artifact. It lists each changed path, selected handler, rollback status, and a reason when the handler needs extra care.
