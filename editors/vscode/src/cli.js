const vscode = require('vscode');
const childProcess = require('child_process');

function runAgentHub(root, args) {
  const executable = vscode.workspace.getConfiguration('agenthub').get('executable', 'agenthub');
  return execFile(executable, args, root).catch(() => (
    execFile('cargo', ['run', '--quiet', '--', ...args], root)
  ));
}

function runAgentHubAsk(root, prompt, approvalRequired) {
  const args = ['ask'];
  if (approvalRequired) {
    args.push('--approval-required');
  }
  args.push(prompt);
  return runAgentHub(root, args);
}

function execFile(command, args, cwd) {
  return new Promise((resolve, reject) => {
    childProcess.execFile(command, args, { cwd }, (error, stdout, stderr) => {
      if (error) {
        reject(new Error(stderr || error.message));
      } else {
        resolve(stdout);
      }
    });
  });
}

module.exports = {
  runAgentHub,
  runAgentHubAsk
};
