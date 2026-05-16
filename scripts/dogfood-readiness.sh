#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HISTORY_DIR="${AGENTHUB_DOGFOOD_HISTORY_DIR:-$ROOT/target/dogfood/history}"
INDEX="$HISTORY_DIR/index.jsonl"
CHECK=false

if [[ "${1:-}" == "--check" ]]; then
  CHECK=true
fi

min_suite="${AGENTHUB_DOGFOOD_MIN_SUITE_RUNS:-3}"
min_provider="${AGENTHUB_DOGFOOD_MIN_PROVIDER_PASSED:-1}"
min_days="${AGENTHUB_DOGFOOD_MIN_DAYS:-2}"

if [[ ! -f "$INDEX" ]]; then
  printf 'dogfood readiness: no history index found at %s\n' "$INDEX" >&2
  if [[ "$CHECK" == true ]]; then
    exit 1
  fi
  exit 0
fi

extract() {
  local key="$1"
  sed -n "s/.*\"$key\":\"\\([^\"]*\\)\".*/\\1/p"
}

total_runs="$(wc -l < "$INDEX" | tr -d ' ')"
suite_runs="$(grep -c '"kind":"suite"' "$INDEX" || true)"
provider_runs="$(grep -c '"kind":"provider"' "$INDEX" || true)"
provider_passed="$(grep '"kind":"provider"' "$INDEX" | grep -c '"provider_status":"passed"' || true)"
latest="$(tail -n 1 "$INDEX")"
latest_run="$(printf '%s\n' "$latest" | extract run_id)"
latest_at="$(printf '%s\n' "$latest" | extract archived_at)"
distinct_days="$(sed -n 's/.*"archived_at":"\([0-9-]*\)T.*/\1/p' "$INDEX" | sort -u | wc -l | tr -d ' ')"
missing_reports=0

while IFS= read -r line; do
  report="$(printf '%s\n' "$line" | extract report)"
  if [[ -n "$report" && ! -f "$report" ]]; then
    missing_reports=$((missing_reports + 1))
  fi
done < "$INDEX"

printf 'AgentHub dogfood readiness\n'
printf 'history: %s\n' "$INDEX"
printf 'total runs: %s\n' "$total_runs"
printf 'suite runs: %s\n' "$suite_runs"
printf 'provider runs: %s\n' "$provider_runs"
printf 'provider passed: %s\n' "$provider_passed"
printf 'distinct days: %s\n' "$distinct_days"
printf 'missing reports: %s\n' "$missing_reports"
printf 'latest: %s %s\n' "$latest_run" "$latest_at"

failed=false
if (( suite_runs < min_suite )); then
  printf 'needs suite runs: %s/%s\n' "$suite_runs" "$min_suite"
  failed=true
fi
if (( provider_passed < min_provider )); then
  printf 'needs passed provider runs: %s/%s\n' "$provider_passed" "$min_provider"
  failed=true
fi
if (( distinct_days < min_days )); then
  printf 'needs distinct dogfood days: %s/%s\n' "$distinct_days" "$min_days"
  failed=true
fi
if (( missing_reports > 0 )); then
  printf 'needs all archived reports to exist\n'
  failed=true
fi

if [[ "$failed" == true ]]; then
  printf 'dogfood readiness: incomplete\n'
  if [[ "$CHECK" == true ]]; then
    exit 1
  fi
else
  printf 'dogfood readiness: ready\n'
fi
