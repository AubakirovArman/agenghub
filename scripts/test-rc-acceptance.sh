#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d "${TMPDIR:-/tmp}/agenthub-rc-acceptance.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT INT TERM

EVIDENCE="$TMP/evidence.jsonl"
WORK="$TMP/work"

AGENTHUB_RC_ACCEPTANCE_WORK="$WORK" \
AGENTHUB_RC_ACCEPTANCE_EVIDENCE="$EVIDENCE" \
  "$ROOT/scripts/rc-acceptance.sh" > "$TMP/acceptance.out"

grep -q 'AgentHub RC acceptance rehearsal passed' "$TMP/acceptance.out"
grep -q '"id":"stats"' "$EVIDENCE"
grep -q '"id":"ops_no_bootstrap"' "$EVIDENCE"
grep -q '"id":"ops_receipts"' "$EVIDENCE"
grep -q '"id":"approval_ux"' "$EVIDENCE"
grep -q '"id":"resume"' "$EVIDENCE"
grep -q '"id":"rewind"' "$EVIDENCE"
grep -q '"flow":"ops"' "$EVIDENCE"
grep -q '"flow":"project_edit"' "$EVIDENCE"
test ! -e "$WORK/ops-empty/.agent"

REPORT="$TMP/dogfood-report.json"
HISTORY="$TMP/history"
cat > "$REPORT" <<'JSON'
{"status":"passed"}
JSON
AGENTHUB_DOGFOOD_ARCHIVE_SOURCE="$REPORT" \
AGENTHUB_DOGFOOD_HISTORY_DIR="$HISTORY" \
AGENTHUB_DOGFOOD_ARCHIVE_ID="acceptance-test" \
AGENTHUB_RC_ACCEPTANCE_EVIDENCE="$EVIDENCE" \
AGENTHUB_RC_ACCEPTANCE_WORK="$WORK" \
  "$ROOT/scripts/archive-dogfood.sh" > "$TMP/archive.out"
test -f "$HISTORY/runs/acceptance-test/rc-acceptance-evidence.jsonl"
test -f "$HISTORY/runs/acceptance-test/rc-acceptance-artifacts/ops-exec.jsonl"
grep -q '"acceptance_evidence":' "$HISTORY/index.jsonl"

printf 'agenthub RC acceptance rehearsal test passed\n'
