# Agent Action Language

语言: [English](aal.en.md), [Русский](aal.ru.md), [中文](aal.zh.md), [Қазақша](aal.kk.md)

AAL 是 AgentHub 的简洁 action language。它描述 agent action、scope、verification、runtime smoke checks 和 transaction policy，然后编译为现有的 `AgentSpec` YAML runtime。

## Parse

```bash
agenthub aal parse examples/add-courses.aal
agenthub aal parse examples/add-courses.aal --output tmp/add-courses.yaml
```

命令会把 diagnostics 输出到 stderr，并把 AgentSpec YAML 输出到 stdout 或 `--output`。

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

`workspace`、`goal`、`topology`、`use skill`、`allow`、`deny`、`rules`、`execute`、`verify` 和 `transaction` 会直接映射到 `AgentSpec` 字段。Quoted strings 可以包含空格。以 `#` 或 `//` 开头的行是 comments。

## 示例

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

Parser errors 会包含行号：

```text
error line 2: unsupported AAL statement `mystery`
```

如果存在 `runtime_smoke route` 但没有 `runtime_start`，parser 会给出 warning：route 会写入 AgentSpec，但在定义 runtime startup 之前不会执行。
