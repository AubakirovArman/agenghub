#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AGENTHUB_BIN="${AGENTHUB_BIN:-agenthub}"
KIMI_AUTH_REPORT="${AGENTHUB_KIMI_AUTH_REPORT:-$ROOT/target/dogfood/kimi-auth-report.json}"
KIMI_AUTH_CHECK_CMD="${AGENTHUB_KIMI_AUTH_CHECK_CMD:-$ROOT/scripts/kimi-auth-check.sh}"
PROVIDER_DOGFOOD_CMD="${AGENTHUB_PROVIDER_DOGFOOD_CMD:-$ROOT/scripts/provider-dogfood.sh}"
RC_EVIDENCE_COLLECT_CMD="${AGENTHUB_RC_EVIDENCE_COLLECT_CMD:-$ROOT/scripts/rc-evidence-collect.sh}"
RC_DOGFOOD_GATE_CMD="${AGENTHUB_RC_DOGFOOD_GATE_CMD:-$ROOT/scripts/rc-dogfood-gate.sh}"
SKIP_PROVIDER_DOGFOOD=false
NO_CHECK=false

usage() {
  cat <<'USAGE'
Usage:
  scripts/kimi-rc-unblock.sh [--skip-provider-dogfood] [--no-check]

Runs the Kimi 1.0 RC unblock path after a key has been rotated:
  1. agenthub providers test kimi
  2. scripts/kimi-auth-check.sh
  3. live Kimi provider dogfood
  4. scripts/rc-evidence-collect.sh
  5. scripts/rc-dogfood-gate.sh --check

If the provider test fails, the auth check still runs as diagnostics so the
redacted auth report covers both official Moonshot endpoints before the command
returns blocked.

If you have a replacement key file, inspect it offline and verify it live
without writing first:
  agenthub providers inspect-key kimi --from-file <new-key-file>
  agenthub providers preflight-key kimi --from-file <new-key-file>

Then run the one product-CLI unblock command:
  agenthub providers rc-unblock kimi --from-file <new-key-file>
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-provider-dogfood)
      SKIP_PROVIDER_DOGFOOD=true
      ;;
    --no-check)
      NO_CHECK=true
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      printf 'unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

run_required() {
  local label="$1"
  shift
  printf 'step\t%s\tbegin\n' "$label"
  if "$@"; then
    printf 'step\t%s\tpassed\n' "$label"
  else
    local code=$?
    printf 'step\t%s\tfailed\t%s\n' "$label" "$code"
    return "$code"
  fi
}

json_field() {
  local key="$1" file="$2"
  if [[ ! -f "$file" ]]; then
    return 0
  fi
  sed -n "s/.*\"$key\"[[:space:]]*:[[:space:]]*\"\\([^\"]*\\)\".*/\\1/p" "$file" | head -n1
}

run_provider_test() {
  local endpoint="${1:-}"
  if [[ -n "$endpoint" ]]; then
    printf 'endpoint_override\tKIMI_API_BASE_URL\t%s\n' "$endpoint"
    KIMI_API_BASE_URL="$endpoint" run_required provider_test "$AGENTHUB_BIN" providers test kimi
  else
    run_required provider_test "$AGENTHUB_BIN" providers test kimi
  fi
}

run_kimi_auth_check() {
  AGENTHUB_KIMI_AUTH_REPORT="$KIMI_AUTH_REPORT" run_required kimi_auth_check "$KIMI_AUTH_CHECK_CMD"
}

blocked_after_provider_test() {
  printf 'status\tblocked\n'
  printf 'reason\tprovider_test_failed\n'
  printf 'next\t1\tagenthub providers inspect-key kimi\n'
  printf 'next\t2\tagenthub providers inspect-key kimi --from-file <new-key-file>\n'
  printf 'next\t3\tagenthub providers preflight-key kimi --from-file <new-key-file>\n'
  printf 'next\t4\tagenthub providers rc-unblock kimi --from-file <new-key-file>\n'
  printf 'next\t5\tagenthub providers rotate-key kimi --from-file <new-key-file>\n'
  printf 'next\t6\tagenthub providers unblock kimi\n'
}

printf 'AgentHub Kimi RC unblock\n'

endpoint_override=""
if ! run_provider_test ""; then
  if run_kimi_auth_check; then
    endpoint_override="$(json_field passed_endpoint "$KIMI_AUTH_REPORT")"
    if [[ -n "$endpoint_override" ]] && run_provider_test "$endpoint_override"; then
      :
    else
      blocked_after_provider_test
      exit 1
    fi
  else
    blocked_after_provider_test
    exit 1
  fi
elif ! run_kimi_auth_check; then
  printf 'status\tblocked\n'
  printf 'reason\tkimi_auth_check_failed\n'
  printf 'next\t1\tagenthub providers inspect-key kimi\n'
  printf 'next\t2\tagenthub providers inspect-key kimi --from-file <new-key-file>\n'
  printf 'next\t3\tagenthub providers preflight-key kimi --from-file <new-key-file>\n'
  printf 'next\t4\tagenthub providers rc-unblock kimi --from-file <new-key-file>\n'
  printf 'next\t5\tagenthub providers rotate-key kimi --from-file <new-key-file>\n'
  printf 'next\t6\tagenthub providers unblock kimi\n'
  exit 1
fi
endpoint_override="${endpoint_override:-$(json_field passed_endpoint "$KIMI_AUTH_REPORT")}"
if [[ -n "$endpoint_override" ]]; then
  printf 'endpoint_override\tKIMI_API_BASE_URL\t%s\n' "$endpoint_override"
fi

if [[ "$SKIP_PROVIDER_DOGFOOD" == true ]]; then
  printf 'step\tprovider_dogfood\tskipped\n'
  printf 'warning\tprovider_dogfood_required_for_rc_gate\n'
else
  if ! KIMI_API_BASE_URL="$endpoint_override" \
    AGENTHUB_PROVIDER_DOGFOOD_PROVIDER=kimi \
    AGENTHUB_PROVIDER_DOGFOOD_LIVE=1 \
    run_required provider_dogfood "$PROVIDER_DOGFOOD_CMD"; then
    printf 'status\tblocked\n'
    printf 'reason\tprovider_dogfood_failed\n'
    printf 'next\t1\tAGENTHUB_PROVIDER_DOGFOOD_PROVIDER=kimi AGENTHUB_PROVIDER_DOGFOOD_LIVE=1 scripts/provider-dogfood.sh\n'
    exit 1
  fi
fi

run_required rc_evidence_collect "$RC_EVIDENCE_COLLECT_CMD"

if [[ "$NO_CHECK" == true ]]; then
  run_required rc_dogfood_gate_summary "$RC_DOGFOOD_GATE_CMD"
  printf 'status\tunchecked\n'
  printf 'next\t1\tscripts/rc-dogfood-gate.sh --check\n'
else
  run_required rc_dogfood_gate "$RC_DOGFOOD_GATE_CMD" --check
  printf 'status\tready\n'
fi
