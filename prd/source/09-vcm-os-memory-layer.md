# 9. VCM-OS Memory Layer

## 9.1 Назначение

VCM-OS — это типизированная память проекта, процесса и агента.

Она заменяет простую историю чата структурированной памятью:

```text
history chat = raw noise
VCM memory = typed project state
```

## 9.2 Core Memory Types

Базовые типы памяти:

* `project_fact`;
* `decision`;
* `requirement`;
* `constraint`;
* `procedure`;
* `error`;
* `failed_attempt`;
* `code_change`;
* `artifact`;
* `tool_result`;
* `style_rule`;
* `preference`;
* `experiment`;
* `checkpoint`;
* `assumption`;
* `risk`;
* `open_question`.

## 9.3 Domain Schemas

VCM-OS должна быть schema-driven.

### 9.3.1 Code Project Schema

* architecture_decision;
* route;
* component;
* api_endpoint;
* db_model;
* dependency_policy;
* coding_rule;
* build_error;
* test_policy;
* style_rule;
* forbidden_library;
* package_manager;
* env_requirement.

### 9.3.2 Infra Project Schema

* environment;
* terraform_module;
* cloud_resource;
* secret_policy;
* cost_constraint;
* deployment_issue;
* state_backend;
* rollback_procedure.

### 9.3.3 Data/ML Schema

* dataset;
* data_quality_rule;
* experiment_run;
* metric;
* hyperparameter;
* model_checkpoint;
* feature_pipeline;
* training_error;
* evaluation_result.

### 9.3.4 Content Schema

* tone_of_voice;
* audience_profile;
* content_format;
* used_topic;
* script_template;
* tts_voice;
* visual_style;
* publishing_rule;
* banned_repetition;
* brand_rule.

### 9.3.5 Media Schema

* scene;
* shot;
* prompt_template;
* asset;
* voice_track;
* render_setting;
* video_style;
* platform_requirement.

## 9.4 Memory Lifecycle

```text
raw event
  ↓
classification
  ↓
staging memory
  ↓
verification result
  ↓
promotion or failed_attempt log
  ↓
compaction / summarization
  ↓
retrieval pack
```

## 9.5 Staging Memory vs Committed Memory

Во время транзакции все новые memory events пишутся в staging.

```text
memory_staging.jsonl
```

После успешной верификации:

```text
memory_staging → committed memory
```

После провала:

```text
memory_staging discarded
failed_attempt written separately
```

## 9.6 Failed Attempt Memory

Failed attempt должен хранить:

* task id;
* intent;
* skills;
* context pack hash;
* model used;
* error fingerprint;
* verifier logs summary;
* failed diff summary;
* avoid_next_time;
* whether human intervention needed.

Пример:

```json
{
  "type": "failed_attempt",
  "task": "AddCoursesPage",
  "fingerprint": "missing_export_CourseCard",
  "reason": "npm run build failed after 3 repair attempts",
  "avoid_next_time": [
    "Check component exports before importing",
    "Do not create imports from non-exported files"
  ]
}
```

## 9.7 Memory Compaction

Со временем память растёт. Нужен compaction.

Принцип:

```text
raw logs → current truth
```

Пример:

```text
Added route /courses
Modified route /courses
Deleted old route /courses-v1
```

Сжимается в:

```text
Current route /courses exists at src/app/courses/page.tsx and uses CoursesGrid.
```

Compaction должен учитывать:

* superseded decisions;
* stale facts;
* contradictions;
* failed attempts;
* last verified commit;
* schema validity.

---

