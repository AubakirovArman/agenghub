# Interactive Shell

The AgentHub shell is a control shell for local transactions. It is designed to make daily use feel like one product surface while still keeping every executed request auditable.

## Start

```bash
agenthub
# or
agenthub shell
```

The shell starts in `plan` mode. Plain text creates a draft AgentSpec. Switch to `run` mode when plain text should execute immediately:

```text
agenthub:plan> mode run
agenthub:run> add a generated health-check file
```

## Session Model

The shell now keeps lightweight chat transcripts under `.agent/shell/chats/`. Use `chats`, `chat latest`, `chat new`, and `messages` to move through the transcript history.

Executed messages are still transaction sessions, not free-form provider chat rooms. When a message is executed, AgentHub creates a transaction with:

- journal and WAL;
- effect ledger;
- command logs and bounded tails;
- verifier output;
- report;
- memory promotion or failed-attempt warning;
- dashboard visibility.

Use `sessions`, `open latest`, `report`, `effects`, and `explain` to move through past transaction work.

## Core Commands

```text
init                  initialize .agent
doctor                check local readiness
providers status      inspect configured providers
provider codex        set up Codex as the default provider
ask <request>         write a draft spec
do <request>          create a draft and run it
mode run              execute future plain text directly
sessions              list previous transactions
chats                 list shell chat transcripts
chat latest           select the latest chat transcript
chat new              start a new chat transcript
messages              print the selected chat transcript
open latest           select the latest transaction
watch latest          follow the live journal
approve <note>        record a human resolution on the selected transaction
resume latest         resume a blocked transaction after approval
report latest         print the report
effects latest        print the effect ledger
explain latest        explain result and next action
dashboard             write/open the static dashboard
quit                  exit
```

## Recommended First Flow

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

## Boundary

The shell does not replace the provider. Codex, Kimi, Gemini, command providers, or OpenAI-compatible endpoints still perform model work. The shell provides transaction control, safety, history, and inspection around that work.
