# Verifier Integrations

Тілдер: [English](verifier-integrations.en.md), [Русский](verifier-integrations.ru.md), [中文](verifier-integrations.zh.md), [Қазақша](verifier-integrations.kk.md)

Verifier Integrations v2 бар verifier commands, runtime smoke checks және domain verifier profiles үстіне structured layer қосады. Бұрынғы `verifier.json` compatibility сақтайды; жаңа artifact — `verifier_integration.json`.

## Artifacts

Әр transaction мыналарды жазады:

- `.agent/tx/<tx-id>/verifier.json`: legacy verifier result.
- `.agent/tx/<tx-id>/verifier_integration.json`: unified structured checks.
- `.agent/tx/<tx-id>/report.md`: structured check count, failed count, fingerprint list және artifact references.

## Check Schema

Әр structured check құрамында:

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

Command verifiers category ретінде `command` қолданады; runtime route checks — `runtime_smoke`; domain profiles өз profile id мәнін қолданады, мысалы `content_quality` немесе `infra_plan`.

## Fingerprints And Trends

Failed checks stable fingerprints жасайды:

```json
{
  "check_id": "command-0",
  "fingerprint": "6f1a2b...",
  "reason": "command:test -f missing:exit Some(1), timeout false"
}
```

Rollback memory verifier fingerprint мәнін failed-attempt warning memory ішіне жазады. `trend` section total, passed, failed және per-category counts сақтайды, сондықтан dashboard және кейінгі analytics verifier quality агрегаттай алады.

## Plugin Compatibility

Егер installed plugin active profile үшін verifier metadata жарияласа, `verifier_integration.json` package id, verifier id, profiles және command жазады. Бұл қазіргі plugin manifest model мен болашақ verifier execution plugins арасындағы compatibility сақтайды.
