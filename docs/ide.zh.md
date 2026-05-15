# IDE 和 Visual Layer

Phase 12 以 `editors/vscode` 中的零构建 VS Code extension 实现。

## 功能

- `Transactions` view：列出 `.agent/tx` runs，并打开 reports、DAGs、journals、verifier logs。
- `Memory` view：打开 committed memory、failed attempts 和 compacted project state。
- `AgentSpecs` view：打开 `.agent/specs` 中的 drafts，以及 `examples` 中的 task examples。
- `Approvals` view：显示带有 `approval_required: true` 的 AgentSpec drafts，以及 `BLOCKED_ON_HUMAN` transactions。
- DAG webview：把 `dag.json` 渲染成 visual execution graph。

## 创建 AgentSpec

运行 `AgentHub: Create AgentSpec From Prompt`，输入 natural request，然后选择模式：

- `Draft Preview`：写入 `.agent/specs/preview-<timestamp>.yaml`。
- `Require Approval`：写入 `.agent/specs/approval-<timestamp>.yaml`，并设置 `transaction.approval_required: true`。

示例 prompt：

```text
Add /courses page in the current dashboard style
```

该命令先尝试 `agenthub ask`；如果全局 CLI 不存在，会 fallback 到：

```bash
cargo run --quiet -- ask "Add /courses page in the current dashboard style"
```

## Approval Flow

1. 用 `Require Approval` 创建 preview。
2. 打开 `Approvals` view，或运行 `AgentHub: Open Approvals`。
3. 选择 pending AgentSpec。
4. 选择 `Run`、`Run Without Commit` 或 `Open Spec`。

`Run Without Commit` 会执行：

```bash
agenthub run .agent/specs/approval-<timestamp>.yaml --no-commit
```

Blocked transactions 也会显示在 `Approvals` 中。选择它会打开 `report.md`。

## 本地开发

在 VS Code 中打开 `editors/vscode`，启动 Extension Development Host。Extension 是 plain JavaScript，不需要 build step。

运行检查：

```bash
cd editors/vscode
npm run check
```
