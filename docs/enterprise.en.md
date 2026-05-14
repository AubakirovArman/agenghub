# AgentHub Enterprise Layer

Languages: [English](enterprise.en.md), [Русский](enterprise.ru.md), [中文](enterprise.zh.md), [Қазақша](enterprise.kk.md)

## Purpose

Phase 14 begins with local enterprise governance: project policy, role-based permissions, audit logs, and compliance reports. This is the local foundation for a future central policy server and remote enterprise runners.

## Files

```text
.agent/enterprise/policy.yaml
.agent/enterprise/audit.jsonl
.agent/enterprise/compliance-<timestamp>.md
```

`audit.jsonl` and generated compliance reports are runtime artifacts and are ignored by git.

## Policy Example

```yaml
enterprise:
  enabled: true
  default_role: developer
  roles:
    developer:
      permissions:
        - transaction.run
        - transaction.read
        - plugins.install
    auditor:
      permissions:
        - enterprise.audit.read
        - enterprise.compliance.generate
    admin:
      permissions:
        - "*"
  secrets:
    provider: env
    allowed_prefixes:
      - AGENTHUB_
  runners:
    default: local
    remote: []
  model_routing:
    private_models: []
```

## RBAC

AgentHub reads the actor from `AGENTHUB_ACTOR` and the role from `AGENTHUB_ROLE`. If no role is set, it uses `enterprise.default_role`.

Examples:

```bash
AGENTHUB_ACTOR=alice AGENTHUB_ROLE=developer agenthub run examples/command-task.yaml
AGENTHUB_ACTOR=bob AGENTHUB_ROLE=auditor agenthub enterprise audit --limit 20
AGENTHUB_ACTOR=carol AGENTHUB_ROLE=admin agenthub plugins install marketplace/skill-packs/content-basic --trust local
```

## Auditing

Audited actions currently include transaction runs, plugin installs, and compliance report generation.

```bash
AGENTHUB_ROLE=admin agenthub enterprise audit --limit 20
```

Output columns:

```text
created_at actor action outcome permission
```

## Compliance Reports

Generate a report:

```bash
AGENTHUB_ROLE=admin agenthub enterprise compliance
```

Generate to a fixed path:

```bash
AGENTHUB_ROLE=admin agenthub enterprise compliance --output tmp/compliance.md
```

The report includes policy status, default role, secret provider, runner mode, configured roles, installed plugin count, transaction count, and recent audit count.
