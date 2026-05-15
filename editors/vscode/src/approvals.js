const fs = require('fs');
const path = require('path');

function listApprovalItems(root) {
  return [
    ...listApprovalSpecs(root),
    ...listBlockedTransactions(root)
  ].sort((a, b) => b.sortKey.localeCompare(a.sortKey));
}

function listApprovalSpecs(root) {
  const specsDir = path.join(root, '.agent', 'specs');
  return listYamlFiles(specsDir)
    .filter((filePath) => approvalRequired(fs.readFileSync(filePath, 'utf8')))
    .map((filePath) => ({
      kind: 'spec',
      label: path.basename(filePath),
      description: 'approval required',
      detail: filePath,
      filePath,
      sortKey: filePath
    }));
}

function listBlockedTransactions(root) {
  const txRoot = path.join(root, '.agent', 'tx');
  if (!fs.existsSync(txRoot)) {
    return [];
  }
  return fs.readdirSync(txRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => path.join(txRoot, entry.name))
    .filter((txDir) => latestState(path.join(txDir, 'journal.jsonl')) === 'BLOCKED_ON_HUMAN')
    .map((txDir) => ({
      kind: 'blocked_tx',
      label: path.basename(txDir),
      description: 'blocked on human',
      detail: path.join(txDir, 'report.md'),
      txDir,
      sortKey: txDir
    }));
}

function listYamlFiles(dir) {
  if (!fs.existsSync(dir)) {
    return [];
  }
  return fs.readdirSync(dir, { withFileTypes: true })
    .filter((entry) => entry.isFile() && /\.(ya?ml)$/i.test(entry.name))
    .map((entry) => path.join(dir, entry.name));
}

function approvalRequired(text) {
  return /^\s*approval_required:\s*true\s*$/m.test(text);
}

function latestState(journalPath) {
  if (!fs.existsSync(journalPath)) {
    return 'UNKNOWN';
  }
  const lines = fs.readFileSync(journalPath, 'utf8').trim().split(/\r?\n/).filter(Boolean);
  if (lines.length === 0) {
    return 'UNKNOWN';
  }
  try {
    return JSON.parse(lines[lines.length - 1]).state || 'UNKNOWN';
  } catch (_) {
    return 'UNKNOWN';
  }
}

module.exports = {
  approvalRequired,
  latestState,
  listApprovalItems
};
