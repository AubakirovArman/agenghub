#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -n "${TMPDIR:-}" ]]; then
  TMP_ROOT="$TMPDIR"
elif [[ -n "${RUNNER_TEMP:-}" ]]; then
  if command -v cygpath >/dev/null 2>&1; then
    TMP_ROOT="$(cygpath -u "$RUNNER_TEMP")"
  else
    TMP_ROOT="$RUNNER_TEMP"
  fi
elif [[ -d /tmp ]]; then
  TMP_ROOT="/tmp"
else
  TMP_ROOT="$ROOT/target/tmp"
fi
mkdir -p "$TMP_ROOT"
TMP="$(mktemp -d "$TMP_ROOT/agenthub-provider-exit.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT INT TERM

if [[ -z "${AGENTHUB_BIN:-}" ]]; then
  cargo build --manifest-path "$ROOT/Cargo.toml" --locked >/dev/null
  AGENTHUB_BIN="$ROOT/target/debug/agenthub"
fi

PROJECT="$TMP/project"
mkdir -p "$PROJECT"

set +e
(
  cd "$TMP"
  unset DEEPSEEK_API_KEY
  unset DEEPSEEK_API_KEY_FILE
  unset ANTHROPIC_AUTH_TOKEN
  unset ANTHROPIC_AUTH_TOKEN_FILE
  unset DEEPSEEK_API_BASE_URL
  unset DEEPSEEK_BASE_URL
  "$AGENTHUB_BIN" --project "$PROJECT" providers test deepseek > "$TMP/out.txt" 2> "$TMP/err.txt"
)
code=$?
set -e

if [[ "$code" -eq 0 ]]; then
  printf 'expected providers test to exit non-zero when provider credentials are missing\n' >&2
  cat "$TMP/out.txt" >&2 || true
  cat "$TMP/err.txt" >&2 || true
  exit 1
fi

grep -q $'^missing\tdeepseek' "$TMP/out.txt"
grep -q 'provider test failed for `deepseek`' "$TMP/err.txt"

printf 'agenthub provider test exit-code smoke passed\n'
