# Verifier Integrations

Языки: [English](verifier-integrations.en.md), [Русский](verifier-integrations.ru.md), [中文](verifier-integrations.zh.md), [Қазақша](verifier-integrations.kk.md)

Verifier Integrations v2 добавляет structured layer поверх существующих verifier commands, runtime smoke checks и domain verifier profiles. Старый `verifier.json` остаётся совместимым; новый artifact — `verifier_integration.json`.

## Artifacts

Каждая transaction пишет:

- `.agent/tx/<tx-id>/verifier.json`: legacy verifier result.
- `.agent/tx/<tx-id>/verifier_integration.json`: unified structured checks.
- `.agent/tx/<tx-id>/report.md`: structured check count, failed count, fingerprint list и artifact references.

## Check Schema

Каждый structured check содержит:

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

Command verifiers получают category `command`; runtime route checks — `runtime_smoke`; domain profiles используют свой profile id, например `content_quality` или `infra_plan`.

## Fingerprints And Trends

Failed checks создают stable fingerprints:

```json
{
  "check_id": "command-0",
  "fingerprint": "6f1a2b...",
  "reason": "command:test -f missing:exit Some(1), timeout false"
}
```

Rollback memory записывает verifier fingerprint в failed-attempt warning memory. Секция `trend` хранит total, passed, failed и per-category counts, чтобы dashboard и будущая analytics могли агрегировать verifier quality.

## Plugin Compatibility

Если installed plugin объявляет verifier metadata для активного profile, `verifier_integration.json` записывает package id, verifier id, profiles и command. Это сохраняет совместимость текущей plugin manifest model с будущими verifier execution plugins.
