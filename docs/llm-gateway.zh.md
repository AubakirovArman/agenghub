# AgentHub LLM Gateway

语言: [English](llm-gateway.en.md), [Русский](llm-gateway.ru.md), [中文](llm-gateway.zh.md), [Қазақша](llm-gateway.kk.md)

## 目的

LLM Gateway 是 model work 的 observability boundary。它记录 planned 和 observed model-call metadata、prompt/context hashes、redacted traces、optional raw traces、token estimates 和 cost estimates。

## 事务 Artifacts

每个事务现在会写入：

```text
.agent/tx/<tx-id>/context_pack.json
.agent/tx/<tx-id>/context_pack_trace.json
.agent/tx/<tx-id>/model_call_metadata.json
.agent/tx/<tx-id>/llm_gateway_summary.json
.agent/tx/<tx-id>/redacted_api.jsonl
.agent/tx/<tx-id>/cost.json
```

`context_pack.json` 和 `redacted_api.jsonl` 默认会被 redacted。

## Raw Debug Mode

只有显式开启时才写入 raw context 和 raw API traces：

```bash
AGENTHUB_RAW_TRACES=1 agenthub run examples/command-task.yaml
```

它会创建：

```text
.agent/tx/<tx-id>/raw_context_pack.json
.agent/tx/<tx-id>/raw_api.jsonl
```

## Cost Estimates

本地 `command` adapter 默认成本为 `0.0`。可以用环境变量设置临时估算：

```bash
AGENTHUB_INPUT_USD_PER_1K=0.001 AGENTHUB_OUTPUT_USD_PER_1K=0.002 agenthub run examples/command-task.yaml
```

估算会写入 `cost.json`，并在 `report.md` 中汇总。
