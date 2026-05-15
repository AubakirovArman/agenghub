# 16. Agent Orchestration

## 16.1 Agent Adapters

Adapters:

* Codex;
* Kimi;
* Gemini;
* Copilot;
* Claude;
* local models;
* custom HTTP models;
* future ACP/MCP adapters.

## 16.2 Topologies

### 16.2.1 Single Executor

Один агент исполняет задачу.

### 16.2.2 Planner → Executor

Один агент планирует, другой делает.

### 16.2.3 Generator → Critic

Один создаёт, другой критикует.

### 16.2.4 Executor → Reviewer → Repair

Классический code workflow.

### 16.2.5 Swarm Research

Параллельные агенты собирают информацию, aggregator объединяет.

### 16.2.6 Manager / Worker

Менеджер разбивает задачу, worker agents выполняют части.

### 16.2.7 Tournament

Несколько агентов предлагают варианты, critic выбирает лучший.

## 16.3 Routing Policy

Факторы выбора агента:

* task type;
* domain;
* cost;
* speed;
* model capability;
* local/remote policy;
* privacy level;
* user preference;
* previous success rate;
* failed attempt history.

---

