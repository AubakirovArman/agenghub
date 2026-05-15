# AgentHub Plugin Ecosystem

Языки: [English](plugin-ecosystem.en.md), [Русский](plugin-ecosystem.ru.md), [中文](plugin-ecosystem.zh.md), [Қазақша](plugin-ecosystem.kk.md)

## Назначение

Phase 13 добавляет локальный marketplace/package layer. Пакет может публиковать skills, workspace plugin metadata, verifier plugin metadata и optional signature metadata. Установка копирует skills в проект, проверяет referenced files и пишет lock-файлы.

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
    kind: git
    profile: content
    schema_path: schemas/content.yaml
    capabilities:
      - markdown
      - frontmatter

verifier_plugins:
  - id: content.markdown_presence
    description: Checks that a markdown artifact exists and is non-empty.
    command: test -s "${CONTENT_FILE}"
    profiles:
      - content_quality
    artifact_globs:
      - content/**/*.md
    timeout_secs: 30

signature:
  algorithm: none
  signer: AgentHub local marketplace
  value: unsigned
```

## Authoring flow

Внешний автор может создать publishable package:

```bash
agenthub plugins scaffold marketplace/skill-packs/my-pack \
  --package-id com.example.my-pack \
  --skill-id com.example.article_outline \
  --description "Article outline skill" \
  --author "Example Author"
```

Затем нужно отредактировать `agenthub-plugin.yaml`, добавить workspace или verifier metadata при необходимости и запустить:

```bash
agenthub plugins inspect marketplace/skill-packs/my-pack
```

`inspect` проверяет `package.version` как `major.minor.patch`, safe relative paths, существование skill manifests и workspace schemas.

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

`signature` — optional metadata. Phase 13 записывает её в lock file; cryptographic verification оставлена для следующего слоя, поэтому enforcement сейчас идёт через `--trust`.

## Lock files

AgentHub пишет два lock-файла:

- `.agent/plugins/installed.json`: package id, version, source, trust, installed skills, verifier plugin metadata, workspace plugin metadata, signature metadata.
- `.agent/skills/installed.json`: skill id, version, target path и source package.

Эти lock-файлы делают plugin и skill versions воспроизводимыми для будущих транзакций.
