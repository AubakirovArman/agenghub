# PRD v2 Task 10 — Adaptive Orchestration

Status: Todo

## Goal

Move topology selection from mostly static configuration toward explainable automatic routing based on task class, risk, historical outcomes, and deterministic override support.

## Acceptance

- Add a task classifier for simple edit, feature, bugfix, refactor, research, infra, content, high-risk, and unknown tasks.
- Add adaptive topology selection that can choose single executor, planner/executor/reviewer, swarm research, generator/critic, or manager/worker based on task class and risk.
- Record the selected topology, classifier inputs, and explanation in transaction artifacts and reports.
- Add a model/topology scoreboard with success, repair, rollback, human-block, cost, and latency fields suitable for later trend analytics.
- Make adaptive mode opt-in or explicitly disableable for deterministic runs.
- Keep existing explicit topology behavior compatible.
- Add tests for classifier decisions, topology selection, report explanation, scoreboard updates, and deterministic opt-out.
- README and docs are updated in English, Russian, Chinese, and Kazakh for user-facing behavior.
- Module-size check stays under 200 lines per Rust/JS implementation file.
