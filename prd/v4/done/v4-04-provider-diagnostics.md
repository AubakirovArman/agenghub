# V4.04 Provider Diagnostics

## Status

Done.

## Completed

- Added provider-specific credential marker metadata for Codex, Gemini, Kimi, and OpenAI-compatible HTTP endpoints.
- `providers diagnose` now reports auth state, auth hint, status hint, and known credential markers without printing secret values.
- `providers test` for CLI providers now includes the same auth marker summary alongside version/template readiness.
- Missing CLI credential markers are reported as `cli_managed_unknown` instead of a failure because each provider CLI may store login state differently.
- Product CLI docs were updated in English, Russian, Chinese, and Kazakh.

## 1.0 Relevance

This makes provider setup more debuggable before users run real model-backed transactions. It improves local product readiness without blocking users on provider-specific auth mechanisms AgentHub cannot reliably prove.
