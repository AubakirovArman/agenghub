# PRD v2 Task 05 — Workspace Runtime Trait

Status: Todo

## Goal

Move workspace execution behind a real `WorkspaceRuntime` abstraction so transaction code can work with pluggable domain runtimes instead of hard-coding the current Git worktree profile.

## Acceptance

- Define a `WorkspaceRuntime` trait for prepare, snapshot, run, diff, verify, commit, rollback, and cleanup responsibilities where supported by the current kernel.
- Extract the existing Git worktree behavior into a `CodeGitWorkspace` implementation.
- Keep transaction manager behavior compatible with existing `code.git` plans.
- Add structured runtime metadata to transaction artifacts or reports.
- Leave clear extension points for content, data, infra, media, and research runtimes.
- Tests prove existing code-git transactions still commit and roll back through the runtime path.
- README and docs are updated in English, Russian, Chinese, and Kazakh for user-facing behavior.
- Module-size check stays under 200 lines per Rust/JS implementation file.
