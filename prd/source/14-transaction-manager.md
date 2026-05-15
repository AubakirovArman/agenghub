# 14. Transaction Manager

## 14.1 Назначение

Transaction Manager — ядро безопасного исполнения.

Он управляет:

* transaction lifecycle;
* workspace isolation;
* baseline capture;
* effect tracking;
* verifier loops;
* repair attempts;
* sync check;
* commit;
* rollback;
* memory promotion;
* reports.

## 14.2 Transaction Lifecycle

```text
CREATED
  ↓
PREFLIGHT_CHECK
  ↓
BASELINE_CAPTURED
  ↓
WORKSPACE_READY
  ↓
CONTEXT_PACK_BUILT
  ↓
PATCHING / EXECUTING
  ↓
DIFF_GUARD
  ↓
VERIFYING
  ↓
REPAIRING if needed
  ↓
SYNC_CHECK
  ↓
FINAL_VERIFY
  ↓
COMMITTED or ROLLED_BACK or BLOCKED_ON_HUMAN
  ↓
POST_COMMIT_EFFECTS
  ↓
CLOSED
```

## 14.3 Failure States

```text
DIFF_GUARD_FAILED → ROLLED_BACK
VERIFY_FAILED_AFTER_N → ROLLED_BACK or BLOCKED_ON_HUMAN
SYNC_CONFLICT → BLOCKED_ON_HUMAN
MISSING_ENV → BLOCKED_ON_HUMAN
POST_COMMIT_EFFECT_FAILED → COMMITTED_PENDING_EFFECTS
```

## 14.4 BLOCKED_ON_HUMAN

Не всё должно сразу откатываться.

Примеры:

* missing env;
* Docker daemon not running;
* port unavailable;
* API key required;
* merge conflict;
* approval required for package install;
* Terraform apply requires approval.

## 14.5 Sync Check

Перед commit:

* проверить base_head;
* проверить current_head;
* проверить hash scoped files;
* определить пересечение изменений main и agent;
* если есть overlap — BLOCKED_ON_HUMAN;
* если нет overlap — rebase + rerun verifier.

Принцип:

```text
No blind merge.
```

## 14.6 Diff Guard

Ограничивает опасные патчи.

Пример политики:

```yaml
diff_limits:
  max_files_changed: 12
  max_lines_added: 600
  max_lines_deleted: 300
  max_single_file_change_percent: 35
  deletion_requires_approval: true
  package_change_requires_skill: dependency_change
```

Если diff опасный:

```text
HARD_FAIL → rollback → failed_attempt log
```

## 14.7 Effect Tracking

Агент может делать эффекты:

* file changes;
* dependency changes;
* DB migrations;
* external API calls;
* Docker containers;
* generated artifacts;
* cloud resources;
* env changes.

Каждый effect должен иметь:

* type;
* scope;
* rollback handler;
* verifier;
* approval policy;
* journal entry.

---

