#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AGENTHUB_BIN="${AGENTHUB_BIN:-$ROOT/target/debug/agenthub}"
WORK="${AGENTHUB_RC_ACCEPTANCE_WORK:-$ROOT/target/rc-acceptance}"
EVIDENCE="${AGENTHUB_RC_ACCEPTANCE_EVIDENCE:-$ROOT/target/dogfood/rc-acceptance-evidence.jsonl}"
ARTIFACTS="$WORK/artifacts"
HOME_DIR="$WORK/home"

if [[ ! -x "$AGENTHUB_BIN" ]]; then
  cargo build --manifest-path "$ROOT/Cargo.toml" --locked >/dev/null
fi

rm -rf "$WORK"
mkdir -p "$ARTIFACTS" "$HOME_DIR" "$(dirname "$EVIDENCE")"
: > "$EVIDENCE"

json_escape() {
  printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g'
}

write_check() {
  local id="$1" source="$2" path="$3"
  printf '{"kind":"check","id":"%s","status":"passed","evidence_type":"acceptance_rehearsal","source":"%s","path":"%s"}\n' \
    "$(json_escape "$id")" "$(json_escape "$source")" "$(json_escape "$path")" >> "$EVIDENCE"
}

write_session() {
  local id="$1" mode="$2" flow="$3" provider="$4" path="$5"
  printf '{"kind":"session","session_id":"%s","mode":"%s","flow":"%s","provider":"%s","status":"passed","cost_receipt":true,"evidence_type":"acceptance_rehearsal","path":"%s"}\n' \
    "$(json_escape "$id")" "$(json_escape "$mode")" "$(json_escape "$flow")" "$(json_escape "$provider")" "$(json_escape "$path")" >> "$EVIDENCE"
}

run_agenthub() {
  AGENTHUB_HOME="$HOME_DIR" "$AGENTHUB_BIN" "$@"
}

CHAT_DIR="$WORK/chat-empty"
mkdir -p "$CHAT_DIR"
run_agenthub --project "$CHAT_DIR" stats > "$ARTIFACTS/stats.txt"
write_check "stats" "agenthub_stats" "$ARTIFACTS/stats.txt"

OPS_DIR="$WORK/ops-empty"
mkdir -p "$OPS_DIR"
run_agenthub --project "$OPS_DIR" ops exec "uptime" --jsonl > "$ARTIFACTS/ops-exec.jsonl"
test ! -e "$OPS_DIR/.agent"
grep -q '"kind":"ops_command_receipt"' "$ARTIFACTS/ops-exec.jsonl"
write_check "ops_no_bootstrap" "ops_exec" "$ARTIFACTS/ops-exec.jsonl"
write_check "ops_receipts" "ops_exec" "$ARTIFACTS/ops-exec.jsonl"
write_session "acceptance-ops-exec" "ops" "ops" "local-shell" "$ARTIFACTS/ops-exec.jsonl"

PROJECT_DIR="$WORK/project-approval"
mkdir -p "$PROJECT_DIR"
run_agenthub --project "$PROJECT_DIR" init > "$ARTIFACTS/project-init.txt"
set +e
run_agenthub --project "$PROJECT_DIR" exec "create docs/headless.md" --jsonl > "$ARTIFACTS/headless-approval.jsonl" 2> "$ARTIFACTS/headless-approval.err"
approval_code=$?
set -e
test "$approval_code" -eq 2
grep -q '"kind":"approval_required"' "$ARTIFACTS/headless-approval.jsonl"
write_check "approval_ux" "headless_exec" "$ARTIFACTS/headless-approval.jsonl"

RESUME_DIR="$WORK/project-resume"
mkdir -p "$RESUME_DIR"
git -C "$RESUME_DIR" -c init.defaultBranch=main init >/dev/null
git -C "$RESUME_DIR" config user.email agenthub@example.invalid
git -C "$RESUME_DIR" config user.name AgentHub
run_agenthub --project "$RESUME_DIR" init > "$ARTIFACTS/resume-init.txt"
mkdir -p "$RESUME_DIR/.agent/policies"
cat > "$RESUME_DIR/.agent/policies/core.yaml" <<'YAML'
commands:
  needs_approval:
    - printf
YAML
printf '# rc acceptance\n' > "$RESUME_DIR/README.md"
git -C "$RESUME_DIR" add README.md .agent
git -C "$RESUME_DIR" commit -m "agenthub baseline" >/dev/null

RESUME_SPEC="$WORK/resume.yaml"
cat > "$RESUME_SPEC" <<'YAML'
task:
  id: rc_acceptance_resume
  type: code.command
workspace:
  type: code.git
  isolation: git_worktree
execution:
  commands:
    - mkdir -p generated
    - printf 'approved\n' > generated/resumed.txt
scope:
  allow:
    - generated/**
verify:
  commands:
    - test -f generated/resumed.txt
transaction:
  commit_on_success: true
  memory_promotion: on_success
  diff_limits:
    max_files_changed: 1
    max_lines_added: 1
    max_lines_deleted: 0
YAML

run_agenthub --project "$RESUME_DIR" run "$RESUME_SPEC" --no-watch > "$ARTIFACTS/resume-blocked.txt" 2>&1
grep -q 'BLOCKED_ON_HUMAN' "$ARTIFACTS/resume-blocked.txt"
tx_id="$(awk 'NR==1 {print $1}' "$ARTIFACTS/resume-blocked.txt")"
run_agenthub --project "$RESUME_DIR" tx resolve "$tx_id" --note "rc acceptance approval" > "$ARTIFACTS/resume-resolve.txt"
run_agenthub --project "$RESUME_DIR" tx resume "$tx_id" > "$ARTIFACTS/resume-resume.txt"
resumed_tx_id="$(awk '$1 == "resumed" {print $3}' "$ARTIFACTS/resume-resume.txt")"
test -n "$resumed_tx_id"
test -f "$RESUME_DIR/generated/resumed.txt"
test -f "$RESUME_DIR/.agent/tx/$tx_id/resume.json"
write_check "resume" "tx_resume" "$ARTIFACTS/resume-resume.txt"
write_session "acceptance-project-resume" "project" "project_edit" "transaction" "$ARTIFACTS/resume-resume.txt"

if [[ -n "$(git -C "$RESUME_DIR" status --short)" ]]; then
  git -C "$RESUME_DIR" add .agent/memory >/dev/null 2>&1 || true
  git -C "$RESUME_DIR" commit -m "agenthub acceptance memory receipts" >/dev/null 2>&1 || true
fi
run_agenthub --project "$RESUME_DIR" undo "$resumed_tx_id" > "$ARTIFACTS/rewind-undo.txt"
test ! -f "$RESUME_DIR/generated/resumed.txt"
write_check "rewind" "tx_undo" "$ARTIFACTS/rewind-undo.txt"

printf 'AgentHub RC acceptance rehearsal passed\n'
printf 'evidence: %s\n' "$EVIDENCE"
printf 'artifacts: %s\n' "$ARTIFACTS"
