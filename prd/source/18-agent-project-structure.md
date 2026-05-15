# 18. `.agent/` Project Structure

## 18.1 Proposed Structure

```text
.agent/
  agent.lock
  project.yaml

  tx/
    tx-123/
      plan.yaml
      agent_ir.txt
      dag.json
      journal.jsonl
      context_pack.json
      redacted_api.jsonl
      raw_api.jsonl
      verifier.log
      diff.patch
      memory_staging.jsonl
      report.md

  memory/
    committed.jsonl
    failed_attempts.jsonl
    compacted/
      project_state.json
      architecture.json
      current_routes.json

  maps/
    routes.map.json
    components.map.json
    api.map.json
    db.map.json
    exports.map.json

  skills/
    installed.json

  policies/
    core.yaml
    security.yaml
    diff_limits.yaml

  workspaces/
    tx-123/

  cache/
    embeddings/
    indexes/
```

## 18.2 Source of Truth

* transaction truth: `journal.jsonl`;
* memory truth: `committed.jsonl` + compacted views;
* failed truth: `failed_attempts.jsonl`;
* current project constraints: `agent.lock`.

SQLite can be used as index/cache, but append-only logs remain debuggable truth.

---

