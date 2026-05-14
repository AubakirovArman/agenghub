# AgentHub Enterprise Layer

语言: [English](enterprise.en.md), [Русский](enterprise.ru.md), [中文](enterprise.zh.md), [Қазақша](enterprise.kk.md)

## 目的

Phase 14 从本地 enterprise governance 开始：project policy、role-based permissions、audit logs 和 compliance reports。这是未来 central policy server 与 remote enterprise runners 的本地基础。

## 文件

```text
.agent/enterprise/policy.yaml
.agent/enterprise/audit.jsonl
.agent/enterprise/compliance-<timestamp>.md
```

`audit.jsonl` 和生成的 compliance reports 是 runtime artifacts，已被 git 忽略。

## Policy 示例

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

AgentHub 从 `AGENTHUB_ACTOR` 读取 actor，从 `AGENTHUB_ROLE` 读取 role。如果未设置 role，则使用 `enterprise.default_role`。

示例：

```bash
AGENTHUB_ACTOR=alice AGENTHUB_ROLE=developer agenthub run examples/command-task.yaml
AGENTHUB_ACTOR=bob AGENTHUB_ROLE=auditor agenthub enterprise audit --limit 20
AGENTHUB_ACTOR=carol AGENTHUB_ROLE=admin agenthub plugins install marketplace/skill-packs/content-basic --trust local
```

## Audit

目前会记录 transaction runs、plugin installs 和 compliance report generation。

```bash
AGENTHUB_ROLE=admin agenthub enterprise audit --limit 20
```

输出列：

```text
created_at actor action outcome permission
```

## Compliance Reports

生成 report：

```bash
AGENTHUB_ROLE=admin agenthub enterprise compliance
```

生成到固定路径：

```bash
AGENTHUB_ROLE=admin agenthub enterprise compliance --output tmp/compliance.md
```

Report 包含 policy status、default role、secret provider、runner mode、configured roles、installed plugin count、transaction count 和 recent audit count。
