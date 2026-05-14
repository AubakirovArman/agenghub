# AgentHub LLM Gateway

Languages: [English](llm-gateway.en.md), [Русский](llm-gateway.ru.md), [中文](llm-gateway.zh.md), [Қазақша](llm-gateway.kk.md)

## Purpose

The LLM Gateway is the observability boundary for model work. It records planned and observed model-call metadata, prompt/context hashes, redacted traces, optional raw traces, token estimates, and cost estimates.

## Transaction Artifacts

Every transaction now writes:

```text
.agent/tx/<tx-id>/context_pack.json
.agent/tx/<tx-id>/context_pack_trace.json
.agent/tx/<tx-id>/model_call_metadata.json
.agent/tx/<tx-id>/llm_gateway_summary.json
.agent/tx/<tx-id>/redacted_api.jsonl
.agent/tx/<tx-id>/cost.json
```

`context_pack.json` and `redacted_api.jsonl` are redacted by default.

## Raw Debug Mode

Raw context and raw API traces are written only when explicitly enabled:

```bash
AGENTHUB_RAW_TRACES=1 agenthub run examples/command-task.yaml
```

That creates:

```text
.agent/tx/<tx-id>/raw_context_pack.json
.agent/tx/<tx-id>/raw_api.jsonl
```

## Cost Estimates

Local `command` adapter calls cost `0.0` by default. To configure a temporary estimate:

```bash
AGENTHUB_INPUT_USD_PER_1K=0.001 AGENTHUB_OUTPUT_USD_PER_1K=0.002 agenthub run examples/command-task.yaml
```

The estimate is stored in `cost.json` and summarized in `report.md`.
