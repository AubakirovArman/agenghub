# V4.09 Dogfood Readiness Gate

## Status

Done.

## Completed

- Added `scripts/dogfood-readiness.sh` to summarize dogfood history and optionally enforce release thresholds.
- Added `scripts/test-dogfood-readiness.sh` with synthetic history fixtures.
- Wired the readiness test into `scripts/release-readiness.sh`.
- Dogfooding docs now describe the readiness summary and `--check` mode in English, Russian, Chinese, and Kazakh.

## 1.0 Relevance

This gives AgentHub a concrete local gate for the remaining multi-day dogfood work. Maintainers can collect real runs in `target/dogfood/history/`, then use `scripts/dogfood-readiness.sh --check` to prove the configured suite, provider, and distinct-day thresholds before tagging 1.0.
