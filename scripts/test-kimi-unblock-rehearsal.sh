#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_ROOT="${TMPDIR:-$ROOT/target/tmp}"
mkdir -p "$TMP_ROOT"
TMP="$(mktemp -d "$TMP_ROOT/agenthub-kimi-rehearsal.XXXXXX")"
PROJECT="$TMP/project"
CURRENT="$PROJECT/.kimi"
CANDIDATE="$TMP/candidate-kimi-key.txt"
OUT="$TMP/rehearsal.out"
JSON_OUT="$TMP/rehearsal.json"
ERR="$TMP/rehearsal.err"

cleanup() {
  rm -rf "$TMP"
}
trap cleanup EXIT

run_agenthub() {
  if [[ -n "${AGENTHUB_BIN:-}" ]]; then
    "$AGENTHUB_BIN" --project "$PROJECT" "$@"
  else
    cargo run --quiet --manifest-path "$ROOT/Cargo.toml" -- --project "$PROJECT" "$@"
  fi
}

require_output() {
  local pattern="$1"
  if ! grep -Fq "$pattern" "$OUT"; then
    printf 'expected Kimi rehearsal output to contain: %s\n' "$pattern" >&2
    printf '%s\n' '--- stdout ---' >&2
    sed -n '1,240p' "$OUT" >&2
    printf '%s\n' '--- stderr ---' >&2
    sed -n '1,120p' "$ERR" >&2
    exit 1
  fi
}

mkdir -p "$PROJECT"
cat > "$CURRENT" <<'JSON'
{"access_token":"cli-access-secret","refresh_token":"cli-refresh-secret","scope":"kimi-code","token_type":"Bearer"}
JSON
printf '  moonshot-rehearsal-candidate-key  \n' > "$CANDIDATE"

KIMI_API_KEY= \
MOONSHOT_API_KEY= \
KIMI_API_KEY_FILE= \
MOONSHOT_API_KEY_FILE= \
  run_agenthub providers rehearse-unblock kimi --from-file "$CANDIDATE" > "$OUT" 2> "$ERR"

require_output $'status\tready_to_preflight'
require_output $'writes_key\tfalse'
require_output $'network\tfalse'
require_output $'current_classification\tkimi_code_cli_oauth'
require_output $'candidate_classification\tplain_api_key_candidate'
require_output $'candidate_status\tcandidate'
require_output "agenthub providers inspect-key kimi --from-file $CANDIDATE"
require_output "agenthub providers preflight-key kimi --from-file $CANDIDATE"
require_output "agenthub providers rc-unblock kimi --from-file $CANDIDATE"
require_output 'live preflight must pass before rc-unblock installs a replacement key'

if grep -q 'moonshot-rehearsal-candidate-key\|cli-access-secret\|cli-refresh-secret' "$OUT" "$ERR"; then
  printf 'Kimi unblock rehearsal leaked credential material\n' >&2
  exit 1
fi
if ! grep -q 'cli-access-secret' "$CURRENT"; then
  printf 'Kimi unblock rehearsal unexpectedly changed the active .kimi file\n' >&2
  exit 1
fi

KIMI_API_KEY= \
MOONSHOT_API_KEY= \
KIMI_API_KEY_FILE= \
MOONSHOT_API_KEY_FILE= \
  run_agenthub providers rehearse-unblock kimi --json --from-file "$CANDIDATE" > "$JSON_OUT"

grep -q '"objective": "kimi_replacement_key_unblock_rehearsal"' "$JSON_OUT"
grep -q '"status": "ready_to_preflight"' "$JSON_OUT"
grep -q '"writes_key": false' "$JSON_OUT"
grep -q '"network": false' "$JSON_OUT"
if grep -q 'moonshot-rehearsal-candidate-key\|cli-access-secret\|cli-refresh-secret' "$JSON_OUT"; then
  printf 'Kimi unblock rehearsal JSON leaked credential material\n' >&2
  exit 1
fi

printf 'agenthub Kimi unblock rehearsal smoke passed\n'
