# 8. AgentHub Layers

## 8.1 Interface Layer

### 8.1.1 CLI / SLI

Первичный интерфейс.

Пример команд:

```bash
agenthub init
agenthub ask "создай веб-приложение для курсов"
agenthub run task.yaml
agenthub tx status
agenthub tx report tx-123
agenthub tx rollback tx-123
agenthub memory inspect
agenthub skills list
agenthub workspace scan
```

### 8.1.2 TUI

Будущий terminal dashboard:

* текущие транзакции;
* состояние DAG;
* логи verifier’ов;
* cost breakdown;
* failed attempts;
* memory changes;
* approval prompts.

### 8.1.3 Web Dashboard

Для визуальной работы:

* список проектов;
* memory graph;
* agent traces;
* transaction timeline;
* skill registry;
* policies;
* cost analytics;
* workspace reports.

### 8.1.4 VS Code Extension

Функции:

* запуск AgentHub из IDE;
* просмотр transaction report;
* visual diff;
* approval UI;
* memory facts panel;
* skill selection;
* AgentSpec editing;
* diagnostics for AAL.

---

## 8.2 Intent Layer

### 8.2.1 Natural Language Input

Пользователь может писать обычным языком:

```text
Хочу приложение, где пользователи могут загружать курсы, писать блоги и новости, а админ всё модерирует.
```

### 8.2.2 Intent Normalizer

Преобразует “водяной” запрос в структурированное намерение:

```json
{
  "intent": "create_app",
  "app_type": "content_learning_platform",
  "modules": ["auth", "courses", "blog", "news", "admin"],
  "unknowns": ["payments", "storage", "roles", "deployment"]
}
```

### 8.2.3 Clarification Engine

Система должна задавать вопросы только по блокирующим решениям.

Принцип:

```text
Ask only what changes architecture.
Use defaults for non-critical choices.
```

Пример вопросов:

* это новый проект или существующий;
* веб, desktop, mobile или backend API;
* нужен ли auth;
* нужны ли payments;
* где хранить файлы;
* какой стек использовать;
* можно ли использовать defaults.

### 8.2.4 Default Resolver

Если пользователь не уточняет, система может применять project defaults.

Пример defaults для code profile:

```text
frontend: Next.js
language: TypeScript
styling: Tailwind
ui: shadcn
database: PostgreSQL
orm: Prisma
auth: custom/session cookie
package manager: npm/pnpm detected from project
```

---

## 8.3 Language Layer

## 8.3.1 AAL — Agent Action Language

AAL описывает не классический код, а действие агента.

Он должен уметь описывать:

* goal;
* workspace;
* task type;
* scope;
* constraints;
* required skills;
* memory policy;
* transaction policy;
* verifier profile;
* rollback policy;
* agent topology;
* artifact outputs;
* approval requirements.

Пример будущего AAL:

```text
change AddCoursesPage {
  workspace code.git
  goal "Add /courses page"

  use skill code.nextjs.add_page
  use skill code.ui.reuse_existing_style

  allow edit:
    - "src/app/courses/**"
    - "src/components/courses/**"

  deny edit:
    - "src/auth/**"
    - "prisma/schema.prisma"

  rules:
    - R_MOD_200
    - R_REUSE_FIRST
    - R_NO_SECRET
    - R_SCOPE_ONLY

  verify:
    - npm_build
    - runtime_smoke route "/courses" expect 200

  transaction:
    isolation git_worktree
    max_repair_attempts 3
    on_failure rollback
    on_success commit_code promote_memory
}
```

## 8.3.2 AgentSpec

На ранних этапах AgentSpec может быть YAML/JSON.

Пример:

```yaml
task:
  id: add_courses_page
  type: code.add_page
  target: /courses

workspace:
  type: code.git
  isolation: git_worktree

skills:
  - code.nextjs.add_page
  - code.ui.reuse_existing_style

scope:
  allow:
    - src/app/courses/**
    - src/components/courses/**
  deny:
    - src/auth/**
    - prisma/schema.prisma

rules:
  - R_MOD_200
  - R_REUSE_FIRST
  - R_SCOPE_ONLY

verify:
  profile: web_runtime_smoke
  commands:
    - npm run build
  routes:
    - path: /courses
      expect: 200

transaction:
  max_repair_attempts: 3
  rollback_on_failure: true
  memory_promotion: on_success
```

## 8.3.3 AgentIR

AgentIR — компактное промежуточное представление.

Пример:

```text
TX code.add_page /courses
WS code.git iso=worktree
SKILL code.nextjs.add_page code.ui.reuse_style
RULE MOD_200 REUSE_FIRST SCOPE_ONLY
ALLOW src/app/courses/** src/components/courses/**
DENY src/auth/** prisma/schema.prisma
VERIFY npm_build route_smoke:/courses:200
REPAIR max=3
MEM promote_on_success failed_log_on_rollback
```

AgentIR понимает AgentHub, а не обязательно сама LLM. На раннем этапе AgentHub компилирует AgentIR в понятные prompt/context/tool calls.

---

