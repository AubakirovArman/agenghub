# PRD v4

语言: [English](prd-v4.en.md), [Русский](prd-v4.ru.md), [中文](prd-v4.zh.md), [Қазақша](prd-v4.kk.md)

PRD v4 准备 AgentHub 的第一个 tagged local developer preview：`v0.2.0-local-preview`。

## Scope

- 将 package version 提升到 `0.2.0-local-preview`。
- 用四种语言记录 known limitations。
- 添加 `scripts/dogfood.sh`，用于可重复的 local product checks。
- 添加 `scripts/release-readiness.sh`，用于 release validation、packaging、local install、`version` 和 `doctor`。
- 只有 Linux、macOS 和 Windows CI 全部通过后才发布 GitHub Release assets。
- 在 owner approval 后，将项目从 `UNLICENSED` 改为 Apache-2.0 open source。

## 不在 Scope 内

PRD v4 不添加 hosted SaaS，也不声称拥有完整 security sandbox。这些仍是后续 product 或 hardening tracks。
