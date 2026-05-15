# IDE Және Visual Layer

Phase 12 `editors/vscode` ішіндегі zero-build VS Code extension ретінде іске асырылған.

## Не Береді

- `Transactions` view: `.agent/tx` runs тізімін көрсетеді және reports, DAGs, journals, verifier logs ашады.
- `Memory` view: committed memory, failed attempts және compacted project state ашады.
- `AgentSpecs` view: `.agent/specs` ішіндегі drafts және `examples` ішіндегі task examples ашады.
- `Approvals` view: `approval_required: true` бар AgentSpec drafts және `BLOCKED_ON_HUMAN` transactions көрсетеді.
- DAG webview: `dag.json` файлын visual execution graph ретінде көрсетеді.

## AgentSpec Жасау

`AgentHub: Create AgentSpec From Prompt` командасын іске қос, natural request енгіз және режим таңда:

- `Draft Preview`: `.agent/specs/preview-<timestamp>.yaml` жазады.
- `Require Approval`: `.agent/specs/approval-<timestamp>.yaml` жазады және `transaction.approval_required: true` қояды.

Prompt мысалы:

```text
Add /courses page in the current dashboard style
```

Команда алдымен `agenthub ask` қолданады; global CLI жоқ болса, fallback ретінде мынаны іске қосады:

```bash
cargo run --quiet -- ask "Add /courses page in the current dashboard style"
```

## Approval Flow

1. `Require Approval` режимімен preview жаса.
2. `Approvals` view аш немесе `AgentHub: Open Approvals` командасын іске қос.
3. Pending AgentSpec таңда.
4. `Run`, `Run Without Commit` немесе `Open Spec` таңда.

`Run Without Commit` мынаны орындайды:

```bash
agenthub run .agent/specs/approval-<timestamp>.yaml --no-commit
```

Blocked transactions да `Approvals` ішінде көрінеді. Оны таңдасаң, `report.md` ашылады.

## Local Development

VS Code ішінде `editors/vscode` ашып, Extension Development Host іске қос. Extension plain JavaScript, сондықтан build step қажет емес.

Тексеру:

```bash
cd editors/vscode
npm run check
```
