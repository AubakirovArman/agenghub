# AgentHub PRD v3

AgentHub v3 turns the v2 runtime foundation into an installable local-first CLI product.

## Product Target

AgentHub is a local-first CLI/SLI runtime for safe AI-agent work. It connects agent tools such as Codex, Gemini, Kimi, and local/OpenAI-compatible providers, then wraps their work in transactions with memory, verifiers, rollback, policies, analytics, and reports.

## Milestone

AgentHub v0.2 — Installable Local Developer Preview.

Success criteria:

1. Install with one command or from a release binary.
2. Run `agenthub doctor`.
3. Configure and test at least one provider.
4. Initialize a project.
5. Run one safe transaction.
6. Open the dashboard.
7. See report, memory, effects, cost, and analytics.
8. Update or remove without manual repo knowledge.

## Phase Order

1. Phase 0: product/repository naming audit.
2. Phase A: release engineering.
3. Phase B: installers and binary packaging.
4. Phase C: product CLI UX.
5. Phase D: real provider gateway.
6. Phase E: product quality fixtures.
7. Phase F: security hardening.

## Rules

- Keep local-first behavior working at all times.
- Add docs and README updates in English, Russian, Chinese, and Kazakh for user-facing behavior.
- Keep Rust and JavaScript implementation files at or under 200 lines where practical.
- Commit each completed phase with evidence.
