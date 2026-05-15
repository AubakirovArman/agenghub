const fs = require('fs');
const path = require('path');

function listSpecItems(root) {
  return [
    ...listFiles(path.join(root, '.agent', 'specs'), 'draft'),
    ...listFiles(path.join(root, 'examples'), 'example')
  ].sort((a, b) => a.label.localeCompare(b.label));
}

function listFiles(dir, kind) {
  if (!fs.existsSync(dir)) {
    return [];
  }
  return fs.readdirSync(dir, { withFileTypes: true })
    .filter((entry) => entry.isFile() && /\.(ya?ml)$/i.test(entry.name))
    .filter((entry) => kind === 'draft' || /task\.ya?ml$/i.test(entry.name))
    .map((entry) => {
      const filePath = path.join(dir, entry.name);
      return {
        kind,
        label: `${kind}: ${entry.name}`,
        description: kind === 'draft' ? '.agent/specs' : 'examples',
        filePath
      };
    });
}

module.exports = {
  listSpecItems
};
