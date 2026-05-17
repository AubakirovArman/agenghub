#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d "${TMPDIR:-/tmp}/agenthub-kimi-auth-check.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT INT TERM

FAKE="$TMP/agenthub"
REPORT="$TMP/report.json"
ARTIFACTS="$TMP/artifacts"
cat > "$FAKE" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "providers" && "${2:-}" == "diagnose" && "${3:-}" == "kimi" ]]; then
  cat <<'OUT'
provider	kimi
model	kimi-k2.6
auth_key_source	file:/tmp/.kimi
auth_key_chars	72
auth_key_sha256_12	abc123abc123
auth_key_trimmed_for_request	false
OUT
  exit 0
fi
if [[ "${1:-}" == "providers" && "${2:-}" == "test" && "${3:-}" == "kimi" ]]; then
  case "${KIMI_API_BASE_URL:-}" in
    https://api.moonshot.ai/v1)
      cat <<'OUT'
failed	kimi	auth
endpoint	https://api.moonshot.ai/v1
reason	HTTP provider returned status 401
OUT
      exit 1
      ;;
    https://api.moonshot.cn/v1)
      if [[ "${KIMI_AUTH_FAKE_CHINA_PASSES:-0}" == "1" ]]; then
        cat <<'OUT'
ok	kimi	completion_tokens:1
endpoint	https://api.moonshot.cn/v1
OUT
        exit 0
      fi
      cat <<'OUT'
failed	kimi	auth
endpoint	https://api.moonshot.cn/v1
reason	HTTP provider returned status 401
OUT
      exit 1
      ;;
    *)
      printf 'unexpected endpoint: %s\n' "${KIMI_API_BASE_URL:-}" >&2
      exit 2
      ;;
  esac
  exit 0
fi
printf 'unexpected command: %s\n' "$*" >&2
exit 2
SH
chmod +x "$FAKE"

if AGENTHUB_BIN="$FAKE" \
  AGENTHUB_KIMI_AUTH_REPORT="$REPORT" \
  AGENTHUB_KIMI_AUTH_ARTIFACT_DIR="$ARTIFACTS" \
  "$ROOT/scripts/kimi-auth-check.sh" > "$TMP/out.txt" 2> "$TMP/err.txt"; then
  printf 'expected kimi auth check to exit non-zero when both endpoints fail auth\n' >&2
  exit 1
fi

grep -q 'status: blocked' "$TMP/out.txt"
grep -q 'global: auth_failed' "$TMP/out.txt"
grep -q 'china: auth_failed' "$TMP/out.txt"
grep -q '"status": "blocked"' "$REPORT"
grep -q '"auth_key_sha256_12": "abc123abc123"' "$REPORT"
grep -q '"base_url": "https://api.moonshot.ai/v1"' "$REPORT"
grep -q '"base_url": "https://api.moonshot.cn/v1"' "$REPORT"
test -f "$ARTIFACTS/diagnose.txt"
test -f "$ARTIFACTS/global.txt"
test -f "$ARTIFACTS/china.txt"

REPORT2="$TMP/report-china.json"
ARTIFACTS2="$TMP/artifacts-china"
KIMI_AUTH_FAKE_CHINA_PASSES=1 \
AGENTHUB_BIN="$FAKE" \
AGENTHUB_KIMI_AUTH_REPORT="$REPORT2" \
AGENTHUB_KIMI_AUTH_ARTIFACT_DIR="$ARTIFACTS2" \
  "$ROOT/scripts/kimi-auth-check.sh" > "$TMP/out-china.txt" 2> "$TMP/err-china.txt"

grep -q 'status: passed' "$TMP/out-china.txt"
grep -q 'global: auth_failed' "$TMP/out-china.txt"
grep -q 'china: passed' "$TMP/out-china.txt"
grep -q 'passed_endpoint: https://api.moonshot.cn/v1' "$TMP/out-china.txt"
grep -q '"status": "passed"' "$REPORT2"
grep -q '"passed_endpoint_label": "china"' "$REPORT2"
grep -q '"passed_endpoint": "https://api.moonshot.cn/v1"' "$REPORT2"
grep -q 'KIMI_API_BASE_URL=https://api.moonshot.cn/v1' "$REPORT2"

printf 'agenthub Kimi auth check test passed\n'
