# 30. Current Decision Snapshot

Current architectural decisions:

```text
1. AgentHub is a runtime, not a single agent.
2. Core must be domain-agnostic.
3. First reference domain is software development.
4. AI actions are transactions.
5. Memory promotion requires successful verification.
6. Failed attempts are separate from project truth.
7. AgentSpec starts as YAML/JSON before custom AAL syntax.
8. Rust is preferred for core execution kernel.
9. Skills are external packages, not hardcoded prompts.
10. Workspaces are pluggable.
11. Verifiers are domain-specific profiles.
12. LLM Gateway with redaction is mandatory.
13. agent.lock protects against state drift.
14. Context packs follow minimum sufficient context principle.
15. WAL integration is future research, not early runtime dependency.
```

---

