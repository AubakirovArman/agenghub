# Repository Rename

语言: [English](repository-rename.en.md), [Русский](repository-rename.ru.md), [中文](repository-rename.zh.md), [Қазақша](repository-rename.kk.md)

产品名是 `AgentHub`。Repository、crate、binary 和 local folder 应使用 lowercase `agenthub`。

推荐名称:

- Product: `AgentHub`
- GitHub repository: `AubakirovArman/agenthub`
- Rust crate: `agenthub`
- Binary: `agenthub`
- Local folder: `agenthub`

## GitHub Rename Step

GitHub repository rename 需要在 codebase 外部完成:

```text
AubakirovArman/agenghub -> AubakirovArman/agenthub
```

GitHub rename 后更新 local remote:

```bash
git remote set-url origin https://github.com/AubakirovArman/agenthub.git
```

当前 repository 已经使用 `agenthub` 作为 Rust crate 和 binary 名称。
