# 15. Verifier Layer

## 15.1 Назначение

Verifier проверяет не только синтаксис, а реальную пригодность результата.

## 15.2 Verifier Profiles

### 15.2.1 code_build

* package install check;
* typecheck;
* build;
* lint optional.

### 15.2.2 web_runtime_smoke

* build;
* start server;
* wait for readiness;
* curl changed routes;
* expected status;
* kill process tree.

### 15.2.3 backend_tdd

* write/verify tests;
* run unit tests;
* run integration tests;
* check API responses.

### 15.2.4 db_migration

* schema diff;
* migration dry run;
* rollback migration if supported;
* seed check.

### 15.2.5 infra_plan

* terraform fmt;
* terraform validate;
* terraform plan;
* cost estimate;
* security policy check.

### 15.2.6 data_quality

* notebook executes;
* null checks;
* schema checks;
* metric thresholds;
* artifact generation.

### 15.2.7 content_quality

* length check;
* tone check;
* repetition check;
* structure check;
* banned phrase check;
* factuality check if needed.

## 15.3 Runtime Smoke Example

For web route:

```text
npm run build
npm run start -- --port 0
wait_for_ready
GET /courses
expect 200
kill process tree
```

Protected routes can expect:

```text
/dashboard → 302 to /login
```

---

