# Repository Rename

Languages: [English](repository-rename.en.md), [Русский](repository-rename.ru.md), [中文](repository-rename.zh.md), [Қазақша](repository-rename.kk.md)

The product name is `AgentHub`. The repository, crate, binary, and local folder should use lowercase `agenthub`.

Recommended names:

- Product: `AgentHub`
- GitHub repository: `AubakirovArman/agenthub`
- Rust crate: `agenthub`
- Binary: `agenthub`
- Local folder: `agenthub`

## GitHub Rename Step

Rename the GitHub repository outside the codebase:

```text
AubakirovArman/agenghub -> AubakirovArman/agenthub
```

After GitHub rename, update local remotes:

```bash
git remote set-url origin https://github.com/AubakirovArman/agenthub.git
```

This repository already uses `agenthub` for the Rust crate and binary.
