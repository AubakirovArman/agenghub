const vscode = require('vscode');
const { registerCommands } = require('./src/commands');
const { MemoryProvider, TransactionsProvider } = require('./src/providers');
const { ApprovalsProvider } = require('./src/approvalProvider');
const { SpecsProvider } = require('./src/specProvider');

function activate(context) {
  const txProvider = new TransactionsProvider();
  const memoryProvider = new MemoryProvider();
  const approvalsProvider = new ApprovalsProvider();
  const specsProvider = new SpecsProvider();
  const refreshAll = () => {
    txProvider.refresh();
    memoryProvider.refresh();
    approvalsProvider.refresh();
    specsProvider.refresh();
  };

  context.subscriptions.push(
    vscode.window.registerTreeDataProvider('agenthub.transactions', txProvider),
    vscode.window.registerTreeDataProvider('agenthub.memory', memoryProvider),
    vscode.window.registerTreeDataProvider('agenthub.specs', specsProvider),
    vscode.window.registerTreeDataProvider('agenthub.approvals', approvalsProvider),
    vscode.commands.registerCommand('agenthub.refresh', () => {
      refreshAll();
    }),
    ...registerCommands(refreshAll)
  );
}

function deactivate() {}

module.exports = {
  activate,
  deactivate
};
