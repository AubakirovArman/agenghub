# Reference Web Fixture

Languages: [English](reference-web-fixture.en.md), [Русский](reference-web-fixture.ru.md), [中文](reference-web-fixture.zh.md), [Қазақша](reference-web-fixture.kk.md)

The reference web fixture closes the PRD scenario "Add Page to Existing Web App". It ships a small Next.js-style app in `examples/reference-web-app` and an AgentSpec in `examples/reference-web-add-courses.yaml`.

The fixture is dependency-free: `npm run build` and `npm run dev` use local Node scripts. The directory still follows the app-router shape (`src/app/<route>/page.html`) so AgentHub can test route creation, style reuse, scope enforcement, build verification, runtime smoke, memory promotion, report generation, cost artifacts, and WAL replay.

## Run It

```bash
tmp="$(mktemp -d)"
cp -R examples/reference-web-app/. "$tmp/"
git -C "$tmp" init
git -C "$tmp" config user.email test@example.com
git -C "$tmp" config user.name "AgentHub Test"
cargo run -- --project "$tmp" init
git -C "$tmp" add -A
git -C "$tmp" commit -m "reference web baseline"
cargo run -- --project "$tmp" run examples/reference-web-add-courses.yaml
```

After success, `$tmp/src/app/courses/page.html` exists. The transaction report is under `$tmp/.agent/tx/<tx-id>/report.md`; `verifier.json` contains a successful `/courses` runtime smoke result, `cost.json` exposes cost estimates, `committed.jsonl` contains promoted memory, and `wal_replay.json` contains the validated WAL replay.

## What It Proves

- direct edits happen in an isolated git worktree before merge;
- `scope.allow` permits only `src/app/courses/**`;
- `scope.deny` blocks edits to dashboard, styles, package, and scripts;
- `npm run build` validates route shape and style reuse;
- `npm run dev` serves the fixture and AgentHub checks `/courses` with HTTP 200;
- failed scope violations roll back and are stored in `.agent/memory/failed_attempts.jsonl`.
