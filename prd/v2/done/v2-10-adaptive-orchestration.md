# PRD v2 Task 10 — Adaptive Orchestration

Status: Done

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

## Completed

- Added an opt-in adaptive routing flag at `topology.routing.adaptive`.
- Added task classification for simple edit, feature, bugfix, refactor, research, infra, content, high-risk, and unknown tasks.
- Added risk-aware topology selection for `single_executor`, `planner_executor`, `executor_reviewer_repair`, `swarm_research`, `generator_critic`, and `manager_worker`.
- Kept deterministic behavior as the default when adaptive mode is absent or false.
- Added `adaptive.json` with classifier inputs, class, risk, signals, model label, selected topology, original topology, and explanation.
- Added `effective_plan.yaml` to show the AgentSpec after adaptive topology selection.
- Added an `Adaptive Orchestration` report section.
- Added `.agent/metrics/orchestration_scoreboard.json` with runs, success, repair, rollback, human-block, cost, and latency aggregates by task class/topology/model.
- Treated `.agent/metrics/` as runtime artifact output so blocked transaction resume is not dirtied by scoreboard writes.
- Split report markdown rendering into `src/report/markdown.rs` to preserve the 200-line module limit.
- Added classifier/selection unit tests and an end-to-end transaction test for adaptive artifacts, report output, and scoreboard updates.
- Updated README and adaptive orchestration docs in English, Russian, Chinese, and Kazakh.

## Evidence

- Implementation commit: `e7dc957`.
- Checks: `cargo fmt -- --check`; `scripts/check-module-size.sh 200`; `git diff --check`; `cargo test adaptive::tests`; `cargo test adaptive_orchestration_records_decision_report_and_scoreboard`; `cargo test resolve_and_resume_blocked_transaction`; `cargo clippy -- -D warnings`; `cargo test`; `npm run check` in `editors/vscode`.
