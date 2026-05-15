# Interactive Shell

AgentHub shell — local transactions үшін control shell. Ол күнделікті қолдануды бір product surface сияқты етеді, бірақ әр executed request audit-ready болып қалады.

## Іске қосу

```bash
agenthub
# немесе
agenthub shell
```

Shell `plan` mode режимінде басталады. Plain text draft AgentSpec жасайды. Plain text бірден орындалсын десең, `run` mode қос:

```text
agenthub:plan> mode run
agenthub:run> add a generated health-check file
```

## Session Model

Shell енді lightweight chat transcripts файлдарын `.agent/shell/chats/` ішінде сақтайды. Message history көру үшін `chats`, `chat latest`, `chat new` және `messages` қолдан.

Executed messages бәрібір transaction sessions болады, free-form provider chat rooms емес. Message орындалғанда AgentHub transaction жасайды және мынаны сақтайды:

- journal және WAL;
- effect ledger;
- command logs және bounded tails;
- verifier output;
- report;
- memory promotion немесе failed-attempt warning;
- dashboard visibility.

Бұрынғы transaction work көру үшін `sessions`, `open latest`, `report`, `effects` және `explain` қолдан.

## Негізгі Commands

```text
init                  initialize .agent
doctor                local readiness тексеру
providers status      configured providers көру
provider codex        Codex-ті default provider ету
ask <request>         draft spec жазу
do <request>          draft жасап, іске қосу
mode run              келесі plain text бірден орындалады
sessions              previous transactions тізімі
chats                 shell chat transcripts тізімі
chat latest           latest chat transcript таңдау
chat new              жаңа chat transcript бастау
messages              таңдалған chat transcript шығару
open latest           latest transaction таңдау
watch latest          live journal бақылау
approve <note>        таңдалған transaction үшін human resolution жазу
resume latest         approval кейін blocked transaction жалғастыру
report latest         report шығару
effects latest        effect ledger шығару
explain latest        result және next action түсіндіру
dashboard             static dashboard жазу/ашу
quit                  шығу
```

## Ұсынылатын алғашқы Flow

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

## Шекара

Shell provider-ді алмастырмайды. Codex, Kimi, Gemini, command providers немесе OpenAI-compatible endpoints model work орындай береді. Shell сол work айналасында transaction control, safety, history және inspection береді.
