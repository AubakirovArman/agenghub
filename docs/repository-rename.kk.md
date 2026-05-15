# Repository Rename

Тілдер: [English](repository-rename.en.md), [Русский](repository-rename.ru.md), [中文](repository-rename.zh.md), [Қазақша](repository-rename.kk.md)

Product name — `AgentHub`. Repository, crate, binary және local folder lowercase `agenthub` қолдануы керек.

Ұсынылатын атаулар:

- Product: `AgentHub`
- GitHub repository: `AubakirovArman/agenthub`
- Rust crate: `agenthub`
- Binary: `agenthub`
- Local folder: `agenthub`

## GitHub Rename Step

GitHub repository rename codebase сыртында жасалады:

```text
AubakirovArman/agenghub -> AubakirovArman/agenthub
```

GitHub rename кейін local remote жаңарту:

```bash
git remote set-url origin https://github.com/AubakirovArman/agenthub.git
```

Бұл repository ішінде Rust crate және binary қазірдің өзінде `agenthub` деп аталады.
