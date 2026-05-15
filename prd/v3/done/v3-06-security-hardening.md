# PRD v3 Task 06 - Security Hardening

Status: Done

## Goal

Move sandbox/runner controls closer to local product safety with concrete resource and isolation checks.

## Acceptance

- Add OS-specific capability detection for cgroups, containers, Windows process control, and network policy support.
- Add structured sandbox hardening report.
- Add resource limit configuration model for CPU, memory, time, disk, and network.
- Degrade unsupported hardening features to warnings rather than panics.
- Add tests for detection/report/config behavior.
- Update README/docs in English, Russian, Chinese, and Kazakh.
- Module-size check stays under 200 lines per Rust/JS implementation file.

## Completed

- Added `hardening` module with:
  - cgroups v2 detection;
  - docker/podman container backend detection;
  - Windows Job Objects capability status;
  - network policy configuration detection;
  - process tree kill capability status.
- Added structured `SandboxHardeningReport` into transaction `sandbox.json`.
- Added `.agent/policies/resources.yaml` default policy during `agenthub init`.
- Added resource limit config model for timeout, CPU, memory, disk, network, and filesystem.
- Added environment overrides for runner metadata resource limits.
- Degraded unsupported hardening features to warnings.
- Added hardening tests.
- Added security hardening docs in English, Russian, Chinese, and Kazakh.
- Updated README and changelog.

## Evidence

- Implementation commit: `5237d05 Add sandbox hardening reports`
- Every transaction sandbox report now includes `hardening.platform`, `hardening.resource_limits`, `hardening.capabilities`, and `hardening.warnings`.
- Unsupported host features are serialized as warning capability records instead of causing failures.

## Validation

- `cargo fmt -- --check`
- `scripts/check-module-size.sh 200`
- `cargo test --locked hardening`
- `scripts/smoke-test.sh`
- `scripts/test-fixtures.sh`
- `git diff --check`
- `npm --prefix editors/vscode run check`
- `cargo clippy --locked -- -D warnings`
- `cargo test --locked`
