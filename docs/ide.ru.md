# IDE И Visual Layer

Phase 12 реализована как VS Code extension без build step в `editors/vscode`.

## Что Дает

- `Transactions`: показывает `.agent/tx` runs и открывает reports, DAGs, journals, verifier logs.
- `Memory`: открывает committed memory, failed attempts и compacted project state.
- `AgentSpecs`: открывает drafts из `.agent/specs` и task examples из `examples`.
- `Approvals`: показывает AgentSpec drafts с `approval_required: true` и transactions со статусом `BLOCKED_ON_HUMAN`.
- DAG webview: рисует `dag.json` как visual execution graph.

## Создать AgentSpec

Запусти `AgentHub: Create AgentSpec From Prompt`, введи natural request и выбери режим:

- `Draft Preview`: пишет `.agent/specs/preview-<timestamp>.yaml`.
- `Require Approval`: пишет `.agent/specs/approval-<timestamp>.yaml` с `transaction.approval_required: true`.

Пример prompt:

```text
Add /courses page in the current dashboard style
```

Команда сначала пробует `agenthub ask`; если CLI не установлен глобально, использует fallback:

```bash
cargo run --quiet -- ask "Add /courses page in the current dashboard style"
```

## Approval Flow

1. Создай preview в режиме `Require Approval`.
2. Открой view `Approvals` или команду `AgentHub: Open Approvals`.
3. Выбери pending AgentSpec.
4. Нажми `Run`, `Run Without Commit` или `Open Spec`.

`Run Without Commit` выполняет:

```bash
agenthub run .agent/specs/approval-<timestamp>.yaml --no-commit
```

Blocked transactions тоже видны в `Approvals`. Выбор такого пункта открывает `report.md`.

## Локальная Разработка

Открой `editors/vscode` в VS Code и запусти Extension Development Host. Extension написан на plain JavaScript, поэтому build step не нужен.

Проверки:

```bash
cd editors/vscode
npm run check
```
