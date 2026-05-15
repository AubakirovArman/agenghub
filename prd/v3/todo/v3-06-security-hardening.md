# PRD v3 Task 06 — Security Hardening

Status: Todo

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
