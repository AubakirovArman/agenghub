# AgentHub Agent Adapters

Языки: [English](agent-adapters.en.md), [Русский](agent-adapters.ru.md), [中文](agent-adapters.zh.md), [Қазақша](agent-adapters.kk.md)

## Назначение

В AgentHub v0.4 adapter layer уходит от внешних AI CLI. User-facing AI providers теперь API-native `deepseek` и `kimi`; `command` остается встроенным deterministic runner для transaction kernel и тестов.

Executor adapter по-прежнему запускается перед `execution.commands`. Diff guard, reviewer gate, verifier, rollback, commit, memory promotion и reports используют тот же transaction flow.

## AgentSpec fields

```yaml
agent:
  adapter: deepseek
  model: deepseek-chat
  dry_run: true
```

- `adapter`: `command`, `deepseek` или `kimi`.
- `model`: optional model label, который пишется в traces и API requests.
- `dry_run`: пишет adapter artifacts без provider request.

`command_template` больше не является user-facing provider field. AgentHub сам владеет API requests, logs, retries и будущими tool calls.

Role-specific adapters можно задавать в `agents`:

```yaml
agents:
  executor:
    adapter: deepseek
    dry_run: true
  reviewer:
    adapter: kimi
    dry_run: true
```

## Текущий статус project executor

Non-project chat mode уже может вызывать DeepSeek/Kimi напрямую. Project transaction routes для `deepseek` и `kimi` сейчас явно записывают fallback в deterministic kernel, пока подключается API-native project executor и tool loop.

Каждая transaction пишет выбранные routes в `.agent/tx/<tx-id>/agent_trace.json`. Adapter prompt artifacts пишутся как `.agent/tx/<tx-id>/agent_prompt_<role>.md`.
