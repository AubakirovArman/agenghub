# 27. Strategic Connection to WAL and VCM-OS

## 27.1 VCM-OS

VCM-OS is core to AgentHub.

It provides:

* typed memory;
* context compression;
* project continuity;
* failed attempt learning;
* cross-agent memory;
* memory compaction.

## 27.2 WAL

WAL should not be part of early AgentHub runtime.

Potential future roles:

* optimize local models;
* compact model weights;
* build specialized local intent normalizers;
* accelerate skill routing;
* research agent-specific model representations.

Strategic relationship:

```text
WAL = language/IR for model weights
AAL = language/IR for agent actions
VCM-OS = memory OS for agent/project state
AgentHub = runtime that connects them
```

---

