# 25. Open Questions

## 25.1 Language

* Когда переходить от YAML AgentSpec к собственному AAL syntax?
* Нужно ли делать Tree-sitter grammar?
* Какой минимальный набор инструкций AgentIR?
* Должен ли AgentIR быть человекочитаемым?

## 25.2 Memory

* Какие memory types являются обязательными в core?
* Как лучше делать memory compaction?
* Как измерять retrieval sufficiency?
* Как предотвращать cross-project contamination?

## 25.3 Workspace

* Где граница между workspace rollback и effect rollback?
* Как обрабатывать non-rollbackable effects?
* Когда вводить контейнерную песочницу?

## 25.4 Skills

* Как версионировать skills?
* Как проверять качество внешних skills?
* Нужна ли подпись skill packages?
* Как решать skill conflicts?

## 25.5 Agent Routing

* Как выбирать executor/reviewer/repair model?
* Как учитывать стоимость?
* Как учитывать прошлую успешность модели?
* Как поддерживать login-based CLI tools без нарушения правил сервисов?

## 25.6 Enterprise

* Как хранить traces безопасно?
* Как делать redaction достаточно надёжным?
* Как интегрироваться с secrets managers?
* Как делать policy enforcement централизованным?

---

