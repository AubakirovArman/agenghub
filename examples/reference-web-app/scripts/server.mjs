import { readFile } from 'node:fs/promises';
import http from 'node:http';
import path from 'node:path';

const root = process.cwd();
const args = new Map();
for (let index = 2; index < process.argv.length; index += 2) {
  args.set(process.argv[index], process.argv[index + 1]);
}

const host = args.get('--host') ?? '127.0.0.1';
const port = Number(args.get('--port') ?? 3000);

function pagePath(urlPath) {
  if (urlPath === '/') return path.join(root, 'src/app/dashboard/page.html');
  if (urlPath === '/styles/dashboard.css') {
    return path.join(root, 'src/app/styles/dashboard.css');
  }
  const route = urlPath.replace(/^\/+|\/+$/g, '');
  if (!/^[a-z0-9-]+$/.test(route)) return null;
  return path.join(root, 'src/app', route, 'page.html');
}

function contentType(file) {
  return file.endsWith('.css') ? 'text/css; charset=utf-8' : 'text/html; charset=utf-8';
}

const server = http.createServer(async (request, response) => {
  const file = pagePath(new URL(request.url, `http://${host}:${port}`).pathname);
  if (!file) {
    response.writeHead(404);
    response.end('not found');
    return;
  }
  try {
    response.writeHead(200, { 'content-type': contentType(file) });
    response.end(await readFile(file));
  } catch {
    response.writeHead(404);
    response.end('not found');
  }
});

server.listen(port, host);
process.on('SIGTERM', () => server.close(() => process.exit(0)));
process.on('SIGINT', () => server.close(() => process.exit(0)));
