# PRD v2 Task 00 — Brand Audit

Status: Done

## Goal

Verify that the product brand is consistently `AgentHub` and avoid mixing it with legacy typo-name workspace paths.

## Completed

- Confirmed the Rust crate is named `agenthub`.
- Confirmed README files and VS Code extension display name use `AgentHub`.
- Confirmed the CLI examples use `agenthub`.
- Left the old local folder name unchanged at the time because it was an external workspace path, not a product string inside the repository.

## Evidence

- Implementation commit: `5531f56`.
- Checks: `git diff --check`.
