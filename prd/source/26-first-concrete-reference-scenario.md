# 26. First Concrete Reference Scenario

Несмотря на масштаб проекта, первый reference scenario должен быть достаточно конкретным.

## Scenario: Add Page to Existing Web App

User:

```text
Добавь страницу курсов в текущий Next.js проект, в стиле текущего dashboard.
```

System flow:

```text
1. Detect existing project
2. Load agent.lock
3. Load code memory schema
4. Build AgentSpec
5. Compile DAG
6. Create CodeWorkspace via git worktree
7. Build context pack
8. Select skills:
   - code.nextjs.add_page
   - design.reuse_existing_style
   - verifier.web_runtime_smoke
9. Execute agent
10. Run diff guard
11. Run npm build
12. Run runtime smoke /courses
13. Run sync check
14. Commit or rollback
15. Promote memory or write failed attempt
16. Generate report
```

Acceptance:

* no direct main mutation;
* out-of-scope edits blocked;
* runtime route tested;
* memory promoted only on success;
* failed attempt stored separately;
* report generated;
* cost visible.

---

