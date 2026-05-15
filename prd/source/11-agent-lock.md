# 11. Agent Lock

## 11.1 Назначение

`agent.lock` защищает проект от State Drift.

Он фиксирует устойчивые решения проекта.

## 11.2 Содержимое agent.lock

Пример:

```yaml
project:
  type: code
  stack: nextjs
  router: app
  language: typescript
  styling: tailwind
  ui: shadcn
  orm: prisma
  database: postgres
  package_manager: npm

policies:
  preferred:
    http_client: fetch
    validation: zod
  forbidden:
    - axios
    - clerk

rulesets:
  - core.strict_modularity.v1
  - code.no_secret_leak.v1
  - code.reuse_first.v1

skills:
  code.nextjs.add_page: 1.0.0
  code.ui.shadcn: 1.0.0
  code.prisma.crud: 1.0.0

verifiers:
  default: web_runtime_smoke

commands:
  build: npm run build
  dev: npm run dev
  start: npm run start
```

## 11.3 Правила

* агент не может использовать запрещённые технологии;
* новые зависимости требуют explicit approval или skill permission;
* изменения agent.lock требуют отдельной транзакции;
* при изменении agent.lock все pending transactions должны пройти sync check.

---

