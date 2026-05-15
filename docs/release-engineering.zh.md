# Release Engineering

语言: [English](release-engineering.en.md), [Русский](release-engineering.ru.md), [中文](release-engineering.zh.md), [Қазақша](release-engineering.kk.md)

在 PRD v3 中，release engineering 是产品能力的一部分。本地 CLI 在交给其他开发者之前，必须能被稳定验证。

## CI

`.github/workflows/ci.yml` 在 Linux、macOS 和 Windows 上运行：

- `cargo fmt -- --check`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `cargo test --locked`
- `scripts/check-module-size.sh 200`
- `npm --prefix editors/vscode run check`
- 对 `examples/add-courses.aal` 做 AAL parse smoke
- 通过 `scripts/smoke-test.sh` 做 CLI smoke

## Smoke Test

`scripts/smoke-test.sh` 会创建临时 Git 项目，初始化 AgentHub，运行 no-commit transaction，检查 transaction status，并生成 static dashboard。

本地运行：

```bash
scripts/smoke-test.sh
```

测试已构建 binary：

```bash
AGENTHUB_BIN=target/debug/agenthub scripts/smoke-test.sh
```

## Releases

`.github/workflows/release.yml` 在推送 `v*` tag 时为 Linux、macOS Intel、macOS Apple Silicon 和 Windows 构建 release binaries。资产命名示例：

```text
agenthub-x86_64-unknown-linux-gnu.tar.gz
agenthub-aarch64-apple-darwin.tar.gz
agenthub-x86_64-pc-windows-msvc.zip
```

## Project Metadata

`CHANGELOG.md`、`LICENSE`、`SECURITY.md` 和 `CONTRIBUTING.md` 构成第一层公开维护界面。当前 license 仍为 `UNLICENSED`，直到项目所有者选择 open-source 或 commercial license。
