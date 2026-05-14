# AgentHub Plugin Ecosystem

Языки: [English](plugin-ecosystem.en.md), [Русский](plugin-ecosystem.ru.md), [中文](plugin-ecosystem.zh.md), [Қазақша](plugin-ecosystem.kk.md)

## Назначение

Phase 13 добавляет локальный marketplace/package layer. Пакет может публиковать skills, workspace plugin metadata, verifier plugin metadata и optional signature metadata. Установка копирует skills в проект и пишет lock-файлы.

## Структура пакета

```text
marketplace/skill-packs/content-basic/
  agenthub-plugin.yaml
  skills/content.article_outline/skill.yaml
  schemas/content.yaml
```

## Пример manifest

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

## Install flow

Проверить пакет перед установкой:

```bash
agenthub plugins inspect marketplace/skill-packs/content-basic
```

Установить и зафиксировать версии:

```bash
agenthub plugins install marketplace/skill-packs/content-basic --trust local
```

Показать установленные пакеты:

```bash
agenthub plugins list
```

## Trust model

`--trust` принимает:

- `local`: пакет находится в локальном проекте или репозитории.
- `trusted`: пакет получен из доверенного источника.
- `untrusted`: пакет помечается как недоверенный и требует `--allow-untrusted`.

Пример:

```bash
agenthub plugins install ./some-package --trust untrusted --allow-untrusted
```

## Lock files

AgentHub пишет два lock-файла:

- `.agent/plugins/installed.json`: package id, version, source, trust, installed skills, verifier plugins, workspace plugins, signature metadata.
- `.agent/skills/installed.json`: skill id, version, target path и source package.

Эти lock-файлы делают plugin и skill versions воспроизводимыми для будущих транзакций.
