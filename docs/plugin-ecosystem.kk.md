# AgentHub Plugin Ecosystem

Тілдер: [English](plugin-ecosystem.en.md), [Русский](plugin-ecosystem.ru.md), [中文](plugin-ecosystem.zh.md), [Қазақша](plugin-ecosystem.kk.md)

## Мақсаты

Phase 13 жергілікті marketplace/package layer қосады. Package skills, workspace plugin metadata, verifier plugin metadata және optional signature metadata жариялай алады. Орнату кезінде skills жобаға көшіріледі және lock files жазылады.

## Package құрылымы

```text
marketplace/skill-packs/content-basic/
  agenthub-plugin.yaml
  skills/content.article_outline/skill.yaml
  schemas/content.yaml
```

## Manifest мысалы

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

Орнату алдында package тексеру:

```bash
agenthub plugins inspect marketplace/skill-packs/content-basic
```

Орнатып, нұсқаларды lock жасау:

```bash
agenthub plugins install marketplace/skill-packs/content-basic --trust local
```

Орнатылған packages тізімі:

```bash
agenthub plugins list
```

## Trust model

`--trust` мәндері:

- `local`: package жергілікті project/repo ішінен.
- `trusted`: package сенімді source ішінен.
- `untrusted`: package сенімсіз деп белгіленеді және `--allow-untrusted` талап етеді.

Мысал:

```bash
agenthub plugins install ./some-package --trust untrusted --allow-untrusted
```

## Lock files

AgentHub екі lock file жазады:

- `.agent/plugins/installed.json`: package id, version, source, trust, installed skills, verifier plugins, workspace plugins, signature metadata.
- `.agent/skills/installed.json`: skill id, version, target path және source package.

Бұл lock files болашақ транзакциялар үшін plugin және skill versions қайталанатын етеді.
