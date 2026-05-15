# PRD v3 Task 00 - Product Rename Audit

Status: Done

## Goal

Make the product and repository naming consistent around `AgentHub` / `agenthub`, and remove visible `agenghub` references from product docs.

## Acceptance

- Find remaining `agenghub` references.
- Update product-facing docs to use `AgentHub` or `agenthub`.
- Document the external GitHub repository rename step from `AubakirovArman/agenghub` to `AubakirovArman/agenthub`.
- Keep local build/test behavior unchanged.
- Add evidence and commit the tracker update.

## Completed

- Added four-language repository rename guidance:
  - `docs/repository-rename.en.md`
  - `docs/repository-rename.ru.md`
  - `docs/repository-rename.zh.md`
  - `docs/repository-rename.kk.md`
- Linked the repository rename guidance from all four README files.
- Confirmed the Rust crate and binary already use `agenthub`.
- Confirmed remaining `agenghub` references are limited to rename guidance, this completed task, and historical audit/task text.

## Evidence

- Product-facing docs now state `AgentHub` for the product and `agenthub` for repository/crate/binary/local folder names.
- External GitHub action is documented as:

```text
AubakirovArman/agenghub -> AubakirovArman/agenthub
```

- Local remote update command is documented:

```bash
git remote set-url origin https://github.com/AubakirovArman/agenthub.git
```

## Validation

- `rg -n "agenghub|AubakirovArman/agenghub" . --glob '!target/**' --glob '!.git/**'`
- Remaining matches are intentional: rename instructions, this task evidence, and historical PRD v2 audit context.
