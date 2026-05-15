# Interactive Shell

AgentHub shell 是本地 transactions 的 control shell。它让日常使用像一个完整产品界面，同时保证每个执行的请求都可审计。

## 启动

```bash
agenthub
# 或
agenthub shell
```

Shell 默认进入 `plan` mode。Plain text 会创建 draft AgentSpec。如果希望 plain text 立即执行，切换到 `run` mode：

```text
agenthub:plan> mode run
agenthub:run> add a generated health-check file
```

## Session Model

Shell 现在会把轻量 chat transcripts 保存在 `.agent/shell/chats/`。使用 `chats`、`chat latest`、`chat new` 和 `messages` 浏览消息历史。

已执行的消息仍然是 transaction sessions，而不是 free-form provider chat rooms。消息被执行时，AgentHub 会创建 transaction，并包含：

- journal 和 WAL；
- effect ledger；
- command logs 和 bounded tails；
- verifier output；
- report；
- memory promotion 或 failed-attempt warning；
- dashboard visibility。

使用 `sessions`、`open latest`、`report`、`effects` 和 `explain` 浏览过去的 transaction work。

## 核心命令

```text
init                  initialize .agent
doctor                检查本地环境
providers status      查看 configured providers
provider codex        设置 Codex 为 default provider
ask <request>         写入 draft spec
do <request>          创建 draft 并运行
mode run              让后续 plain text 直接执行
sessions              列出 previous transactions
chats                 列出 shell chat transcripts
chat latest           选择 latest chat transcript
chat new              开始新的 chat transcript
messages              打印当前 chat transcript
open latest           选择 latest transaction
watch latest          跟随 live journal
approve <note>        为当前 transaction 记录 human resolution
resume latest         approval 后继续 blocked transaction
report latest         输出 report
effects latest        输出 effect ledger
explain latest        解释结果和 next action
dashboard             写入/打开 static dashboard
quit                  退出
```

## 推荐的首次 Flow

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

## 边界

Shell 不替代 provider。Codex、Kimi、Gemini、command providers 或 OpenAI-compatible endpoints 仍然执行 model work。Shell 为这些工作提供 transaction control、safety、history 和 inspection。
