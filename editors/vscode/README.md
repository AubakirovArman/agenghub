# AgentHub VS Code Extension

This is the Phase 12 v0 IDE surface for AgentHub.

It is intentionally zero-build JavaScript:

- transaction tree over `.agent/tx`;
- memory tree over `.agent/memory`;
- AgentSpec tree for `.agent/specs` drafts and `examples/*task.yaml`;
- approval queue for pending specs and blocked transactions;
- latest report opener;
- DAG webview for `dag.json`;
- prompt-to-AgentSpec preview command backed by `agenthub ask`;
- AgentSpec JSON schema validation.

Approval previews are created through `AgentHub: Create AgentSpec From Prompt`
with `Require Approval`. They appear in the `Approvals` view and can be run
normally, run with `--no-commit`, or opened for editing.

## Local Development

Open this folder in VS Code and run the extension host from `editors/vscode`.

The extension first tries `agenthub ask`. If the CLI is not installed globally,
it falls back to `cargo run --quiet -- ask` from the workspace root.

Run local checks with:

```bash
npm run check
```
