# 17. LLM Gateway and Observability

## 17.1 Назначение

LLM Gateway — чёрный ящик системы.

Он логирует:

* requests;
* responses;
* context pack;
* prompt hashes;
* skill ids;
* memory ids;
* model names;
* token usage;
* cost;
* latency;
* errors;
* redacted traces.

## 17.2 Redaction

Нельзя бездумно хранить secrets.

Redaction должен удалять:

* API keys;
* tokens;
* passwords;
* `.env` values;
* private keys;
* credentials;
* database URLs;
* OAuth secrets.

Хранить можно:

```text
raw_api.jsonl — только в local/private debug mode
redacted_api.jsonl — безопасный trace по умолчанию
context_pack.json — с redacted sensitive values
```

## 17.3 Cost Profiler

AgentHub должен показывать стоимость транзакции.

Пример:

```text
Transaction tx-123 SUCCESS
Time: 45s

Cost Breakdown:
- Intent Normalization: $0.001
- Context Pack Build: local
- Code Generation: $0.040
- Repair Loop: $0.015
- Review: $0.008
Total: $0.064
Tokens: 14,200
```

## 17.4 Transaction Report

Файл:

```text
.agent/tx/tx-123/report.md
```

Содержит:

* task;
* status;
* base commit;
* final commit;
* changed files;
* diff summary;
* verifier results;
* repair attempts;
* sync check;
* diff guard;
* memory promotion;
* failed attempt fingerprint;
* cost;
* duration;
* human actions required.

---

