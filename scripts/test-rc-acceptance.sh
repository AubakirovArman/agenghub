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

printf 'agenthub RC acceptance rehearsal test passed\n'
