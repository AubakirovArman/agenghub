# Repository Rename

Языки: [English](repository-rename.en.md), [Русский](repository-rename.ru.md), [中文](repository-rename.zh.md), [Қазақша](repository-rename.kk.md)

Название продукта — `AgentHub`. Repository, crate, binary и local folder должны использовать lowercase `agenthub`.

Рекомендуемые имена:

- Product: `AgentHub`
- GitHub repository: `AubakirovArman/agenthub`
- Rust crate: `agenthub`
- Binary: `agenthub`
- Local folder: `agenthub`

## GitHub Rename Step

Переименование GitHub repository делается вне codebase:

```text
AubakirovArman/agenghub -> AubakirovArman/agenthub
```

После rename обнови local remote:

```bash
git remote set-url origin https://github.com/AubakirovArman/agenthub.git
```

В этом repository Rust crate и binary уже называются `agenthub`.
