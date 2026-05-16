# AgentHub v0.4 API-native runtime

## Цель

AgentHub должен перестать быть оболочкой над чужими CLI. В v0.4 внешний LLM слой ограничен двумя API-провайдерами: DeepSeek и Kimi. Это дает собственные логи, контролируемую память, предсказуемый retry/fallback и возможность строить sub-agent orchestration внутри AgentHub.

## Режимы запуска

- Chat mode: обычный `agenthub` в папке без проекта не требует Git и `.agent`; история, индекс чатов и command logs пишутся в глобальный AgentHub home.
- DevOps mode: пользователь может обсуждать сервер и запускать `!command` без создания файлов проекта.
- Project mode: `.agent` и Git нужны только когда пользователь запускает transaction с изменением файлов, verifier, rollback, commit и memory promotion.

## Провайдеры

- `deepseek`: OpenAI-compatible DeepSeek API, ключ `DEEPSEEK_API_KEY`; legacy `ANTHROPIC_AUTH_TOKEN` можно переиспользовать для DeepSeek-compatible deployments.
- `kimi`: Kimi/Moonshot API, ключ `KIMI_API_KEY` или `MOONSHOT_API_KEY`.
- `command`: внутренний deterministic runner для transaction kernel и тестов; это не внешний AI provider.

На сервере можно положить ключи в `.deepseek` и `.kimi` в project directory или любой parent directory. AgentHub читает эти файлы как runtime secrets и не сохраняет содержимое в config.

DeepSeek API, Kimi API, legacy aliases и generic custom profiles больше не являются user-facing provider surface.

## Текущий статус

- Non-project shell уже не инициализирует Git и `.agent` автоматически.
- Chat/history/index/command logs получают global home fallback.
- Non-project plain messages идут напрямую в DeepSeek/Kimi API, если ключ настроен.
- Project transaction routes для `deepseek`/`kimi` пока явно fallback-ятся в deterministic kernel и записывают reason в trace; следующий этап - API-native project executor с tool loop.

## Следующие этапы

1. Streaming SSE для chat и dashboard.
2. Tool-calling loop: shell commands, file reads/writes, diff preview, verifier invocation.
3. Sub-agent manager/worker orchestration внутри AgentHub, без внешних CLI.
4. Memory policy: global user memory, project memory, failed-attempt warnings, promotion rules.
5. UI rewrite: отдельные Chat, DevOps и Project transaction screens.
6. Cost/token accounting per provider, request, role and transaction.
