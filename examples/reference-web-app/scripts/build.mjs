import { copyFile, mkdir, readdir, readFile, rm } from 'node:fs/promises';
import path from 'node:path';

const root = process.cwd();
const appDir = path.join(root, 'src', 'app');
const outDir = path.join(root, '.next', 'agenthub');

async function discoverPages() {
  const entries = await readdir(appDir, { withFileTypes: true });
  const pages = [];
  for (const entry of entries) {
    if (!entry.isDirectory() || entry.name === 'styles') continue;
    const file = path.join(appDir, entry.name, 'page.html');
    pages.push({ route: entry.name, file });
  }
  return pages;
}

async function validatePage(page) {
  const html = await readFile(page.file, 'utf8');
  if (!html.includes('/styles/dashboard.css')) {
    throw new Error(`${page.route} does not reuse dashboard.css`);
  }
  if (!/<main[\s>]/.test(html) || !/<h1[\s>]/.test(html)) {
    throw new Error(`${page.route} is missing main content landmarks`);
  }
}

await rm(outDir, { recursive: true, force: true });
const pages = await discoverPages();
if (pages.length === 0) throw new Error('no app routes found');

for (const page of pages) {
  await validatePage(page);
  const target = path.join(outDir, page.route, 'index.html');
  await mkdir(path.dirname(target), { recursive: true });
  await copyFile(page.file, target);
}

console.log(`built ${pages.length} route(s) into .next/agenthub`);
