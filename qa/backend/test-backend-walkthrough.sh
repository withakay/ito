#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

QA_ROOT="${ITO_BACKEND_QA_ROOT:-$REPO_ROOT/.local/backend-qa}"
DATA_DIR="$QA_ROOT/data"
RUNTIME_DIR="$QA_ROOT/runtime"
SERVER_LOG="$RUNTIME_DIR/server.log"
SERVER_PID_FILE="$RUNTIME_DIR/server.pid"
REQUEST_JSON="$RUNTIME_DIR/ingest-request.json"
HOST="${ITO_BACKEND_BIND:-127.0.0.1}"
PORT="${ITO_BACKEND_PORT:-9010}"
BASE_URL="http://$HOST:$PORT"
ADMIN_TOKEN="${ITO_BACKEND_ADMIN_TOKEN:-dev-admin-token}"
TOKEN_SEED="${ITO_BACKEND_TOKEN_SEED:-dev-token-seed}"
# Track whether the caller explicitly set ITO_BIN before we apply the default.
ITO_BIN_EXPLICIT="${ITO_BIN+yes}"
ITO_BIN="${ITO_BIN:-$REPO_ROOT/target/debug/ito}"
ALLOW_ORGS=(acme globex)

PROJECT_A_ORG="acme"
PROJECT_A_REPO="widgets"
PROJECT_B_ORG="globex"
PROJECT_B_REPO="gadgets"
PROJECT_A_CHANGE="001-01_alpha-feature"
PROJECT_A_SECOND_CHANGE="001-02_logging-pass"
PROJECT_A_MODULE="001"
PROJECT_B_CHANGE="002-01_beta-fix"
INGEST_KEY="qa-key-001"

print_usage() {
  cat <<EOF
Backend QA walkthrough helper

Usage:
  qa/backend/test-backend-walkthrough.sh <command>

Commands:
  reset           Recreate sample backend data
  start           Start ito serve-api against the sample data
  stop            Stop the sample backend
  status          Show runtime paths and server state
  walk            Run the full guided walkthrough
  verify          Run a non-interactive self-check for automation
  health          Curl /api/v1/health
  ready           Curl /api/v1/ready
  changes-a       List changes for acme/widgets
  change-a        Show one change for acme/widgets
  tasks-a         Show tasks for acme/widgets
  modules-a       List modules for acme/widgets
  module-a        Show one module for acme/widgets
  changes-b       List changes for globex/gadgets
  ingest          Post one audit event batch
  retry-ingest    Repeat the same batch to show idempotency
  inspect-files   Show generated backend files
  inspect-audit   Pretty-print the audit log JSONL
  inspect-key     Show the idempotency progress file
  clean           Stop server and remove all QA data

Environment overrides:
  ITO_BACKEND_QA_ROOT   Default: $REPO_ROOT/.local/backend-qa
  ITO_BACKEND_BIND      Default: 127.0.0.1
  ITO_BACKEND_PORT      Default: 9010
  ITO_BACKEND_ADMIN_TOKEN / ITO_BACKEND_TOKEN_SEED
  ITO_BIN               Default: $REPO_ROOT/target/debug/ito
  BACKEND_QA_NO_PAUSE   Set to 1 to skip pauses in walk mode
EOF
}

note() {
  printf '\n== %s ==\n' "$*"
}

run_cmd() {
  printf '+ ' >&2
  printf '%q ' "$@" >&2
  printf '\n' >&2
  "$@"
}

pause_step() {
  if [[ "${BACKEND_QA_NO_PAUSE:-0}" == "1" || ! -t 0 ]]; then
    return
  fi

  printf '\nPress Enter for the next step... '
  read -r _
}

ensure_runtime_dirs() {
  mkdir -p "$DATA_DIR" "$RUNTIME_DIR"
}

project_ito_dir() {
  local org="$1"
  local repo="$2"
  printf '%s/projects/%s/%s/.ito' "$DATA_DIR" "$org" "$repo"
}

audit_log_path() {
  local org="$1"
  local repo="$2"
  printf '%s/.state/audit/events.jsonl' "$(project_ito_dir "$org" "$repo")"
}

idempotency_key_path() {
  local org="$1"
  local repo="$2"
  printf '%s/.state/ingest-keys/%s' "$(project_ito_dir "$org" "$repo")" "$INGEST_KEY"
}

server_running() {
  if [[ ! -f "$SERVER_PID_FILE" ]]; then
    return 1
  fi

  local pid
  pid="$(<"$SERVER_PID_FILE")"

  # Verify the PID is still alive and belongs to our ito process.
  if ! kill -0 "$pid" 2>/dev/null; then
    return 1
  fi

  # Check the process name to avoid matching a recycled PID.
  if ! ps -p "$pid" -o comm= 2>/dev/null | grep -q "ito"; then
    return 1
  fi

  return 0
}

ensure_built_binary() {
  if [[ -x "$ITO_BIN" ]]; then
    return
  fi

  # If ITO_BIN was explicitly set to a custom path but the file is missing or
  # not executable, do not silently build the default binary — that would make
  # the custom override a no-op.  Fail loudly instead.
  if [[ "${ITO_BIN_EXPLICIT:-}" == "yes" ]]; then
    fail "ITO_BIN is set to '$ITO_BIN' but the binary does not exist or is not executable"
  fi

  note "Building ito binary for the walkthrough"
  run_cmd cargo build -p ito-cli --manifest-path "$REPO_ROOT/Cargo.toml"
}

pretty_json() {
  python3 -m json.tool
}

fail() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 1
}

json_value() {
  local path="$1"

  JSON_INPUT="${JSON_INPUT:-}" python3 - "$path" <<'PY'
import json
import os
import sys

path = sys.argv[1]
data = json.loads(os.environ["JSON_INPUT"])

value = data
for part in path.split('.'):
    if isinstance(value, list):
        value = value[int(part)]
    else:
        value = value[part]

if isinstance(value, (dict, list)):
    print(json.dumps(value, sort_keys=True))
else:
    print(value)
PY
}

assert_json_value() {
  local json="$1"
  local path="$2"
  local expected="$3"
  local actual

  actual="$(JSON_INPUT="$json" json_value "$path")"
  if [[ "$actual" != "$expected" ]]; then
    fail "expected JSON path '$path' to be '$expected' but got '$actual'"
  fi
}

assert_json_array_length() {
  local json="$1"
  local expected="$2"
  local actual

  actual="$(JSON_INPUT="$json" python3 - <<'PY'
import json
import os
import sys

data = json.loads(os.environ["JSON_INPUT"])
print(len(data))
PY
)"

  if [[ "$actual" != "$expected" ]]; then
    fail "expected JSON array length '$expected' but got '$actual'"
  fi
}

assert_file_exists() {
  local path="$1"

  if [[ ! -f "$path" ]]; then
    fail "expected file to exist: $path"
  fi
}

assert_line_count() {
  local path="$1"
  local expected="$2"
  local actual

  actual="$(python3 - "$path" <<'PY'
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
# Count lines without loading the whole file into memory.
count = sum(1 for _ in path.open())
print(count)
PY
)"

  if [[ "$actual" != "$expected" ]]; then
    fail "expected $path to have $expected lines but found $actual"
  fi
}

assert_file_contents() {
  local path="$1"
  local expected="$2"
  local actual

  actual="$(<"$path")"
  if [[ "$actual" != "$expected" ]]; then
    fail "expected $path to equal '$expected' but found '$actual'"
  fi
}

seed_change() {
  local org="$1"
  local repo="$2"
  local change_id="$3"
  local proposal_title="$4"
  local task_one_status="$5"
  local task_two_status="$6"
  local with_spec="$7"

  local change_dir
  change_dir="$(project_ito_dir "$org" "$repo")/changes/$change_id"

  mkdir -p "$change_dir"
  cat >"$change_dir/proposal.md" <<EOF
# $proposal_title

This change exists only for backend QA. It gives the shared backend something
real to list over HTTP.
EOF

  cat >"$change_dir/tasks.md" <<EOF
## Tasks

- [$task_one_status] 1.1 Confirm the backend can read seeded files
- [$task_two_status] 1.2 Confirm the backend exposes task progress
EOF

  if [[ "$with_spec" == "yes" ]]; then
    mkdir -p "$change_dir/specs/backend-state"
    cat >"$change_dir/specs/backend-state/spec.md" <<EOF
## ADDED Requirements

### Requirement: QA walkthrough seed data
The backend SHALL expose seeded change data so a human can verify shared-state reads.
EOF
  fi
}

seed_module() {
  local org="$1"
  local repo="$2"
  local module_id="$3"
  local module_name="$4"

  local module_dir
  module_dir="$(project_ito_dir "$org" "$repo")/modules/${module_id}_${module_name}"

  mkdir -p "$module_dir"
  cat >"$module_dir/module.yaml" <<EOF
description: "Backend QA module for manual verification"
EOF
}

write_ingest_request() {
  ensure_runtime_dirs
  cat >"$REQUEST_JSON" <<EOF
{
  "events": [
    {
      "v": 1,
      "ts": "2026-03-07T12:00:00.000Z",
      "entity": "task",
      "entity_id": "1.1",
      "scope": "$PROJECT_A_CHANGE",
      "op": "create",
      "to": "pending",
      "actor": "cli",
      "by": "@qa",
      "ctx": {
        "session_id": "backend-qa-walkthrough"
      }
    }
  ],
  "idempotency_key": "$INGEST_KEY"
}
EOF
}

validate_qa_root() {
  # Guard against accidentally deleting unrelated directories.  QA_ROOT must
  # point somewhere inside the repo's .local/ tree, a system temp directory,
  # or contain a sentinel file we wrote.  The minimum safeguard is that the
  # path must not be empty, must not be '/', $HOME, $REPO_ROOT, or any common
  # system directory.
  local root="$1"
  if [[ -z "$root" ]]; then
    fail "QA_ROOT is empty — refusing to delete"
  fi
  case "$root" in
    / | /tmp | /var | /usr | /etc | /home | /root | "$HOME" | "$REPO_ROOT")
      fail "QA_ROOT='$root' looks like a system path — refusing to delete"
      ;;
  esac

  # Resolve the system temp directory (handles macOS $TMPDIR=/var/folders/...).
  local sys_tmp
  sys_tmp="${TMPDIR:-/tmp}"
  # Strip trailing slash for consistent prefix matching.
  sys_tmp="${sys_tmp%/}"

  # Require the path to be a subdirectory of a known-safe location OR contain
  # a sentinel file written by this script.
  local is_safe=0
  if [[ "$root" == "$REPO_ROOT/.local/"* ]]; then
    is_safe=1
  elif [[ -n "$sys_tmp" && "$root" == "$sys_tmp/"* ]]; then
    is_safe=1
  elif [[ "$root" == /tmp/* ]]; then
    is_safe=1
  elif [[ -f "$root/runtime/README.txt" ]]; then
    # A README we wrote is present — the directory was created by this script.
    is_safe=1
  fi
  if (( is_safe == 0 )); then
    fail "QA_ROOT='$root' is not under .local/, a temp dir, and has no script sentinel — refusing to delete"
  fi
}

reset_data() {
  note "Resetting backend QA data"
  stop_server >/dev/null 2>&1 || true
  validate_qa_root "$QA_ROOT"
  rm -rf "$QA_ROOT"
  ensure_runtime_dirs

  mkdir -p "$(project_ito_dir "$PROJECT_A_ORG" "$PROJECT_A_REPO")/changes"
  mkdir -p "$(project_ito_dir "$PROJECT_A_ORG" "$PROJECT_A_REPO")/modules"
  mkdir -p "$(project_ito_dir "$PROJECT_B_ORG" "$PROJECT_B_REPO")/changes"

  seed_change "$PROJECT_A_ORG" "$PROJECT_A_REPO" "$PROJECT_A_CHANGE" "Alpha feature" " " "x" yes
  seed_change "$PROJECT_A_ORG" "$PROJECT_A_REPO" "$PROJECT_A_SECOND_CHANGE" "Logging pass" "x" " " no
  seed_change "$PROJECT_B_ORG" "$PROJECT_B_REPO" "$PROJECT_B_CHANGE" "Beta fix" " " " " no
  seed_module "$PROJECT_A_ORG" "$PROJECT_A_REPO" "$PROJECT_A_MODULE" "backend"
  write_ingest_request

  cat >"$RUNTIME_DIR/README.txt" <<EOF
Generated by qa/backend/test-backend-walkthrough.sh

Important paths:
- data dir: $DATA_DIR
- server log: $SERVER_LOG
- request body: $REQUEST_JSON
- audit log (after ingest): $(audit_log_path "$PROJECT_A_ORG" "$PROJECT_A_REPO")
- idempotency file (after ingest): $(idempotency_key_path "$PROJECT_A_ORG" "$PROJECT_A_REPO")
EOF

  note "Sample backend data is ready"
  printf 'Data dir: %s\n' "$DATA_DIR"
  printf 'Request body: %s\n' "$REQUEST_JSON"
}

wait_for_server() {
  local attempts=50

  while (( attempts > 0 )); do
    if curl -fsS "$BASE_URL/api/v1/health" >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.2
    attempts=$((attempts - 1))
  done

  printf 'Server did not become healthy. See %s\n' "$SERVER_LOG" >&2

  # Kill and reap the child process, then clean up the pid file so a later
  # invocation of start/status/stop does not mistake a recycled PID for ours.
  if [[ -f "$SERVER_PID_FILE" ]]; then
    local pid
    pid="$(<"$SERVER_PID_FILE")"
    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
    rm -f "$SERVER_PID_FILE"
  fi

  return 1
}

start_server() {
  ensure_runtime_dirs
  ensure_built_binary

  if server_running; then
    note "Backend QA server is already running"
    status
    return
  fi

  : >"$SERVER_LOG"

  note "Starting backend server"
  (
    export ITO_BACKEND_ADMIN_TOKEN="$ADMIN_TOKEN"
    export ITO_BACKEND_TOKEN_SEED="$TOKEN_SEED"
    # Build --allow-org flags dynamically so adding orgs to ALLOW_ORGS works
    # without touching this invocation.
    local org_args=()
    for org in "${ALLOW_ORGS[@]}"; do
      org_args+=(--allow-org "$org")
    done
    exec "$ITO_BIN" serve-api \
      --bind "$HOST" \
      --port "$PORT" \
      --data-dir "$DATA_DIR" \
      "${org_args[@]}"
  ) >"$SERVER_LOG" 2>&1 &

  local pid=$!
  printf '%s\n' "$pid" >"$SERVER_PID_FILE"
  wait_for_server

  printf 'PID: %s\n' "$pid"
  printf 'Base URL: %s\n' "$BASE_URL"
  printf 'Server log: %s\n' "$SERVER_LOG"
}

stop_server() {
  if ! server_running; then
    rm -f "$SERVER_PID_FILE"
    printf 'Backend QA server is not running.\n'
    return
  fi

  local pid
  pid="$(<"$SERVER_PID_FILE")"

  note "Stopping backend server"
  kill "$pid"
  wait "$pid" 2>/dev/null || true
  rm -f "$SERVER_PID_FILE"
  printf 'Stopped PID %s\n' "$pid"
}

status() {
  note "Backend QA status"
  printf 'Repo root: %s\n' "$REPO_ROOT"
  printf 'QA root: %s\n' "$QA_ROOT"
  printf 'Data dir: %s\n' "$DATA_DIR"
  printf 'Base URL: %s\n' "$BASE_URL"
  printf 'Admin token: <redacted>\n'
  printf 'Request body: %s\n' "$REQUEST_JSON"
  if server_running; then
    printf 'Server: running (pid %s)\n' "$(<"$SERVER_PID_FILE")"
  else
    printf 'Server: stopped\n'
  fi
}

require_server() {
  if ! server_running; then
    printf 'Backend QA server is not running. Start it with: qa/backend/test-backend-walkthrough.sh start\n' >&2
    exit 1
  fi
}

auth_header() {
  printf 'Authorization: Bearer %s' "$ADMIN_TOKEN"
}

curl_json() {
  require_server
  run_cmd curl -sS "$@" | pretty_json
}

health() {
  note "Health endpoint"
  curl_json "$BASE_URL/api/v1/health"
}

ready() {
  note "Ready endpoint"
  curl_json "$BASE_URL/api/v1/ready"
}

changes_a() {
  note "Changes for $PROJECT_A_ORG/$PROJECT_A_REPO"
  curl_json -H "$(auth_header)" "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/changes"
}

change_a() {
  note "Change detail for $PROJECT_A_CHANGE"
  curl_json -H "$(auth_header)" "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/changes/$PROJECT_A_CHANGE"
}

tasks_a() {
  note "Tasks for $PROJECT_A_CHANGE"
  curl_json -H "$(auth_header)" "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/changes/$PROJECT_A_CHANGE/tasks"
}

modules_a() {
  note "Modules for $PROJECT_A_ORG/$PROJECT_A_REPO"
  curl_json -H "$(auth_header)" "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/modules"
}

module_a() {
  note "Module detail for $PROJECT_A_MODULE"
  curl_json -H "$(auth_header)" "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/modules/$PROJECT_A_MODULE"
}

changes_b() {
  note "Changes for $PROJECT_B_ORG/$PROJECT_B_REPO"
  curl_json -H "$(auth_header)" "$BASE_URL/api/v1/projects/$PROJECT_B_ORG/$PROJECT_B_REPO/changes"
}

ingest() {
  require_server
  write_ingest_request
  note "Ingesting one audit event into $PROJECT_A_ORG/$PROJECT_A_REPO"
  printf 'Request body file: %s\n' "$REQUEST_JSON"
  curl_json \
    -X POST \
    -H "$(auth_header)" \
    -H "Content-Type: application/json" \
    --data "@$REQUEST_JSON" \
    "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/events"
}

retry_ingest() {
  require_server
  note "Retrying the exact same ingest request"
  curl_json \
    -X POST \
    -H "$(auth_header)" \
    -H "Content-Type: application/json" \
    --data "@$REQUEST_JSON" \
    "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/events"
}

inspect_files() {
  note "Generated backend files under $QA_ROOT"
  run_cmd ls -Ra "$QA_ROOT"
}

inspect_audit() {
  local log_path
  log_path="$(audit_log_path "$PROJECT_A_ORG" "$PROJECT_A_REPO")"

  note "Audit log"
  printf 'Path: %s\n' "$log_path"
  if [[ ! -f "$log_path" ]]; then
    printf 'Audit log does not exist yet. Run the ingest step first.\n'
    return
  fi

  python3 - "$log_path" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
# Iterate line-by-line to avoid loading the entire file into memory at once,
# which can be costly for large audit logs.
with path.open() as fh:
    for index, line in enumerate(fh, start=1):
        line = line.rstrip("\n")
        if not line:
            continue
        print(f"Line {index}:")
        print(json.dumps(json.loads(line), indent=2, sort_keys=True))
PY
}

inspect_key() {
  local key_path
  key_path="$(idempotency_key_path "$PROJECT_A_ORG" "$PROJECT_A_REPO")"

  note "Idempotency progress file"
  printf 'Path: %s\n' "$key_path"
  if [[ ! -f "$key_path" ]]; then
    printf 'Idempotency key file does not exist yet. Run the ingest step first.\n'
    return
  fi
  run_cmd cat "$key_path"
}

clean() {
  note "Cleaning backend QA files"
  stop_server >/dev/null 2>&1 || true
  validate_qa_root "$QA_ROOT"
  rm -rf "$QA_ROOT"
  printf 'Removed %s\n' "$QA_ROOT"
}

walk() {
  note "Guided backend QA walkthrough"
  printf 'This walkthrough seeds two projects, starts the backend, curls the API,\n'
  printf 'and then inspects the filesystem-backed audit log.\n'

  pause_step
  reset_data

  pause_step
  start_server

  pause_step
  health

  pause_step
  ready

  pause_step
  changes_a

  pause_step
  change_a

  pause_step
  tasks_a

  pause_step
  modules_a

  pause_step
  module_a

  pause_step
  changes_b

  pause_step
  ingest

  pause_step
  inspect_audit

  pause_step
  inspect_key

  pause_step
  retry_ingest

  pause_step
  inspect_audit

  pause_step
  inspect_key

  note "Walkthrough complete"
  printf 'The backend is still running so you can continue poking at it.\n'
  printf 'Stop it with: qa/backend/test-backend-walkthrough.sh stop\n'
}

verify() {
  local health_json
  local ready_json
  local changes_a_json
  local changes_b_json
  local module_json
  local ingest_json
  local retry_json
  local audit_path
  local key_path

  trap 'stop_server >/dev/null 2>&1 || true' EXIT

  note "Running automated backend QA verification"
  reset_data
  start_server

  health_json="$(curl -sS "$BASE_URL/api/v1/health")"
  ready_json="$(curl -sS "$BASE_URL/api/v1/ready")"
  changes_a_json="$(curl -sS -H "$(auth_header)" "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/changes")"
  changes_b_json="$(curl -sS -H "$(auth_header)" "$BASE_URL/api/v1/projects/$PROJECT_B_ORG/$PROJECT_B_REPO/changes")"
  module_json="$(curl -sS -H "$(auth_header)" "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/modules/$PROJECT_A_MODULE")"
  ingest_json="$(curl -sS -X POST -H "$(auth_header)" -H "Content-Type: application/json" --data "@$REQUEST_JSON" "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/events")"
  retry_json="$(curl -sS -X POST -H "$(auth_header)" -H "Content-Type: application/json" --data "@$REQUEST_JSON" "$BASE_URL/api/v1/projects/$PROJECT_A_ORG/$PROJECT_A_REPO/events")"

  assert_json_value "$health_json" "status" "ok"
  assert_json_value "$ready_json" "status" "ready"
  assert_json_array_length "$changes_a_json" "2"
  assert_json_value "$changes_a_json" "0.id" "$PROJECT_A_CHANGE"
  assert_json_array_length "$changes_b_json" "1"
  assert_json_value "$changes_b_json" "0.id" "$PROJECT_B_CHANGE"
  assert_json_value "$module_json" "id" "$PROJECT_A_MODULE"
  assert_json_value "$module_json" "name" "backend"
  assert_json_value "$ingest_json" "accepted" "1"
  assert_json_value "$ingest_json" "duplicates" "0"
  assert_json_value "$retry_json" "accepted" "0"
  assert_json_value "$retry_json" "duplicates" "1"

  audit_path="$(audit_log_path "$PROJECT_A_ORG" "$PROJECT_A_REPO")"
  key_path="$(idempotency_key_path "$PROJECT_A_ORG" "$PROJECT_A_REPO")"

  assert_file_exists "$audit_path"
  assert_file_exists "$key_path"
  assert_line_count "$audit_path" "1"
  assert_file_contents "$key_path" "1"

  stop_server >/dev/null 2>&1 || true
  trap - EXIT

  printf 'Verification passed.\n'
  printf 'Audit log: %s\n' "$audit_path"
  printf 'Idempotency key: %s\n' "$key_path"
}

main() {
  local command="${1:-help}"

  case "$command" in
    help|-h|--help)
      print_usage
      ;;
    reset)
      reset_data
      ;;
    start)
      start_server
      ;;
    stop)
      stop_server
      ;;
    status)
      status
      ;;
    walk)
      walk
      ;;
    verify)
      verify
      ;;
    health)
      health
      ;;
    ready)
      ready
      ;;
    changes-a)
      changes_a
      ;;
    change-a)
      change_a
      ;;
    tasks-a)
      tasks_a
      ;;
    modules-a)
      modules_a
      ;;
    module-a)
      module_a
      ;;
    changes-b)
      changes_b
      ;;
    ingest)
      ingest
      ;;
    retry-ingest)
      retry_ingest
      ;;
    inspect-files)
      inspect_files
      ;;
    inspect-audit)
      inspect_audit
      ;;
    inspect-key)
      inspect_key
      ;;
    clean)
      clean
      ;;
    *)
      print_usage >&2
      exit 1
      ;;
  esac
}

main "$@"
