# AgentHub LLM Gateway

Языки: [English](llm-gateway.en.md), [Русский](llm-gateway.ru.md), [中文](llm-gateway.zh.md), [Қазақша](llm-gateway.kk.md)

## Назначение

LLM Gateway — observability boundary для model work. Он записывает planned и observed model-call metadata, prompt/context hashes, redacted traces, optional raw traces, token estimates и cost estimates.

## Артефакты транзакции

Каждая транзакция теперь пишет:

```text
.agent/tx/<tx-id>/context_pack.json
.agent/tx/<tx-id>/context_pack_trace.json
.agent/tx/<tx-id>/model_call_metadata.json
.agent/tx/<tx-id>/llm_gateway_summary.json
.agent/tx/<tx-id>/redacted_api.jsonl
.agent/tx/<tx-id>/cost.json
```

`context_pack.json` и `redacted_api.jsonl` по умолчанию проходят redaction.

## Raw debug mode

Raw context и raw API traces пишутся только при явном включении:

```bash
AGENTHUB_RAW_TRACES=1 agenthub run examples/command-task.yaml
```

Это создаёт:

```text
.agent/tx/<tx-id>/raw_context_pack.json
.agent/tx/<tx-id>/raw_api.jsonl
```

## Cost estimates

Local `command` adapter по умолчанию стоит `0.0`. Временную оценку можно задать так:

```bash
AGENTHUB_INPUT_USD_PER_1K=0.001 AGENTHUB_OUTPUT_USD_PER_1K=0.002 agenthub run examples/command-task.yaml
```

Оценка сохраняется в `cost.json` и показывается в `report.md`.
