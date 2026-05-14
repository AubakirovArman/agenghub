# AgentHub Plugin Ecosystem

语言: [English](plugin-ecosystem.en.md), [Русский](plugin-ecosystem.ru.md), [中文](plugin-ecosystem.zh.md), [Қазақша](plugin-ecosystem.kk.md)

## 目的

Phase 13 引入本地 marketplace/package layer。一个 package 可以发布 skills、workspace plugin metadata、verifier plugin metadata 和 optional signature metadata。安装时会把 skills 复制到项目中，并写入 lock files。

## Package 结构

```text
marketplace/skill-packs/content-basic/
  agenthub-plugin.yaml
  skills/content.article_outline/skill.yaml
  schemas/content.yaml
```

## Manifest 示例

```yaml
package:
  id: agenthub.content.basic
  version: 0.1.0
  description: Basic content authoring skill package.
  author: AgentHub

skills:
  - path: skills/content.article_outline/skill.yaml

workspace_plugins:
  - id: content.git
    description: Git-backed content workspace profile.
    schema_path: schemas/content.yaml

verifier_plugins:
  - id: content.markdown_presence
    description: Checks that a markdown artifact exists and is non-empty.
    command: test -s "${CONTENT_FILE}"

signature: null
```

## 安装流程

安装前检查 package：

```bash
agenthub plugins inspect marketplace/skill-packs/content-basic
```

安装并锁定版本：

```bash
agenthub plugins install marketplace/skill-packs/content-basic --trust local
```

查看已安装 packages：

```bash
agenthub plugins list
```

## Trust Model

`--trust` 支持：

- `local`: package 来自本地项目或仓库。
- `trusted`: package 来自可信来源。
- `untrusted`: package 记录为不可信，需要 `--allow-untrusted`。

示例：

```bash
agenthub plugins install ./some-package --trust untrusted --allow-untrusted
```

## Lock Files

AgentHub 写入两个 lock files：

- `.agent/plugins/installed.json`: package id、version、source、trust、installed skills、verifier plugins、workspace plugins、signature metadata。
- `.agent/skills/installed.json`: skill id、version、target path 和 source package。

这些 lock files 让 plugin 和 skill versions 在未来事务中可复现。
