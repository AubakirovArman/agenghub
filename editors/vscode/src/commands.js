const vscode = require('vscode');
const fs = require('fs');
const path = require('path');
const { listApprovalItems } = require('./approvals');
const { runAgentHub, runAgentHubAsk } = require('./cli');
const { renderDagHtml } = require('./dagView');
const { latestTxDir, timestamp, workspaceRoot } = require('./utils');

function registerCommands(refreshAll = () => {}) {
  return [
    vscode.commands.registerCommand('agenthub.openLatestReport', openLatestReport),
    vscode.commands.registerCommand('agenthub.openMemory', openMemory),
    vscode.commands.registerCommand('agenthub.createSpecFromPrompt', () => createSpecFromPrompt(refreshAll)),
    vscode.commands.registerCommand('agenthub.openDag', openDag),
    vscode.commands.registerCommand('agenthub.openApprovals', () => openApprovals(refreshAll)),
    vscode.commands.registerCommand('agenthub.approveSpec', (filePath) => approveSpec(filePath, refreshAll))
  ];
}

async function openLatestReport() {
  const txDir = latestTxDir();
  if (!txDir) {
    vscode.window.showInformationMessage('No AgentHub transactions found.');
    return;
  }
  await vscode.commands.executeCommand('vscode.open', vscode.Uri.file(path.join(txDir, 'report.md')));
}

async function openMemory() {
  const root = workspaceRoot();
  if (!root) {
    return;
  }

  const picks = [
    ['Committed Memory', path.join(root, '.agent', 'memory', 'committed.jsonl')],
    ['Failed Attempts', path.join(root, '.agent', 'memory', 'failed_attempts.jsonl')],
    ['Compacted Project State', path.join(root, '.agent', 'memory', 'compacted', 'project_state.json')]
  ].filter(([, file]) => fs.existsSync(file));

  const selected = await vscode.window.showQuickPick(
    picks.map(([label, file]) => ({ label, file })),
    { placeHolder: 'Open AgentHub memory artifact' }
  );
  if (selected) {
    await vscode.commands.executeCommand('vscode.open', vscode.Uri.file(selected.file));
  }
}

async function createSpecFromPrompt(refreshAll) {
  const root = workspaceRoot();
  if (!root) {
    return;
  }

  const prompt = await vscode.window.showInputBox({
    prompt: 'Describe the AgentHub task',
    placeHolder: 'Add /courses page in the current dashboard style'
  });
  if (!prompt) {
    return;
  }

  const mode = await vscode.window.showQuickPick([
    { label: 'Draft Preview', approvalRequired: false },
    { label: 'Require Approval', approvalRequired: true }
  ], { placeHolder: 'Choose AgentSpec preview mode' });
  if (!mode) {
    return;
  }

  const yaml = await runAgentHubAsk(root, prompt, mode.approvalRequired);
  const specsDir = path.join(root, '.agent', 'specs');
  fs.mkdirSync(specsDir, { recursive: true });
  const prefix = mode.approvalRequired ? 'approval' : 'preview';
  const specPath = path.join(specsDir, `${prefix}-${timestamp()}.yaml`);
  fs.writeFileSync(specPath, yaml);
  await vscode.commands.executeCommand('vscode.open', vscode.Uri.file(specPath));
  refreshAll();
}

async function openDag(filePath) {
  const txDir = latestTxDir();
  const dagPath = typeof filePath === 'string' ? filePath : txDir && path.join(txDir, 'dag.json');
  if (!dagPath || !fs.existsSync(dagPath)) {
    vscode.window.showInformationMessage('No AgentHub DAG found.');
    return;
  }

  const dag = JSON.parse(fs.readFileSync(dagPath, 'utf8'));
  const panel = vscode.window.createWebviewPanel(
    'agenthubDag',
    `AgentHub DAG: ${dag.task_id || path.basename(path.dirname(dagPath))}`,
    vscode.ViewColumn.One,
    { enableScripts: false }
  );
  panel.webview.html = renderDagHtml(dag);
}

async function openApprovals(refreshAll) {
  const root = workspaceRoot();
  if (!root) {
    return;
  }

  const items = listApprovalItems(root);
  if (items.length === 0) {
    vscode.window.showInformationMessage('No AgentHub approvals are waiting.');
    return;
  }

  const selected = await vscode.window.showQuickPick(items, {
    placeHolder: 'Select an AgentHub approval item'
  });
  if (!selected) {
    return;
  }

  if (selected.kind === 'spec') {
    await approveSpec(selected.filePath, refreshAll);
    return;
  }
  await vscode.commands.executeCommand(
    'vscode.open',
    vscode.Uri.file(path.join(selected.txDir, 'report.md'))
  );
}

async function approveSpec(filePath, refreshAll) {
  const root = workspaceRoot();
  if (!root || !filePath) {
    return;
  }

  const action = await vscode.window.showWarningMessage(
    `Run approved AgentSpec ${path.basename(filePath)}?`,
    { modal: true },
    'Run',
    'Run Without Commit',
    'Open Spec'
  );
  if (!action) {
    return;
  }
  if (action === 'Open Spec') {
    await vscode.commands.executeCommand('vscode.open', vscode.Uri.file(filePath));
    return;
  }

  const args = ['run', filePath];
  if (action === 'Run Without Commit') {
    args.push('--no-commit');
  }
  await runApprovedSpec(root, args, refreshAll);
}

async function runApprovedSpec(root, args, refreshAll) {
  try {
    const output = await runAgentHub(root, args);
    vscode.window.showInformationMessage(output.trim() || 'AgentHub transaction finished.');
    refreshAll();
  } catch (error) {
    vscode.window.showErrorMessage(`AgentHub approval failed: ${error.message}`);
  }
}

module.exports = {
  registerCommands
};
