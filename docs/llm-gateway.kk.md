# AgentHub LLM Gateway

Тілдер: [English](llm-gateway.en.md), [Русский](llm-gateway.ru.md), [中文](llm-gateway.zh.md), [Қазақша](llm-gateway.kk.md)

## Мақсаты

LLM Gateway — model work үшін observability boundary. Ол planned және observed model-call metadata, prompt/context hashes, redacted traces, optional raw traces, token estimates және cost estimates жазады.

## Транзакция artifacts

Әр транзакция қазір жазады:

```text
.agent/tx/<tx-id>/context_pack.json
.agent/tx/<tx-id>/context_pack_trace.json
.agent/tx/<tx-id>/model_call_metadata.json
.agent/tx/<tx-id>/llm_gateway_summary.json
.agent/tx/<tx-id>/redacted_api.jsonl
.agent/tx/<tx-id>/cost.json
```

`context_pack.json` және `redacted_api.jsonl` default бойынша redacted болады.

## Raw debug mode

Raw context және raw API traces тек нақты қосылғанда жазылады:

```bash
AGENTHUB_RAW_TRACES=1 agenthub run examples/command-task.yaml
```

Ол мыналарды жасайды:

```text
.agent/tx/<tx-id>/raw_context_pack.json
.agent/tx/<tx-id>/raw_api.jsonl
```

## Cost estimates

Local `command` adapter default бойынша `0.0` тұрады. Уақытша estimate былай беріледі:

```bash
AGENTHUB_INPUT_USD_PER_1K=0.001 AGENTHUB_OUTPUT_USD_PER_1K=0.002 agenthub run examples/command-task.yaml
```

Estimate `cost.json` ішіне жазылады және `report.md` ішінде көрсетіледі.
