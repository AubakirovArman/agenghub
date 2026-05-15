# Interactive Shell

AgentHub shell — это control shell для локальных транзакций. Он нужен, чтобы ежедневная работа ощущалась как один продуктовый интерфейс, но каждый выполненный запрос оставался аудируемым.

## Запуск

```bash
agenthub
# или
agenthub shell
```

Shell стартует в режиме `plan`. Plain text создаёт draft AgentSpec. Переключись в `run`, если plain text должен выполняться сразу:

```text
agenthub:plan> mode run
agenthub:run> add a generated health-check file
```

## Модель сессий

Shell теперь хранит лёгкие chat transcripts в `.agent/shell/chats/`. Используй `chats`, `chat latest`, `chat new` и `messages`, чтобы переходить по истории сообщений.

Выполненные сообщения всё равно становятся transaction sessions, а не free-form provider chat rooms. Когда сообщение выполняется, AgentHub создаёт transaction с:

- journal и WAL;
- effect ledger;
- command logs и bounded tails;
- verifier output;
- report;
- memory promotion или failed-attempt warning;
- видимостью в dashboard.

Используй `sessions`, `open latest`, `report`, `effects` и `explain`, чтобы переходить по прошлым transaction work.

## Основные команды

```text
init                  initialize .agent
doctor                проверить локальную готовность
providers status      посмотреть configured providers
provider codex        настроить Codex как default provider
ask <request>         записать draft spec
do <request>          создать draft и запустить
mode run              выполнять будущий plain text сразу
sessions              список прошлых transactions
chats                 список shell chat transcripts
chat latest           выбрать последний chat transcript
chat new              начать новый chat transcript
messages              показать выбранный chat transcript
open latest           выбрать последнюю transaction
watch latest          следить за live journal
report latest         напечатать report
effects latest        напечатать effect ledger
explain latest        объяснить результат и next action
dashboard             записать/открыть static dashboard
quit                  выйти
```

## Рекомендуемый первый flow

```text
agenthub> init
agenthub> doctor
agenthub> providers status
agenthub> provider codex
agenthub> ask add a small docs page
agenthub> run .agent/drafts/<draft>.yaml
agenthub> explain latest
agenthub> dashboard
```

## Граница

Shell не заменяет provider. Codex, Kimi, Gemini, command providers или OpenAI-compatible endpoints всё ещё выполняют model work. Shell даёт transaction control, safety, history и inspection вокруг этой работы.
