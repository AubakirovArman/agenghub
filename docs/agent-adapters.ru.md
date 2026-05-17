# AgentHub Provider Routes

Языки: [English](agent-adapters.en.md), [Русский](agent-adapters.ru.md), [中文](agent-adapters.zh.md), [Қазақша](agent-adapters.kk.md)

## Назначение

В AgentHub v0.4 provider work уходит от внешних AI CLI. User-facing AI providers теперь API-native `deepseek` и `kimi`; `command` остается встроенным deterministic runner для transaction kernel и тестов.

Поле AgentSpec всё ещё называется `agent.adapter` для совместимости, но в product language это route selector, а не external CLI integration. Selected route запускается перед `execution.commands`. Diff guard, reviewer gate, verifier, rollback, commit, memory promotion и reports используют тот же transaction flow.

## AgentSpec fields

```yaml
agent:
  adapter: deepseek
  model: deepseek-chat
  dry_run: true
```

- `adapter`: compatibility route selector. `deepseek` и `kimi` — AgentHub-owned API routes; `command` — internal deterministic runner.
- `model`: optional model label, который пишется в traces и API requests.
- `dry_run`: пишет route artifacts без provider request.

`command_template` больше не является user-facing provider field. AgentHub сам владеет API requests, logs, retries и native command-plan tool calls.

Role-specific routes можно задавать в `agents`:

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

Non-project chat mode уже вызывает DeepSeek/Kimi напрямую со streaming output. Project transaction routes для `deepseek` и `kimi` используют AgentHub-owned API requests: provider может вызвать bounded builtin read/search/read-only-shell tools для контекста, AgentHub reinjects redacted tool results в тот же turn, затем provider вызывает native `agenthub_command_plan` tool call или возвращает JSON command plan fallback. AgentHub валидирует и permission-checks эти команды внутри isolated worktree, затем продолжается обычный diff guard, verifier, rollback, commit и memory promotion flow.

Каждая transaction пишет выбранные routes в `.agent/tx/<tx-id>/agent_trace.json`. Provider prompt artifacts пишутся как `.agent/tx/<tx-id>/agent_prompt_<role>.md`, native command-plan tool-loop receipts пишутся как `.agent/tx/<tx-id>/tool_loop_<role>.json`, builtin tool-result reinjection receipts с path/output/network/limit policy summaries пишутся как `.agent/tx/<tx-id>/tool_results_<role>.json`, а API executor plans/results пишутся как `.agent/tx/<tx-id>/api_execution_<role>.json`.
