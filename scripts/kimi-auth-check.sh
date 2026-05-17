#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AGENTHUB_BIN="${AGENTHUB_BIN:-agenthub}"
REPORT_PATH="${AGENTHUB_KIMI_AUTH_REPORT:-$ROOT/target/dogfood/kimi-auth-report.json}"
ARTIFACT_DIR="${AGENTHUB_KIMI_AUTH_ARTIFACT_DIR:-$ROOT/target/dogfood/kimi-auth}"

json_escape() {
  printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g'
}

field() {
  local key="$1" file="$2"
  sed -n "s/^$key[[:space:]]*//p" "$file" | head -n1
}

classify_test() {
  local file="$1"
  if grep -q '^ok[[:space:]]kimi' "$file"; then
    printf 'passed'
  elif grep -q '^failed[[:space:]]kimi[[:space:]]auth' "$file"; then
    printf 'auth_failed'
  elif grep -q '^failed[[:space:]]kimi[[:space:]]rate_limited' "$file"; then
    printf 'rate_limited'
  elif grep -q '^failed[[:space:]]kimi[[:space:]]timeout' "$file"; then
    printf 'timeout'
  elif grep -q '^failed[[:space:]]kimi' "$file"; then
    printf 'failed'
  else
    printf 'unknown'
  fi
}

run_endpoint() {
  local label="$1" endpoint="$2" output="$3"
  set +e
  KIMI_API_BASE_URL="$endpoint" "$AGENTHUB_BIN" providers test kimi > "$output" 2> "$output.stderr"
  local exit_code=$?
  set -e
  if [[ "$exit_code" -ne 0 && ! -s "$output" ]]; then
    {
      printf 'failed\tkimi\ttransport\n'
      printf 'endpoint\t%s\n' "$endpoint"
      printf 'reason\tproviders test exited %s for %s\n' "$exit_code" "$label"
      cat "$output.stderr"
    } > "$output"
  fi
}

mkdir -p "$ARTIFACT_DIR" "$(dirname "$REPORT_PATH")"
diagnose="$ARTIFACT_DIR/diagnose.txt"
global_out="$ARTIFACT_DIR/global.txt"
china_out="$ARTIFACT_DIR/china.txt"

"$AGENTHUB_BIN" providers diagnose kimi > "$diagnose"
run_endpoint "global" "https://api.moonshot.ai/v1" "$global_out"
run_endpoint "china" "https://api.moonshot.cn/v1" "$china_out"

global_status="$(classify_test "$global_out")"
china_status="$(classify_test "$china_out")"
overall="blocked"
next_action="replace or rotate the Kimi/Moonshot API key, then run agenthub providers test kimi"
passed_endpoint=""
passed_endpoint_label=""
if [[ "$global_status" == "passed" || "$china_status" == "passed" ]]; then
  overall="passed"
  if [[ "$global_status" == "passed" ]]; then
    passed_endpoint_label="global"
    passed_endpoint="https://api.moonshot.ai/v1"
  else
    passed_endpoint_label="china"
    passed_endpoint="https://api.moonshot.cn/v1"
  fi
  next_action="run KIMI_API_BASE_URL=$passed_endpoint AGENTHUB_PROVIDER_DOGFOOD_PROVIDER=kimi AGENTHUB_PROVIDER_DOGFOOD_LIVE=1 scripts/provider-dogfood.sh"
elif [[ "$global_status" == "rate_limited" || "$china_status" == "rate_limited" ]]; then
  overall="rate_limited"
  next_action="wait for Kimi/Moonshot quota reset or raise limits, then rerun scripts/kimi-auth-check.sh"
elif [[ "$global_status" == "timeout" || "$china_status" == "timeout" ]]; then
  overall="network_timeout"
  next_action="check network reachability to Moonshot endpoints, then rerun scripts/kimi-auth-check.sh"
fi

auth_source="$(field auth_key_source "$diagnose")"
auth_chars="$(field auth_key_chars "$diagnose")"
auth_sha="$(field auth_key_sha256_12 "$diagnose")"
auth_trimmed="$(field auth_key_trimmed_for_request "$diagnose")"
model="$(field model "$diagnose")"

cat > "$REPORT_PATH" <<JSON
{
  "provider": "kimi",
  "status": "$(json_escape "$overall")",
  "model": "$(json_escape "$model")",
  "auth_key_source": "$(json_escape "$auth_source")",
  "auth_key_chars": "$(json_escape "$auth_chars")",
  "auth_key_sha256_12": "$(json_escape "$auth_sha")",
  "auth_key_trimmed_for_request": "$(json_escape "$auth_trimmed")",
  "passed_endpoint_label": "$(json_escape "$passed_endpoint_label")",
  "passed_endpoint": "$(json_escape "$passed_endpoint")",
  "endpoints": [
    {
      "label": "global",
      "base_url": "https://api.moonshot.ai/v1",
      "status": "$(json_escape "$global_status")",
      "artifact": "$(json_escape "$global_out")"
    },
    {
      "label": "china",
      "base_url": "https://api.moonshot.cn/v1",
      "status": "$(json_escape "$china_status")",
      "artifact": "$(json_escape "$china_out")"
    }
  ],
  "diagnose_artifact": "$(json_escape "$diagnose")",
  "next_action": "$(json_escape "$next_action")"
}
JSON

printf 'AgentHub Kimi auth check\n'
printf 'status: %s\n' "$overall"
printf 'global: %s\n' "$global_status"
printf 'china: %s\n' "$china_status"
if [[ -n "$passed_endpoint" ]]; then
  printf 'passed_endpoint: %s\n' "$passed_endpoint"
fi
printf 'auth_key_source: %s\n' "${auth_source:-unknown}"
printf 'auth_key_sha256_12: %s\n' "${auth_sha:-unknown}"
printf 'report: %s\n' "$REPORT_PATH"
printf 'artifacts: %s\n' "$ARTIFACT_DIR"
printf 'next: %s\n' "$next_action"

if [[ "$overall" == "blocked" ]]; then
  exit 1
fi
