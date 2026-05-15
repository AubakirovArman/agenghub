const assert = require('assert');
const fs = require('fs');
const os = require('os');
const path = require('path');
const { approvalRequired, latestState, listApprovalItems } = require('../src/approvals');
const { listSpecItems } = require('../src/specs');

const root = fs.mkdtempSync(path.join(os.tmpdir(), 'agenthub-vscode-'));

try {
  fs.mkdirSync(path.join(root, '.agent', 'specs'), { recursive: true });
  fs.mkdirSync(path.join(root, '.agent', 'tx', 'tx-blocked'), { recursive: true });
  fs.mkdirSync(path.join(root, 'examples'), { recursive: true });

  const approvalSpec = path.join(root, '.agent', 'specs', 'approval.yaml');
  fs.writeFileSync(approvalSpec, 'transaction:\n  approval_required: true\n');
  fs.writeFileSync(path.join(root, '.agent', 'specs', 'draft.yaml'), 'task:\n  id: draft\n');
  fs.writeFileSync(path.join(root, 'examples', 'content-task.yaml'), 'task:\n  id: example\n');
  fs.writeFileSync(path.join(root, 'examples', 'notes.yaml'), 'ignored: true\n');

  const journal = path.join(root, '.agent', 'tx', 'tx-blocked', 'journal.jsonl');
  fs.writeFileSync(journal, '{"state":"CREATED"}\n{"state":"BLOCKED_ON_HUMAN"}\n');

  assert.strictEqual(approvalRequired(fs.readFileSync(approvalSpec, 'utf8')), true);
  assert.strictEqual(approvalRequired('transaction:\n  approval_required: false\n'), false);
  assert.strictEqual(latestState(journal), 'BLOCKED_ON_HUMAN');

  const approvals = listApprovalItems(root);
  assert.deepStrictEqual(approvals.map((item) => item.kind).sort(), ['blocked_tx', 'spec']);

  const specs = listSpecItems(root);
  assert.deepStrictEqual(specs.map((item) => item.label).sort(), [
    'draft: approval.yaml',
    'draft: draft.yaml',
    'example: content-task.yaml'
  ]);
} finally {
  fs.rmSync(root, { recursive: true, force: true });
}
