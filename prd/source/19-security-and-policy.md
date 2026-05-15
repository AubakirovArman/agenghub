# 19. Security and Policy

## 19.1 Security Principles

* least privilege;
* explicit scopes;
* command allowlist;
* network policy;
* secret redaction;
* sandboxed execution;
* audit logs;
* human approval for dangerous effects;
* no blind external apply.

## 19.2 Command Policy

Commands can be classified:

```text
safe:
  - npm run build
  - npm test
  - pytest

needs_approval:
  - npm install
  - pip install
  - docker compose up

restricted:
  - rm -rf
  - sudo
  - terraform apply
  - cloud resource deletion
```

## 19.3 Sandbox Levels

### Level 0 — Local Controlled

* worktree;
* process groups;
* timeouts;
* kill tree;
* command allowlist.

### Level 1 — Local Sandbox

* containers;
* resource limits;
* network restrictions.

### Level 2 — Strong Isolation

* cgroups;
* namespaces;
* Firecracker/microVM;
* remote runner.

### Level 3 — Enterprise Runner

* central policy;
* audit;
* secrets manager;
* isolated execution pools.

---

