#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_ROOT="${TMPDIR:-$ROOT/target/tmp}"
mkdir -p "$TMP_ROOT"
TMP="$(mktemp -d "$TMP_ROOT/agenthub-shell-ux.XXXXXX")"
PROJECT="$TMP/project"
HOME_DIR="$TMP/home"
CONFIG_DIR="$TMP/config"
OUT="$TMP/shell.out"
ERR="$TMP/shell.err"

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
    printf 'expected shell output to contain: %s\n' "$pattern" >&2
    printf '%s\n' '--- stdout ---' >&2
    sed -n '1,240p' "$OUT" >&2
    printf '%s\n' '--- stderr ---' >&2
    sed -n '1,120p' "$ERR" >&2
    exit 1
  fi
}

mkdir -p "$PROJECT" "$HOME_DIR" "$CONFIG_DIR"

{
  printf '/mode chat\n'
  printf '/mode devops\n'
  printf '/mode project\n'
  printf '/sessions\n'
  printf '/cost\n'
  printf '/balance\n'
  printf '/hosts\n'
  printf '/connect shell-smoke-host\n'
  printf '/provider deepseek\n'
  printf '!printf shell-smoke-ok\n'
  printf '/exit\n'
} | AGENTHUB_HOME="$HOME_DIR" \
    XDG_CONFIG_HOME="$CONFIG_DIR" \
    GIT_CEILING_DIRECTORIES="$TMP" \
    DEEPSEEK_API_KEY="shell-smoke-key" \
    run_agenthub shell >"$OUT" 2>"$ERR"

require_output 'Mode: chat  Git: not required  .agent: not required'
require_output $'workspace_mode\tchat'
require_output $'workspace_mode\tops'
require_output $'workspace_mode\tproject\tpending_runtime'
require_output 'Chats'
require_output 'Chat Usage'
require_output $'provider_balance\tnot_available'
require_output 'Ops hosts:'
require_output $'host\tops-host-shell-smoke-host'
require_output $'selected\tdeepseek'
require_output $'default_provider\tdeepseek'
require_output 'tool_permission tool=shell profile=read-only'
require_output 'shell-smoke-ok'

test ! -e "$PROJECT/.git"
test ! -e "$PROJECT/.agent"
test -f "$CONFIG_DIR/agenthub/config.yaml"
grep -Fq 'default_provider: deepseek' "$CONFIG_DIR/agenthub/config.yaml"
test -d "$HOME_DIR/sessions"

printf 'agenthub shell UX alias smoke passed\n'
