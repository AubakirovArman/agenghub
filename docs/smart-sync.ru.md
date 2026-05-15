# Smart Sync

Языки: [English](smart-sync.en.md), [Русский](smart-sync.ru.md), [中文](smart-sync.zh.md), [Қазақша](smart-sync.kk.md)

Smart Sync заменяет старый HEAD-only block на file-level overlap detection. Когда project `HEAD` меняется во время транзакции, AgentHub сравнивает файлы, изменённые в main, с файлами, изменёнными транзакцией.

## Решения

- `clean`: project `HEAD` не сдвинулся.
- `rebase_required`: project `HEAD` сдвинулся, но file sets не пересекаются. AgentHub запускает `git rebase --autostash`, повторяет diff guard и verifier, затем делает commit.
- `blocked_overlap`: project `HEAD` сдвинулся и обе стороны изменили хотя бы один одинаковый path. AgentHub блокирует на human вместо blind merge.

## Артефакты

```text
.agent/tx/<tx-id>/baseline.json
.agent/tx/<tx-id>/sync.json
.agent/tx/<tx-id>/report.md
```

`baseline.json` хранит transaction `base_head`, хеши scoped files из `scope.allow` и хеши relevant files, выбранных из context maps.

`sync.json` хранит base/current heads, main changed files, transaction changed files, overlaps и флаг verifier rerun.

Пример:

```json
{
  "decision": "rebase_required",
  "overlapping_files": [],
  "verifier_rerun_required": true
}
```
