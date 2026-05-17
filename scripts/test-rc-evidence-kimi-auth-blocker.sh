#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d "${TMPDIR:-/tmp}/agenthub-rc-kimi-auth.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT INT TERM

HOME_DIR="$TMP/home"
PROJECT="$TMP/project"
HISTORY="$TMP/history"
EVIDENCE="$TMP/evidence.jsonl"
KIMI_REPORT="$TMP/kimi-auth-report.json"
mkdir -p "$HOME_DIR" "$PROJECT" "$HISTORY"

cat > "$KIMI_REPORT" <<'JSON'
{
  "provider": "kimi",
  "status": "blocked",
  "auth_key_sha256_12": "abc123abc123",
  "next_action": "replace or rotate the Kimi/Moonshot API key"
}
JSON

AGENTHUB_HOME="$HOME_DIR" \
AGENTHUB_RC_SOURCE_ROOT="$PROJECT" \
AGENTHUB_DOGFOOD_HISTORY_DIR="$HISTORY" \
AGENTHUB_RC_EVIDENCE="$EVIDENCE" \
AGENTHUB_RC_KIMI_AUTH_REPORT="$KIMI_REPORT" \
AGENTHUB_RC_COLLECT_RUN_STATS=0 \
  "$ROOT/scripts/rc-evidence-collect.sh" > "$TMP/collect.out"

grep -q '"kind":"blocker"' "$EVIDENCE"
grep -q '"id":"kimi-auth"' "$EVIDENCE"
grep -q '"severity":"critical"' "$EVIDENCE"
grep -q '"status":"open"' "$EVIDENCE"
grep -q '"source":"kimi_auth_report"' "$EVIDENCE"
grep -q 'replace or rotate the Kimi/Moonshot API key' "$EVIDENCE"

if AGENTHUB_DOGFOOD_HISTORY_DIR="$HISTORY" \
  AGENTHUB_DOGFOOD_MIN_SUITE_RUNS=0 \
  AGENTHUB_DOGFOOD_MIN_PROVIDER_PASSED=0 \
  AGENTHUB_DOGFOOD_MIN_DAYS=0 \
  AGENTHUB_RC_EVIDENCE="$EVIDENCE" \
  AGENTHUB_RC_MIN_REAL_SESSIONS=0 \
  AGENTHUB_RC_MIN_OPS_FLOWS=0 \
  AGENTHUB_RC_MIN_PROJECT_EDIT_FLOWS=0 \
  AGENTHUB_RC_MIN_COST_RECEIPTS=0 \
  AGENTHUB_RC_REQUIRED_PROVIDERS= \
  AGENTHUB_RC_REQUIRED_CHECKS= \
    "$ROOT/scripts/rc-dogfood-gate.sh" --check > "$TMP/gate.out" 2>&1; then
  printf 'expected gate to fail on Kimi auth blocker\n' >&2
  exit 1
fi

grep -q 'open blocker/critical blockers: 1' "$TMP/gate.out"
grep -q 'needs blocker/critical issues closed before 1.0 RC' "$TMP/gate.out"

printf 'agenthub RC evidence Kimi auth blocker test passed\n'
