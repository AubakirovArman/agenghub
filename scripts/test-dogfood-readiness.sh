#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d "${TMPDIR:-/tmp}/agenthub-dogfood-readiness.XXXXXX")"
HISTORY="$TMP/history"
mkdir -p "$HISTORY/runs/suite-1" "$HISTORY/runs/suite-2" "$HISTORY/runs/suite-3" "$HISTORY/runs/provider-1"
trap 'rm -rf "$TMP"' EXIT INT TERM

touch "$HISTORY/runs/suite-1/dogfood-report.json"
touch "$HISTORY/runs/suite-2/dogfood-report.json"
touch "$HISTORY/runs/suite-3/dogfood-report.json"
touch "$HISTORY/runs/provider-1/provider-dogfood-report.json"

cat > "$HISTORY/index.jsonl" <<JSONL
{"run_id":"suite-1","archived_at":"2026-05-14T00:00:00Z","kind":"suite","report":"$HISTORY/runs/suite-1/dogfood-report.json","provider_report":"","provider":"","provider_status":"skipped","tx_id":""}
{"run_id":"suite-2","archived_at":"2026-05-15T00:00:00Z","kind":"suite","report":"$HISTORY/runs/suite-2/dogfood-report.json","provider_report":"","provider":"","provider_status":"skipped","tx_id":""}
{"run_id":"suite-3","archived_at":"2026-05-16T00:00:00Z","kind":"suite","report":"$HISTORY/runs/suite-3/dogfood-report.json","provider_report":"","provider":"","provider_status":"skipped","tx_id":""}
{"run_id":"provider-1","archived_at":"2026-05-16T01:00:00Z","kind":"provider","report":"$HISTORY/runs/provider-1/provider-dogfood-report.json","provider_report":"$HISTORY/runs/provider-1/provider-dogfood-report.json","provider":"codex","provider_status":"passed","tx_id":"tx-demo"}
JSONL

AGENTHUB_DOGFOOD_HISTORY_DIR="$HISTORY" "$ROOT/scripts/dogfood-readiness.sh" --check > "$TMP/pass.out"
grep -q 'dogfood readiness: ready' "$TMP/pass.out"

rm "$HISTORY/runs/suite-3/dogfood-report.json"
if AGENTHUB_DOGFOOD_HISTORY_DIR="$HISTORY" "$ROOT/scripts/dogfood-readiness.sh" --check > "$TMP/fail.out" 2>&1; then
  printf 'expected dogfood readiness check to fail when an archived report is missing\n' >&2
  exit 1
fi
grep -q 'missing reports: 1' "$TMP/fail.out"

printf 'agenthub dogfood readiness test passed\n'
