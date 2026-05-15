# Reference Web Fixture

Языки: [English](reference-web-fixture.en.md), [Русский](reference-web-fixture.ru.md), [中文](reference-web-fixture.zh.md), [Қазақша](reference-web-fixture.kk.md)

Reference web fixture закрывает PRD-сценарий "Add Page to Existing Web App". В репозитории есть маленькое Next.js-style приложение `examples/reference-web-app` и AgentSpec `examples/reference-web-add-courses.yaml`.

Fixture не требует внешних npm-зависимостей: `npm run build` и `npm run dev` используют локальные Node scripts. При этом структура похожа на app router (`src/app/<route>/page.html`), поэтому AgentHub проверяет создание route, reuse существующего style, scope enforcement, build verification, runtime smoke, memory promotion, report, cost artifacts и WAL replay.

## Запуск

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

После успеха появится `$tmp/src/app/courses/page.html`. Отчёт находится в `$tmp/.agent/tx/<tx-id>/report.md`; `verifier.json` содержит успешный runtime smoke для `/courses`, `cost.json` показывает cost estimates, `committed.jsonl` содержит promoted memory, а `wal_replay.json` содержит проверенный replay WAL.

## Что проверяется

- прямые изменения выполняются в isolated git worktree до merge;
- `scope.allow` разрешает только `src/app/courses/**`;
- `scope.deny` блокирует dashboard, styles, package и scripts;
- `npm run build` проверяет форму route и reuse style;
- `npm run dev` запускает fixture, а AgentHub проверяет `/courses` по HTTP 200;
- failed scope violations откатываются и сохраняются в `.agent/memory/failed_attempts.jsonl`.
