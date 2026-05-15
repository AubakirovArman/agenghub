# PRD v3 Task 00 - Product Rename Audit

Status: Done

## Goal

Make the product and repository naming consistent around `AgentHub` / `agenthub`, and remove visible legacy typo-name references from product docs.

## Acceptance

- Find remaining legacy typo-name references.
- Update product-facing docs to use `AgentHub` or `agenthub`.
- Document and verify the canonical GitHub repository name `AubakirovArman/agenthub`.
- Keep local build/test behavior unchanged.
- Add evidence and commit the tracker update.

## Completed

- Added four-language repository naming guidance:
  - `docs/repository-rename.en.md`
  - `docs/repository-rename.ru.md`
  - `docs/repository-rename.zh.md`
  - `docs/repository-rename.kk.md`
- Linked the repository naming guidance from all four README files.
- Confirmed the Rust crate and binary already use `agenthub`.
- Confirmed the GitHub repository and local remote use `AubakirovArman/agenthub`.

## Evidence

- Product-facing docs state `AgentHub` for the product and `agenthub` for repository/crate/binary/local folder names.
- Canonical GitHub repository:

```text
AubakirovArman/agenthub
```

- Local remote update command:

```bash
git remote set-url origin https://github.com/AubakirovArman/agenthub.git
```

## Validation

- `git remote -v`
- Repository old-name scan was rerun after canonicalization.
- Remaining legacy typo-name references are historical PRD v2 audit context only.
