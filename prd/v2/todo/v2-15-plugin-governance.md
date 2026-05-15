# PRD v2 Task 15 — Plugin Marketplace Governance

Status: Todo

## Goal

Move plugin support from install/signature foundations toward governed marketplace behavior with test harnesses, scorecards, permissions, publisher identity, and review/deprecation metadata.

## Acceptance

- Add plugin permission metadata for commands, network, filesystem, model access, workspace profiles, and verifier/runtime capabilities.
- Add plugin scorecard output covering manifest validity, signature state, tests, permissions, trust, and compatibility.
- Add plugin test harness support for golden examples or manifest-declared checks.
- Add publisher identity and review/deprecation metadata to plugin lock or registry artifacts.
- Add semantic version compatibility checks for AgentHub/plugin API versions.
- Add vulnerability/deprecation warning artifacts that do not panic normal listing.
- Ensure untrusted plugins cannot request dangerous capabilities without explicit trust/override metadata.
- Add tests for permission parsing, scorecard generation, test harness behavior, compatibility, and lock output.
- README and docs are updated in English, Russian, Chinese, and Kazakh for user-facing behavior.
- Module-size check stays under 200 lines per Rust/JS implementation file.
