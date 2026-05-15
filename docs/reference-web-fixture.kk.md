# Reference Web Fixture

Тілдер: [English](reference-web-fixture.en.md), [Русский](reference-web-fixture.ru.md), [中文](reference-web-fixture.zh.md), [Қазақша](reference-web-fixture.kk.md)

Reference web fixture PRD ішіндегі "Add Page to Existing Web App" сценарийін жабады. Репозиторийде шағын Next.js-style app `examples/reference-web-app` және AgentSpec `examples/reference-web-add-courses.yaml` бар.

Fixture external npm dependencies қажет етпейді: `npm run build` және `npm run dev` local Node scripts пайдаланады. Бірақ құрылымы app router формасына ұқсайды (`src/app/<route>/page.html`), сондықтан AgentHub route creation, style reuse, scope enforcement, build verification, runtime smoke, memory promotion, report, cost artifacts және WAL replay тексереді.

## Іске қосу

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

Сәтті аяқталса, `$tmp/src/app/courses/page.html` пайда болады. Transaction report `$tmp/.agent/tx/<tx-id>/report.md` ішінде; `verifier.json` `/courses` үшін successful runtime smoke нәтижесін көрсетеді, `cost.json` cost estimates береді, `committed.jsonl` promoted memory сақтайды, ал `wal_replay.json` validated WAL replay сақтайды.

## Нені дәлелдейді

- direct edits алдымен isolated git worktree ішінде орындалады, содан кейін merge болады;
- `scope.allow` тек `src/app/courses/**` рұқсат етеді;
- `scope.deny` dashboard, styles, package және scripts өзгерістерін блоктайды;
- `npm run build` route shape және style reuse тексереді;
- `npm run dev` fixture іске қосады, AgentHub `/courses` route-ын HTTP 200 арқылы тексереді;
- failed scope violations rollback болып, `.agent/memory/failed_attempts.jsonl` ішіне жазылады.
