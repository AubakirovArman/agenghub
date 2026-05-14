# AgentHub Enterprise Layer

Языки: [English](enterprise.en.md), [Русский](enterprise.ru.md), [中文](enterprise.zh.md), [Қазақша](enterprise.kk.md)

## Назначение

Phase 14 начинается с локального enterprise governance: project policy, role-based permissions, audit logs и compliance reports. Это локальная основа для будущего central policy server и remote enterprise runners.

## Файлы

```text
.agent/enterprise/policy.yaml
.agent/enterprise/audit.jsonl
.agent/enterprise/compliance-<timestamp>.md
```

`audit.jsonl` и generated compliance reports являются runtime artifacts и игнорируются git.

## Пример policy

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

AgentHub читает actor из `AGENTHUB_ACTOR`, а role из `AGENTHUB_ROLE`. Если role не задана, используется `enterprise.default_role`.

Примеры:

```bash
AGENTHUB_ACTOR=alice AGENTHUB_ROLE=developer agenthub run examples/command-task.yaml
AGENTHUB_ACTOR=bob AGENTHUB_ROLE=auditor agenthub enterprise audit --limit 20
AGENTHUB_ACTOR=carol AGENTHUB_ROLE=admin agenthub plugins install marketplace/skill-packs/content-basic --trust local
```

## Аудит

Сейчас аудитируются transaction runs, plugin installs и compliance report generation.

```bash
AGENTHUB_ROLE=admin agenthub enterprise audit --limit 20
```

Колонки вывода:

```text
created_at actor action outcome permission
```

## Compliance reports

Создать report:

```bash
AGENTHUB_ROLE=admin agenthub enterprise compliance
```

Создать report по фиксированному пути:

```bash
AGENTHUB_ROLE=admin agenthub enterprise compliance --output tmp/compliance.md
```

Report включает policy status, default role, secret provider, runner mode, configured roles, installed plugin count, transaction count и recent audit count.
