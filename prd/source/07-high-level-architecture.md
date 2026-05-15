# 7. High-Level Architecture

## 7.1 Macro Architecture

```text
AgentHub
│
├── Interface Layer
│   ├── CLI / SLI
│   ├── TUI
│   ├── Web Dashboard
│   ├── VS Code Extension
│   └── Future Visual Agent IDE
│
├── Intent Layer
│   ├── Natural Language Input
│   ├── Intent Normalizer
│   ├── Clarification Engine
│   └── Default Resolver
│
├── Language Layer
│   ├── AAL
│   ├── AgentSpec
│   ├── AgentIR
│   └── Agent Lock
│
├── Compiler Layer
│   ├── Spec Parser
│   ├── Policy Validator
│   ├── Skill Resolver
│   ├── Memory Schema Resolver
│   ├── Agent Topology Planner
│   └── Execution DAG Builder
│
├── VCM-OS Memory Layer
│   ├── Typed Memory Core
│   ├── Domain Schemas
│   ├── Retrieval Engine
│   ├── Memory Compaction
│   ├── Staging Memory
│   └── Failed Attempt Memory
│
├── Skill Registry
│   ├── Skill Manifests
│   ├── Prompt Fragments
│   ├── Actions
│   ├── Verifiers
│   ├── Policies
│   └── Skill Dependencies
│
├── Agent Orchestration Layer
│   ├── Agent Adapters
│   ├── Topologies
│   ├── Routing Policies
│   ├── Reviewer/Critic Loops
│   └── Repair Loops
│
├── Workspace Runtime
│   ├── CodeWorkspace
│   ├── DataWorkspace
│   ├── InfraWorkspace
│   ├── ContentWorkspace
│   ├── MediaWorkspace
│   └── ResearchWorkspace
│
├── Execution Kernel
│   ├── Transaction Manager
│   ├── DAG Executor
│   ├── Process Supervisor
│   ├── Effect Tracker
│   ├── Diff Guard
│   ├── Sync Check
│   ├── Rollback Engine
│   └── Post-Commit Effects
│
├── Verifier Layer
│   ├── Build/Test Verifier
│   ├── Runtime Smoke Verifier
│   ├── Browser Verifier
│   ├── Data Quality Verifier
│   ├── Terraform Plan Verifier
│   ├── Content Quality Verifier
│   ├── Security Verifier
│   └── Policy Verifier
│
├── LLM Gateway / Observability
│   ├── Request/Response Trace
│   ├── Redaction
│   ├── Token/Cost Profiler
│   ├── Context Pack Trace
│   ├── Skill Trace
│   └── Transaction Reports
│
└── Plugin / Marketplace Layer
    ├── Workspace Plugins
    ├── Skill Packages
    ├── Agent Adapters
    ├── Verifier Plugins
    └── Memory Schemas
```

## 7.2 Core Architectural Decision

```text
AgentHub Core не знает домен.
Workspace + Skill + Memory Schema задают домен.
```

Пример:

```text
create web app
  → workspace: CodeWorkspace
  → memory_schema: code_project
  → skills: nextjs, auth, ui, db
  → verifiers: build, runtime_smoke
```

```text
write YouTube script
  → workspace: ContentWorkspace
  → memory_schema: content_channel
  → skills: hook, script, tts_prompt, style
  → verifiers: length_check, tone_check, repetition_check
```

```text
deploy AWS infra
  → workspace: InfraWorkspace
  → memory_schema: infra_project
  → skills: terraform, aws, security
  → verifiers: terraform_plan, cost_estimate, policy_check
```

---

