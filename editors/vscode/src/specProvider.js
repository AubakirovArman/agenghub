const vscode = require('vscode');
const { listSpecItems } = require('./specs');
const { workspaceRoot } = require('./utils');

class SpecsProvider {
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
    return listSpecItems(root).map((entry) => specTreeItem(entry));
  }
}

function specTreeItem(entry) {
  const item = new vscode.TreeItem(entry.label, vscode.TreeItemCollapsibleState.None);
  item.description = entry.description;
  item.resourceUri = vscode.Uri.file(entry.filePath);
  item.contextValue = 'agenthubSpec';
  item.iconPath = new vscode.ThemeIcon(entry.kind === 'draft' ? 'edit' : 'file-code');
  item.command = {
    command: 'vscode.open',
    title: 'Open AgentSpec',
    arguments: [vscode.Uri.file(entry.filePath)]
  };
  return item;
}

module.exports = {
  SpecsProvider
};
