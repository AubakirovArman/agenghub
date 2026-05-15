# Reference Web Fixture

语言: [English](reference-web-fixture.en.md), [Русский](reference-web-fixture.ru.md), [中文](reference-web-fixture.zh.md), [Қазақша](reference-web-fixture.kk.md)

Reference web fixture 用来关闭 PRD 场景 "Add Page to Existing Web App"。仓库提供一个小型 Next.js-style 应用 `examples/reference-web-app`，以及 AgentSpec `examples/reference-web-add-courses.yaml`。

该 fixture 不依赖外部 npm package：`npm run build` 和 `npm run dev` 都使用本地 Node scripts。但目录仍采用 app-router 形状（`src/app/<route>/page.html`），因此 AgentHub 可以验证 route creation、style reuse、scope enforcement、build verification、runtime smoke、memory promotion、report、cost artifacts 和 WAL replay。

## 运行

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

成功后会生成 `$tmp/src/app/courses/page.html`。事务报告位于 `$tmp/.agent/tx/<tx-id>/report.md`；`verifier.json` 包含 `/courses` 的成功 runtime smoke 结果，`cost.json` 包含 cost estimates，`committed.jsonl` 包含 promoted memory，`wal_replay.json` 包含已验证的 WAL replay。

## 验证内容

- 直接编辑先发生在 isolated git worktree 中，再 merge；
- `scope.allow` 只允许 `src/app/courses/**`；
- `scope.deny` 阻止修改 dashboard、styles、package 和 scripts；
- `npm run build` 验证 route shape 和 style reuse；
- `npm run dev` 启动 fixture，AgentHub 用 HTTP 200 检查 `/courses`；
- failed scope violations 会 rollback，并写入 `.agent/memory/failed_attempts.jsonl`。
