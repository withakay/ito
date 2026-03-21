# Backend QA Walkthrough

This directory contains the backend shared-state QA walkthrough.

It is a hands-on, human-friendly walkthrough for the file-backed backend API.

It is meant to answer these questions clearly:

- Can I start the backend locally from the CLI?
- Can I hit it with plain `curl` requests?
- Can I see shared state for more than one project?
- When I post audit events, do I get real files on disk?
- If I retry the same event batch, does idempotency work?

The walkthrough uses `qa/backend/test-backend-walkthrough.sh` to keep the setup boring and repeatable, but every meaningful step is still just `ito backend serve` plus `curl`.

For automation, the same script also exposes a `verify` command that runs a compact non-interactive self-check and exits non-zero if anything unexpected happens.

## What It Creates

The helper script seeds a disposable backend data directory under:

```text
.local/backend-qa/
```

Important generated paths:

- Seeded project state: `.local/backend-qa/data/projects/<org>/<repo>/.ito/`
- Backend server log: `.local/backend-qa/runtime/server.log`
- Request payload used for event ingest: `.local/backend-qa/runtime/ingest-request.json`
- Backend-managed audit log after ingest: `.local/backend-qa/data/projects/acme/widgets/.ito/.state/audit/events.jsonl`
- Idempotency progress file: `.local/backend-qa/data/projects/acme/widgets/.ito/.state/ingest-keys/qa-key-001`

## Audit Storage Model

Audit commands now read from routed storage instead of assuming a tracked
`.ito/.state/audit/events.jsonl` on the branch you are editing.

- Backend mode: the backend-managed project store is authoritative, so client
  worktrees do not need a tracked audit JSONL file.
- Local mode: durable audit history lives on the internal `ito/internal/audit`
  branch when Git branch storage is available.
- Fallback mode: if internal branch writes are unavailable, Ito uses an
  untracked local fallback store and continues to validate/reconcile against it.
- Legacy migration: if an older tracked worktree audit log exists, routed audit
  reads import that history into the current durable store; remove the tracked
  worktree file from normal branches after migration so it stops creating churn.

The sample data contains two allowlisted projects served by one backend instance:

- `acme/widgets`
- `globex/gadgets`

That lets you verify shared-state routing and tenant isolation without any extra setup.

## Fastest Way: Guided Walkthrough

From the repo root in the worktree:

```bash
qa/backend/test-backend-walkthrough.sh walk
```

That command:

1. Seeds sample project files.
2. Starts `ito backend serve` against the seeded data.
3. Pauses between the important API calls.
4. Posts an event batch.
5. Shows the audit log and idempotency marker file.
6. Retries the same request so you can see duplicates handled correctly.

If you want it to run straight through without pausing:

```bash
BACKEND_QA_NO_PAUSE=1 qa/backend/test-backend-walkthrough.sh walk
```

## Automation-Friendly Check

If you want one command that behaves like a real integration test, use:

```bash
qa/backend/test-backend-walkthrough.sh verify
```

That command resets the sample data, starts the backend, exercises the important endpoints, verifies event-ingest idempotency, checks the written files on disk, and then stops the backend.

## Manual, Step-by-Step Version

### 1. Reset the sample data

```bash
qa/backend/test-backend-walkthrough.sh reset
```

This rebuilds `.local/backend-qa/` from scratch.

### 2. Start the backend

```bash
qa/backend/test-backend-walkthrough.sh start
```

Under the hood, the helper runs the CLI directly against the sample data dir:

```bash
ITO_BACKEND_ADMIN_TOKEN="dev-admin-token" \
ITO_BACKEND_TOKEN_SEED="dev-token-seed" \
./target/debug/ito backend serve \
  --bind 127.0.0.1 \
  --port 9010 \
  --data-dir ".local/backend-qa/data" \
  --allow-org acme \
  --allow-org globex
```

If `./target/debug/ito` is missing, the helper builds it first.

### 3. Check health and readiness

```bash
qa/backend/test-backend-walkthrough.sh health
qa/backend/test-backend-walkthrough.sh ready
```

Raw `curl` equivalents:

```bash
curl -sS http://127.0.0.1:9010/api/v1/health | python3 -m json.tool
curl -sS http://127.0.0.1:9010/api/v1/ready | python3 -m json.tool
```

Expected behavior:

- `health` returns `status: ok`
- `ready` returns `status: ready`

### 4. Verify project A shared state

```bash
qa/backend/test-backend-walkthrough.sh changes-a
qa/backend/test-backend-walkthrough.sh change-a
qa/backend/test-backend-walkthrough.sh tasks-a
qa/backend/test-backend-walkthrough.sh modules-a
qa/backend/test-backend-walkthrough.sh module-a
```

Raw `curl` equivalents:

```bash
TOKEN="dev-admin-token"
BASE="http://127.0.0.1:9010"

curl -sS -H "Authorization: Bearer $TOKEN" \
  "$BASE/api/v1/projects/acme/widgets/changes" | python3 -m json.tool

curl -sS -H "Authorization: Bearer $TOKEN" \
  "$BASE/api/v1/projects/acme/widgets/changes/001-01_alpha-feature" | python3 -m json.tool

curl -sS -H "Authorization: Bearer $TOKEN" \
  "$BASE/api/v1/projects/acme/widgets/changes/001-01_alpha-feature/tasks" | python3 -m json.tool

curl -sS -H "Authorization: Bearer $TOKEN" \
  "$BASE/api/v1/projects/acme/widgets/modules" | python3 -m json.tool

curl -sS -H "Authorization: Bearer $TOKEN" \
  "$BASE/api/v1/projects/acme/widgets/modules/001" | python3 -m json.tool
```

Expected behavior:

- `acme/widgets` returns two changes.
- The change detail includes proposal text, spec data, and task progress.
- The tasks endpoint returns the parsed checkbox tasks.
- The module endpoint returns module `001` with the seeded description.

### 5. Verify project isolation with project B

```bash
qa/backend/test-backend-walkthrough.sh changes-b
```

Raw `curl` equivalent:

```bash
curl -sS -H "Authorization: Bearer $TOKEN" \
  "$BASE/api/v1/projects/globex/gadgets/changes" | python3 -m json.tool
```

Expected behavior:

- `globex/gadgets` returns different change data from `acme/widgets`.
- One backend process is serving two separate projects correctly.

### 6. Post one event batch

```bash
qa/backend/test-backend-walkthrough.sh ingest
```

The request body lives here so you can inspect it before or after sending it:

```text
.local/backend-qa/runtime/ingest-request.json
```

Before ingesting, run `ito audit validate --change <change-id>` in the source
repo or worktree if you are trying to compare backend state with local audit
history. That catches local audit drift before you treat the backend ingest as
the new source of truth.

Raw `curl` equivalent:

```bash
curl -sS -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  --data @.local/backend-qa/runtime/ingest-request.json \
  "$BASE/api/v1/projects/acme/widgets/events" | python3 -m json.tool
```

Expected behavior:

- Response shows `accepted: 1`
- Response shows `duplicates: 0`
- The backend-managed project audit store is authoritative; no working-branch audit log is required in the client repo.

### 7. Inspect the files the backend wrote

```bash
qa/backend/test-backend-walkthrough.sh inspect-files
qa/backend/test-backend-walkthrough.sh inspect-audit
qa/backend/test-backend-walkthrough.sh inspect-key
```

You can also inspect them directly:

```bash
cat .local/backend-qa/data/projects/acme/widgets/.ito/.state/audit/events.jsonl
cat .local/backend-qa/data/projects/acme/widgets/.ito/.state/ingest-keys/qa-key-001
```

Expected behavior:

- The backend-managed `events.jsonl` exists and contains one JSON object per line.
- The ingest key file exists and contains `1`.

### 8. Retry the exact same request

```bash
qa/backend/test-backend-walkthrough.sh retry-ingest
```

Expected behavior:

- Response shows `accepted: 0`
- Response shows `duplicates: 1`
- The audit log still has only one line.
- The ingest key file still reports `1`.

### 9. Stop the backend when you are done

```bash
qa/backend/test-backend-walkthrough.sh stop
```

If you want to remove all generated QA files too:

```bash
qa/backend/test-backend-walkthrough.sh clean
```

## What This Walkthrough Proves

- `ito backend serve` serves filesystem-backed project state.
- One backend process can serve more than one project namespace.
- Plain `curl` requests are enough to exercise the implemented logic.
- Event ingest appends to the backend-managed audit log for that project.
- Idempotency is file-backed and survives request retries.
- The generated files are easy to inspect by hand.

## Notes

- This walkthrough uses the CLI-hosted backend instead of Docker because it makes the written files easiest to inspect locally.
- If you prefer containers, `docker compose -f docker-compose.backend.yml up -d` still works for a backend runtime, but the QA walkthrough in this doc is centered on the local filesystem-backed flow.
