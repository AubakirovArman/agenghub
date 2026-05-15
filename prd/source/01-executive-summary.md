# 1. Executive Summary

## 1.1 Что такое AgentHub

**AgentHub** — это транзакционная операционная среда для AI-агентов, которая превращает размытые человеческие запросы в структурированные, проверяемые, воспроизводимые и откатываемые агентные процессы.

AgentHub не заменяет Codex, Kimi, Gemini, GitHub Copilot, Claude Code или локальные модели. Он управляет ими как исполнительными движками.

Ключевая идея:

```text
AI action = transaction
```

Любое действие агента должно проходить через:

```text
intent → spec → plan → isolated execution → verification → commit/rollback → memory update
```

Если действие не прошло проверку, оно не должно попадать в основную рабочую среду и не должно загрязнять основную память проекта.

## 1.2 Что именно строится

AgentHub включает:

* **SLI / CLI / future UI** — интерфейс постановки задач;
* **AAL — Agent Action Language** — язык действий AI-агентов;
* **AgentSpec** — человекочитаемая спецификация задачи;
* **AgentIR** — промежуточное представление / байткод действий;
* **Compiler** — компилятор intent/spec в execution DAG;
* **VCM-OS** — структурированная память проекта и процесса;
* **Skill Registry** — модульные пакеты опыта и операций;
* **Agent Topology Planner** — планировщик графов агентов;
* **Workspace Runtime** — абстракция сред исполнения;
* **Transaction Manager** — ACID-подобное ядро исполнения;
* **Verifier Layer** — build/test/runtime/policy/data/content проверки;
* **Repair / Critic Loop** — контролируемые попытки исправления;
* **LLM Gateway** — трассировка, redaction, cost tracking;
* **Policy Engine** — ограничения, security, diff limits, allow/deny;
* **Observability Layer** — journal, reports, traces, metrics;
* **Agent Adapters** — Codex, Kimi, Gemini, Copilot, Claude, local models;
* **Domain Profiles** — Code, Infra, Data, Content, Media, Research.

## 1.3 Главная формула системы

```text
Human Request
  ↓
Intent Normalizer
  ↓
AAL / AgentSpec
  ↓
Compiler
  ↓
AgentIR / Execution DAG
  ↓
VCM-OS Memory Pack
  ↓
Skill Registry
  ↓
Agent Topology Planner
  ↓
Workspace Runtime
  ↓
Transaction Manager
  ↓
Agent Execution
  ↓
Verifier / Repair / Critic
  ↓
Commit or Rollback
  ↓
Memory Promotion / Failed Attempt Log
```

---

