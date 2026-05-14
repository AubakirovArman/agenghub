# AgentHub Plugin Ecosystem

Languages: [English](plugin-ecosystem.en.md), [Русский](plugin-ecosystem.ru.md), [中文](plugin-ecosystem.zh.md), [Қазақша](plugin-ecosystem.kk.md)

## Purpose

Phase 13 introduces a local marketplace/package layer. A package can publish skills, workspace plugin metadata, verifier plugin metadata, and optional signature metadata. Installation copies skills into the project and writes lock files.

## Package Layout

```text
marketplace/skill-packs/content-basic/
  agenthub-plugin.yaml
  skills/content.article_outline/skill.yaml
  schemas/content.yaml
```

## Manifest Example

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

## Install Flow

Inspect a package before installing it:

```bash
agenthub plugins inspect marketplace/skill-packs/content-basic
```

Install and lock it:

```bash
agenthub plugins install marketplace/skill-packs/content-basic --trust local
```

List installed packages:

```bash
agenthub plugins list
```

## Trust Model

`--trust` accepts:

- `local`: package is local project/repo code.
- `trusted`: package comes from a trusted source.
- `untrusted`: package is recorded as untrusted and requires `--allow-untrusted`.

Example:

```bash
agenthub plugins install ./some-package --trust untrusted --allow-untrusted
```

## Lock Files

AgentHub writes two locks:

- `.agent/plugins/installed.json`: installed package id, version, source, trust, installed skills, verifier plugins, workspace plugins, signature metadata.
- `.agent/skills/installed.json`: installed skill id, version, target path, and source package.

These locks make plugin and skill versions reproducible for future transactions.
