# IDE And Visual Layer

Phase 12 is implemented as a zero-build VS Code extension in `editors/vscode`.

## What It Provides

- `Transactions` view: lists `.agent/tx` runs and opens reports, DAGs, journals, and verifier logs.
- `Memory` view: opens committed memory, failed attempts, and compacted project state.
- `AgentSpecs` view: opens drafts from `.agent/specs` and task examples from `examples`.
- `Approvals` view: shows AgentSpec drafts marked with `approval_required: true` and transactions blocked on human input.
- DAG webview: renders `dag.json` as a visual execution graph.

## Create An AgentSpec

Run `AgentHub: Create AgentSpec From Prompt`, enter a natural request, and choose one mode:

- `Draft Preview`: writes `.agent/specs/preview-<timestamp>.yaml`.
- `Require Approval`: writes `.agent/specs/approval-<timestamp>.yaml` with `transaction.approval_required: true`.

Example prompt:

```text
Add /courses page in the current dashboard style
```

The command first tries `agenthub ask`; if the CLI is not installed globally, it falls back to:

```bash
cargo run --quiet -- ask "Add /courses page in the current dashboard style"
```

## Approval Flow

1. Create a preview with `Require Approval`.
2. Open the `Approvals` view or run `AgentHub: Open Approvals`.
3. Select the pending AgentSpec.
4. Choose `Run`, `Run Without Commit`, or `Open Spec`.

`Run Without Commit` executes:

```bash
agenthub run .agent/specs/approval-<timestamp>.yaml --no-commit
```

Blocked transactions are also shown in the `Approvals` view. Selecting one opens its `report.md`.

## Local Development

Open `editors/vscode` in VS Code and start an Extension Development Host. The extension is plain JavaScript, so no build step is required.

Run checks:

```bash
cd editors/vscode
npm run check
```
