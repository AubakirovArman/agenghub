# Release Engineering

Тілдер: [English](release-engineering.en.md), [Русский](release-engineering.ru.md), [中文](release-engineering.zh.md), [Қазақша](release-engineering.kk.md)

PRD v3 ішінде release engineering өнімнің бір бөлігі болып саналады. Local CLI басқа developer-ге берілмей тұрып тұрақты тексерілуі керек.

## CI

`.github/workflows/ci.yml` Linux, macOS және Windows жүйелерінде іске қосылады:

- `cargo fmt -- --check`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `cargo test --locked`
- `scripts/check-module-size.sh 200`
- `npm --prefix editors/vscode run check`
- `examples/add-courses.aal` үшін AAL parse smoke
- `scripts/smoke-test.sh` арқылы CLI smoke

## Smoke Test

`scripts/smoke-test.sh` уақытша Git project жасайды, AgentHub инициализациялайды, no-commit transaction іске қосады, transaction status тексереді және static dashboard жазады.

Local іске қосу:

```bash
scripts/smoke-test.sh
```

Дайын binary тексеру:

```bash
AGENTHUB_BIN=target/debug/agenthub scripts/smoke-test.sh
```

## Releases

`.github/workflows/release.yml` `v*` tag push болғанда Linux, macOS Intel, macOS Apple Silicon және Windows үшін release binaries жинайды. Asset атаулары:

```text
agenthub-x86_64-unknown-linux-gnu.tar.gz
agenthub-aarch64-apple-darwin.tar.gz
agenthub-x86_64-pc-windows-msvc.zip
```

## Project Metadata

`CHANGELOG.md`, `LICENSE`, `SECURITY.md` және `CONTRIBUTING.md` алғашқы public maintenance surface береді. Project owner open-source немесе commercial license таңдағанға дейін current license `UNLICENSED` болып қалады.
