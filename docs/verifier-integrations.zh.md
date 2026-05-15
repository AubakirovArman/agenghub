# Verifier Integrations

语言: [English](verifier-integrations.en.md), [Русский](verifier-integrations.ru.md), [中文](verifier-integrations.zh.md), [Қазақша](verifier-integrations.kk.md)

Verifier Integrations v2 在现有 verifier commands、runtime smoke checks 和 domain verifier profiles 之上增加 structured layer。旧的 `verifier.json` 保持兼容；新的 artifact 是 `verifier_integration.json`。

## Artifacts

每个 transaction 会写入：

- `.agent/tx/<tx-id>/verifier.json`: legacy verifier result。
- `.agent/tx/<tx-id>/verifier_integration.json`: unified structured checks。
- `.agent/tx/<tx-id>/report.md`: structured check count、failed count、fingerprint list 和 artifact references。

## Check Schema

每个 structured check 包含：

```json
{
  "id": "command-0",
  "category": "command",
  "name": "cargo test",
  "status": "passed",
  "detail": "exit Some(0), timeout false",
  "command": "cargo test"
}
```

Command verifiers 使用 category `command`；runtime route checks 使用 `runtime_smoke`；domain profiles 使用自己的 profile id，例如 `content_quality` 或 `infra_plan`。

## Fingerprints And Trends

Failed checks 会生成 stable fingerprints：

```json
{
  "check_id": "command-0",
  "fingerprint": "6f1a2b...",
  "reason": "command:test -f missing:exit Some(1), timeout false"
}
```

Rollback memory 会把 verifier fingerprint 写入 failed-attempt warning memory。`trend` section 保存 total、passed、failed 和 per-category counts，供 dashboard 和未来 analytics 聚合 verifier quality。

## Plugin Compatibility

如果 installed plugin 为当前 profile 声明了 verifier metadata，`verifier_integration.json` 会记录 package id、verifier id、profiles 和 command。这让当前 plugin manifest model 与未来的 verifier execution plugins 保持兼容。
