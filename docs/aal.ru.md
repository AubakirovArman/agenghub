# Agent Action Language

Языки: [English](aal.en.md), [Русский](aal.ru.md), [中文](aal.zh.md), [Қазақша](aal.kk.md)

AAL — короткий action language для AgentHub. Он описывает agent action, scope, verification, runtime smoke checks и transaction policy, а затем компилируется в существующий `AgentSpec` YAML runtime.

## Parse

```bash
agenthub aal parse examples/add-courses.aal
agenthub aal parse examples/add-courses.aal --output tmp/add-courses.yaml
```

Команда печатает diagnostics в stderr и выводит AgentSpec YAML в stdout или в `--output`.

## Grammar

```text
change <Name> {
  workspace <workspace.type>
  goal "<human title>"
  topology <topology.kind>
  use skill <skill.id>

  allow edit:
    - "<glob>"
  deny edit:
    - "<glob>"
  rules:
    - <rule_id>
  execute:
    - "<command>"
  verify:
    - profile <profile_id>
    - command "<command>"
    - runtime_start "<command>"
    - runtime_base_url "<url>"
    - runtime_timeout_secs <seconds>
    - runtime_smoke route "<path>" expect <status>
  transaction:
    max_repair_attempts <number>
    approval_required true|false
    on_failure rollback|keep
    on_success commit_code promote_memory
}
```

`workspace`, `goal`, `topology`, `use skill`, `allow`, `deny`, `rules`, `execute`, `verify` и `transaction` напрямую переходят в поля `AgentSpec`. Quoted strings могут содержать пробелы. Строки, начинающиеся с `#` или `//`, считаются comments.

## Пример

```aal
change AddCoursesPage {
  workspace code.git
  goal "Add /courses page"
  use skill code.nextjs.add_page

  allow edit:
    - "src/app/courses/**"
  verify:
    - command "npm run build"
    - runtime_start "npm run dev -- --host 127.0.0.1 --port 3000"
    - runtime_base_url "http://127.0.0.1:3000"
    - runtime_smoke route "/courses" expect 200
  transaction:
    max_repair_attempts 3
    on_failure rollback
    on_success commit_code promote_memory
}
```

## Diagnostics

Parser errors содержат номер строки:

```text
error line 2: unsupported AAL statement `mystery`
```

Если есть `runtime_smoke route`, но нет `runtime_start`, parser выдаёт warning: route будет записан в AgentSpec, но не будет выполняться, пока не задан runtime startup.
