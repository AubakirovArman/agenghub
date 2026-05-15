# 21. Development Roadmap

Important: this is not “MVP in one week”. This is staged construction of a large system.

## Phase 1 — Execution Kernel Foundation

Goal: build transactional core.

Deliverables:

* CLI skeleton;
* transaction lifecycle;
* journal.jsonl;
* worktree-based CodeWorkspace;
* process supervisor;
* timeout handling;
* build verifier;
* rollback;
* transaction report;
* basic sync check;
* basic diff guard.

Acceptance:

* task can run in isolated worktree;
* failed build rolls back;
* successful build commits;
* main memory not updated on failure;
* report generated.

## Phase 2 — Observability and LLM Gateway

Deliverables:

* redacted traces;
* raw traces optional;
* context pack logs;
* token/cost estimate;
* model call metadata;
* skill trace placeholder;
* error fingerprints.

Acceptance:

* every transaction can be debugged;
* prompts and context packs can be inspected;
* secrets redacted by default.

## Phase 3 — VCM-OS Core

Deliverables:

* typed memory objects;
* committed memory;
* staging memory;
* failed attempt log;
* memory promotion;
* simple retrieval;
* compact project facts.

Acceptance:

* successful transaction promotes memory;
* failed transaction writes failed_attempt only;
* context pack uses memory facts.

## Phase 4 — AgentSpec YAML and Compiler

Deliverables:

* AgentSpec YAML schema;
* parser;
* policy validator;
* compiler to Execution DAG;
* AgentIR text form;
* basic rules.

Acceptance:

* user can run `agenthub run task.yaml`;
* DAG generated from spec;
* invalid scopes rejected before execution.

## Phase 5 — Skill Registry v1

Deliverables:

* skill manifest format;
* skill loader;
* code.add_page skill;
* design.reuse_style skill;
* verifier.web_runtime_smoke skill;
* skill dependency checks.

Acceptance:

* task selects skills;
* context pack includes skill-specific instructions only;
* irrelevant skills are not loaded.

## Phase 6 — Agent Adapters v1

Deliverables:

* CLI adapter abstraction;
* Codex adapter;
* Kimi adapter;
* Gemini adapter;
* process transcript capture;
* simple routing policy.

Acceptance:

* same AgentSpec can run with different executor adapters;
* traces show which agent was used.

## Phase 7 — Runtime Smoke and Repair Loop

Deliverables:

* web_runtime_smoke verifier;
* process tree kill;
* repair loop;
* max repair attempts;
* BLOCKED_ON_HUMAN;
* missing env detection.

Acceptance:

* build success but runtime fail is caught;
* repair attempts are bounded;
* unresolved missing env pauses transaction.

## Phase 8 — Context Maps

Deliverables:

* routes.map;
* components.map;
* exports.map;
* invalidation by hash;
* map-based context retrieval.

Acceptance:

* context pack can include interfaces/locations instead of full files;
* stale maps are detected.

## Phase 9 — Natural Language to AgentSpec

Deliverables:

* intent normalizer;
* clarification engine;
* defaults resolver;
* generated AgentSpec preview;
* user approval mode optional.

Acceptance:

* user can type natural request;
* system produces structured AgentSpec;
* blocking unknowns are asked as questions.

## Phase 10 — Advanced Agent Topologies

Deliverables:

* planner/executor;
* executor/reviewer/repair;
* generator/critic;
* swarm research;
* cost-aware routing.

Acceptance:

* DAG can contain multiple model roles;
* reviewer can block bad output;
* repair agent can be different from executor.

## Phase 11 — Additional Workspaces

Deliverables:

* ContentWorkspace;
* DataWorkspace;
* InfraWorkspace basic;
* domain memory schemas;
* domain verifiers.

Acceptance:

* same core transaction manager can execute non-code tasks.

## Phase 12 — IDE and Visual Layer

Deliverables:

* VS Code extension;
* transaction panel;
* memory panel;
* AgentSpec editor;
* visual DAG viewer;
* approval UI.

Acceptance:

* developer can inspect and manage AgentHub from IDE.

## Phase 13 — Marketplace / Plugin Ecosystem

Deliverables:

* skill package format;
* workspace plugin format;
* verifier plugin format;
* versioning;
* trust model;
* signing optional.

Acceptance:

* external author can publish a skill;
* project can install and lock skill versions.

## Phase 14 — Enterprise Layer

Deliverables:

* policy server;
* team audit logs;
* central secrets integration;
* role-based permissions;
* remote runners;
* private model routing;
* compliance reports.

Acceptance:

* enterprise team can enforce policies across projects.

---

