# 12. Skill Registry

## 12.1 Назначение

Skill — это пакет доменного опыта.

Skill не должен быть просто prompt-фрагментом. Он должен описывать:

* inputs;
* outputs;
* required workspace;
* memory schemas;
* actions;
* policies;
* verifiers;
* rollback hints;
* common errors;
* prompt fragments;
* examples;
* dependencies.

## 12.2 Skill Manifest

Пример:

```yaml
skill:
  id: code.nextjs.add_page
  version: 1.0.0
  description: Adds a page to a Next.js App Router project.

inputs:
  route: string
  style_source: optional<string>

requires:
  workspace: code.git
  memory_schema:
    - code.routes
    - code.components

provides:
  actions:
    - inspect_routes
    - generate_page
    - update_navigation_optional

policies:
  max_files_changed: 8
  allow_package_change: false
  require_scope: true

verifiers:
  - npm_build
  - runtime_route_smoke

common_errors:
  - missing_use_client
  - wrong_app_router_path
  - component_import_not_exported
```

## 12.3 Skill Types

### Code Skills

* add_page;
* auth;
* crud;
* dashboard;
* api_route;
* database_model;
* migration;
* file_upload;
* email;
* tests;
* refactor;
* bugfix.

### Design Skills

* premium_dark_saas;
* soft_ui;
* mobile_first;
* dashboard_layout;
* landing_page;
* chart_design;
* shadcn_ui;
* responsive_layout.

### Infra Skills

* terraform_module;
* aws_s3;
* docker_compose;
* ci_cd;
* secrets_policy;
* deployment_check.

### Data Skills

* csv_cleaning;
* notebook_analysis;
* feature_engineering;
* model_eval;
* data_quality_report.

### Content Skills

* youtube_script;
* tiktok_short;
* brand_voice;
* article_outline;
* storytelling;
* tts_prompt.

## 12.4 Progressive Disclosure

Skill Registry должен подгружать только нужные skills.

Не надо давать агенту все правила сразу.

Пример:

```text
Task: add /courses page
Active skills:
  - code.nextjs.add_page
  - design.reuse_existing_style
  - verifier.web_runtime_smoke
```

Не активируются:

```text
- auth
- payments
- terraform
- youtube_script
- ml_training
```

---

