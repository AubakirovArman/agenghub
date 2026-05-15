# Verifier Integrations

Languages: [English](verifier-integrations.en.md), [Русский](verifier-integrations.ru.md), [中文](verifier-integrations.zh.md), [Қазақша](verifier-integrations.kk.md)

Verifier Integrations v2 adds a structured layer over existing verifier commands, runtime smoke checks, and domain verifier profiles. Existing `verifier.json` stays compatible; the new artifact is `verifier_integration.json`.

## Artifacts

Each transaction writes:

- `.agent/tx/<tx-id>/verifier.json`: legacy verifier result.
- `.agent/tx/<tx-id>/verifier_integration.json`: unified structured checks.
- `.agent/tx/<tx-id>/report.md`: structured check count, failed count, fingerprint list, and artifact references.

## Check Schema

Each structured check includes:

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

Command verifiers use category `command`; runtime route checks use `runtime_smoke`; domain profiles use their profile id, such as `content_quality` or `infra_plan`.

## Fingerprints And Trends

Failed checks produce stable fingerprints:

```json
{
  "check_id": "command-0",
  "fingerprint": "6f1a2b...",
  "reason": "command:test -f missing:exit Some(1), timeout false"
}
```

Rollback memory includes the verifier fingerprint in failed-attempt warning memory. The `trend` section stores total, passed, failed, and per-category counts so dashboards and later analytics can aggregate verifier quality.

## Plugin Compatibility

If an installed plugin declares verifier metadata for the active profile, `verifier_integration.json` records the package id, verifier id, profiles, and command. This keeps the current plugin manifest model compatible with future verifier execution plugins.
