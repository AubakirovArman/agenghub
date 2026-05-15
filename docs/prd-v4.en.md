# PRD v4

Languages: [English](prd-v4.en.md), [Русский](prd-v4.ru.md), [中文](prd-v4.zh.md), [Қазақша](prd-v4.kk.md)

PRD v4 prepares AgentHub for the first tagged local developer preview: `v0.2.0-local-preview`.

## Scope

- Bump the package version to `0.2.0-local-preview`.
- Document known limitations in four languages.
- Add `scripts/dogfood.sh` for repeatable local product checks.
- Add `scripts/release-readiness.sh` for release validation, packaging, local install, `version`, and `doctor`.
- Publish GitHub Release assets only after CI passes on Linux, macOS, and Windows.
- Switch the project from `UNLICENSED` to Apache-2.0 open source after owner approval.

## Not In Scope

PRD v4 does not add hosted SaaS or claim a full security sandbox. Those remain later product or hardening tracks.
