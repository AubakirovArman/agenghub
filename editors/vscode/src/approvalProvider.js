const vscode = require('vscode');
const path = require('path');
const { listApprovalItems } = require('./approvals');
const { workspaceRoot } = require('./utils');

class ApprovalsProvider {
  constructor() {
    this._onDidChangeTreeData = new vscode.EventEmitter();
    this.onDidChangeTreeData = this._onDidChangeTreeData.event;
  }

  refresh() {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(item) {
    return item;
  }

  getChildren() {
    const root = workspaceRoot();
    if (!root) {
      return [];
    }
    return listApprovalItems(root).map((item) => approvalTreeItem(item));
  }
}

function approvalTreeItem(entry) {
  const item = new vscode.TreeItem(entry.label, vscode.TreeItemCollapsibleState.None);
  item.description = entry.description;
  item.tooltip = entry.detail;
  item.contextValue = entry.kind === 'spec' ? 'agenthubApprovalSpec' : 'agenthubBlockedTx';
  item.iconPath = new vscode.ThemeIcon(entry.kind === 'spec' ? 'pass' : 'warning');
  item.command = commandFor(entry);
  return item;
}

function commandFor(entry) {
  if (entry.kind === 'spec') {
    return {
      command: 'agenthub.approveSpec',
      title: 'Approve AgentSpec',
      arguments: [entry.filePath]
    };
  }
  return {
    command: 'vscode.open',
    title: 'Open Report',
    arguments: [vscode.Uri.file(path.join(entry.txDir, 'report.md'))]
  };
}

module.exports = {
  ApprovalsProvider
};
