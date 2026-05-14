# AgentHub Enterprise Layer

Тілдер: [English](enterprise.en.md), [Русский](enterprise.ru.md), [中文](enterprise.zh.md), [Қазақша](enterprise.kk.md)

## Мақсаты

Phase 14 local enterprise governance арқылы басталады: project policy, role-based permissions, audit logs және compliance reports. Бұл болашақ central policy server және remote enterprise runners үшін жергілікті негіз.

## Файлдар

```text
.agent/enterprise/policy.yaml
.agent/enterprise/audit.jsonl
.agent/enterprise/compliance-<timestamp>.md
```

`audit.jsonl` және generated compliance reports runtime artifacts болып саналады және git оларды елемейді.

## Policy мысалы

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

AgentHub actor мәнін `AGENTHUB_ACTOR` ішінен, role мәнін `AGENTHUB_ROLE` ішінен оқиды. Егер role берілмесе, `enterprise.default_role` қолданылады.

Мысалдар:

```bash
AGENTHUB_ACTOR=alice AGENTHUB_ROLE=developer agenthub run examples/command-task.yaml
AGENTHUB_ACTOR=bob AGENTHUB_ROLE=auditor agenthub enterprise audit --limit 20
AGENTHUB_ACTOR=carol AGENTHUB_ROLE=admin agenthub plugins install marketplace/skill-packs/content-basic --trust local
```

## Audit

Қазір transaction runs, plugin installs және compliance report generation аудитке жазылады.

```bash
AGENTHUB_ROLE=admin agenthub enterprise audit --limit 20
```

Шығыс бағандары:

```text
created_at actor action outcome permission
```

## Compliance Reports

Report жасау:

```bash
AGENTHUB_ROLE=admin agenthub enterprise compliance
```

Белгілі path бойынша жасау:

```bash
AGENTHUB_ROLE=admin agenthub enterprise compliance --output tmp/compliance.md
```

Report policy status, default role, secret provider, runner mode, configured roles, installed plugin count, transaction count және recent audit count көрсетеді.
