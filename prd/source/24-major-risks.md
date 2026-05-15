# 24. Major Risks

## 24.1 Over-Abstraction Risk

Если слишком рано делать универсальность для всех доменов, система может стать сложной и бесполезной.

Mitigation:

```text
Universal core, narrow first reference domain.
```

## 24.2 Weak Verifier Risk

Если verifier слабый, плохой код может пройти.

Mitigation:

* verifier profiles;
* runtime smoke;
* TDD where appropriate;
* policy checks;
* reviewer agents.

## 24.3 Memory Pollution Risk

Если память обновляется без проверки, система деградирует.

Mitigation:

* staging memory;
* promotion only on success;
* failed attempt separation.

## 24.4 Security Risk

Агент может запустить опасную команду или раскрыть secrets.

Mitigation:

* command policy;
* redaction;
* workspace isolation;
* approvals;
* sandbox levels.

## 24.5 Cost Explosion Risk

Сложные topology graphs могут сжигать много токенов.

Mitigation:

* cost profiler;
* budget policies;
* cheap model routing;
* local model routing;
* context minimization.

## 24.6 Skill Quality Risk

Плохие skills могут ухудшать результат.

Mitigation:

* skill versioning;
* skill tests;
* trust model;
* project lock;
* telemetry.

---

