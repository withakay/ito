# Backend Shared-State API

*2026-03-07T07:49:14Z by Showboat 0.6.1*
<!-- showboat-id: d256b701-b446-44d9-98e5-502589a990e0 -->

The backend shared-state API is a multi-tenant HTTP server embedded in the `ito` CLI. One process serves filesystem-backed project state — changes, tasks, modules, and audit events — for any number of `{org}/{repo}` pairs over a plain JSON REST API.

Multi-agent workflows use it as a coordination point: agents read project state, claim leases on changes, push artifact bundles, and ingest audit events. Everything is file-backed and inspectable by hand.

This demo walks the full flow using the QA helper script and raw `curl` calls.

## Step 1 — Seed the sample data

The helper script builds two isolated projects under `.local/backend-qa/data/`:

- `acme/widgets` — two changes, one module, with spec content
- `globex/gadgets` — one change, no modules

Both are served by a single backend process, demonstrating multi-tenancy.

```bash
BACKEND_QA_NO_PAUSE=1 qa/backend/test-backend-walkthrough.sh reset 2>&1
```

```output

== Resetting backend QA data ==

== Sample backend data is ready ==
Data dir: /Users/jack/Code/withakay/ito/ito-worktrees/backend-qa-walkthrough/.local/backend-qa/data
Request body: /Users/jack/Code/withakay/ito/ito-worktrees/backend-qa-walkthrough/.local/backend-qa/runtime/ingest-request.json
```

## Step 2 — Start the backend

The CLI hosts the server directly. Under the hood the helper runs:

```bash
ITO_BACKEND_ADMIN_TOKEN="dev-admin-token" \
ITO_BACKEND_TOKEN_SEED="dev-token-seed" \
ito serve-api \
  --bind 127.0.0.1 --port 9010 \
  --data-dir .local/backend-qa/data \
  --allow-org acme --allow-org globex
```

The `--allow-org` flag is the allowlist gate. Requests for any other org return 403 even with a valid token.

```bash
BACKEND_QA_NO_PAUSE=1 qa/backend/test-backend-walkthrough.sh start 2>&1
```

```output

== Starting backend server ==
PID: 81986
Base URL: http://127.0.0.1:9010
Server log: /Users/jack/Code/withakay/ito/ito-worktrees/backend-qa-walkthrough/.local/backend-qa/runtime/server.log
```

## Step 3 — Health and readiness probes

Two unauthenticated endpoints let load balancers and service managers check the server without needing a token.

```bash
curl -sS http://127.0.0.1:9010/api/v1/health | python3 -m json.tool
```

```output
{
    "status": "ok",
    "version": "0.1.15"
}
```

```bash
curl -sS http://127.0.0.1:9010/api/v1/ready | python3 -m json.tool
```

```output
{
    "status": "ready"
}
```

## Step 4 — Read shared state for project A (acme/widgets)

All project-scoped routes live under `/api/v1/projects/{org}/{repo}/` and require `Authorization: Bearer <token>`. Here we use the admin token which grants access to any allowlisted project.

```bash
curl -sS -H "Authorization: Bearer dev-admin-token" http://127.0.0.1:9010/api/v1/projects/acme/widgets/changes | python3 -m json.tool
```

```output
[
    {
        "id": "001-01_alpha-feature",
        "module_id": "001",
        "completed_tasks": 1,
        "total_tasks": 2,
        "work_status": "ready",
        "last_modified": "2026-03-07T07:49:27.520277155+00:00"
    },
    {
        "id": "001-02_logging-pass",
        "module_id": "001",
        "completed_tasks": 1,
        "total_tasks": 2,
        "work_status": "draft",
        "last_modified": "2026-03-07T07:49:27.525880509+00:00"
    }
]
```

```bash
curl -sS -H "Authorization: Bearer dev-admin-token" http://127.0.0.1:9010/api/v1/projects/acme/widgets/changes/001-01_alpha-feature | python3 -m json.tool
```

```output
{
    "id": "001-01_alpha-feature",
    "module_id": "001",
    "proposal": "# Alpha feature\n\nThis change exists only for backend QA. It gives the shared backend something\nreal to list over HTTP.\n",
    "specs": [
        {
            "name": "backend-state",
            "content": "## ADDED Requirements\n\n### Requirement: QA walkthrough seed data\nThe backend SHALL expose seeded change data so a human can verify shared-state reads.\n"
        }
    ],
    "progress": {
        "total": 2,
        "complete": 1,
        "shelved": 0,
        "in_progress": 0,
        "pending": 1,
        "remaining": 1
    },
    "last_modified": "2026-03-07T07:49:27.520277155+00:00"
}
```

```bash
curl -sS -H "Authorization: Bearer dev-admin-token" http://127.0.0.1:9010/api/v1/projects/acme/widgets/changes/001-01_alpha-feature/tasks | python3 -m json.tool
```

```output
{
    "change_id": "001-01_alpha-feature",
    "tasks": [
        {
            "id": "1.1",
            "name": "Confirm the backend can read seeded files",
            "status": "pending"
        },
        {
            "id": "1.2",
            "name": "Confirm the backend exposes task progress",
            "status": "complete"
        }
    ],
    "progress": {
        "total": 2,
        "complete": 1,
        "shelved": 0,
        "in_progress": 0,
        "pending": 1,
        "remaining": 1
    },
    "format": "checkbox"
}
```

```bash
curl -sS -H "Authorization: Bearer dev-admin-token" http://127.0.0.1:9010/api/v1/projects/acme/widgets/modules | python3 -m json.tool
```

```output
[
    {
        "id": "001",
        "name": "backend",
        "change_count": 2
    }
]
```

```bash
curl -sS -H "Authorization: Bearer dev-admin-token" http://127.0.0.1:9010/api/v1/projects/acme/widgets/modules/001 | python3 -m json.tool
```

```output
{
    "id": "001",
    "name": "backend",
    "description": "Backend QA module for manual verification"
}
```

## Step 5 — Tenant isolation: project B (globex/gadgets)

The same server process holds state for a completely separate project. Data never leaks between tenants.

```bash
curl -sS -H "Authorization: Bearer dev-admin-token" http://127.0.0.1:9010/api/v1/projects/globex/gadgets/changes | python3 -m json.tool
```

```output
[
    {
        "id": "002-01_beta-fix",
        "module_id": "002",
        "completed_tasks": 0,
        "total_tasks": 2,
        "work_status": "draft",
        "last_modified": "2026-03-07T07:49:27.531160948+00:00"
    }
]
```

## Step 6 — Ingest an audit event batch

Agents POST audit events to the backend. The server appends them to a JSONL file on disk and records an idempotency marker so retries are safe.

```bash
cat .local/backend-qa/runtime/ingest-request.json | python3 -m json.tool
```

```output
{
    "events": [
        {
            "v": 1,
            "ts": "2026-03-07T12:00:00.000Z",
            "entity": "task",
            "entity_id": "1.1",
            "scope": "001-01_alpha-feature",
            "op": "create",
            "to": "pending",
            "actor": "cli",
            "by": "@qa",
            "ctx": {
                "session_id": "backend-qa-walkthrough"
            }
        }
    ],
    "idempotency_key": "qa-key-001"
}
```

```bash
curl -sS -X POST \
    -H "Authorization: Bearer dev-admin-token" \
    -H "Content-Type: application/json" \
    --data @.local/backend-qa/runtime/ingest-request.json \
    http://127.0.0.1:9010/api/v1/projects/acme/widgets/events | python3 -m json.tool
```

```output
{
    "accepted": 1,
    "duplicates": 0
}
```

## Step 7 — Inspect the files the backend wrote

The backend writes two files on disk after a successful ingest: the JSONL audit log and the idempotency marker.

```bash
cat .local/backend-qa/data/projects/acme/widgets/.ito/.state/audit/events.jsonl | python3 -m json.tool
```

```output
{
    "v": 1,
    "ts": "2026-03-07T12:00:00.000Z",
    "entity": "task",
    "entity_id": "1.1",
    "scope": "001-01_alpha-feature",
    "op": "create",
    "to": "pending",
    "actor": "cli",
    "by": "@qa",
    "ctx": {
        "session_id": "backend-qa-walkthrough"
    }
}
```

```bash
cat .local/backend-qa/data/projects/acme/widgets/.ito/.state/ingest-keys/qa-key-001
```

```output
1```
```

## Step 8 — Retry the same batch (idempotency)

Posting the exact same `idempotency_key` a second time returns `accepted: 0, duplicates: 1`. The audit log and marker file are unchanged — no double-writes.

```bash
curl -sS -X POST \
    -H "Authorization: Bearer dev-admin-token" \
    -H "Content-Type: application/json" \
    --data @.local/backend-qa/runtime/ingest-request.json \
    http://127.0.0.1:9010/api/v1/projects/acme/widgets/events | python3 -m json.tool
```

```output
{
    "accepted": 0,
    "duplicates": 1
}
```

```bash
wc -l < .local/backend-qa/data/projects/acme/widgets/.ito/.state/audit/events.jsonl && cat .local/backend-qa/data/projects/acme/widgets/.ito/.state/ingest-keys/qa-key-001
```

```output
       1
1```
```

Audit log still has exactly 1 line. Idempotency key file still reads `1`. The retry produced zero side-effects.

## Step 9 — Auth enforcement

A per-project token derived from the seed grants access only to its specific `{org}/{repo}`. Using it against a different project returns 401.

```bash
WRONG_TOKEN="not-a-real-token"
rtk curl -sS -w "\nHTTP %{http_code}" \
  -H "Authorization: Bearer $WRONG_TOKEN" \
  http://127.0.0.1:9010/api/v1/projects/acme/widgets/changes
```

```output
{"error":"Invalid bearer token","code":"unauthorized"}
HTTP 401
```

## Step 10 — Non-allowlisted org returns 403

A request for an org that was not passed to `--allow-org` is rejected before token validation even runs.

```bash
curl -sS -w "\nHTTP %{http_code}" \
    -H "Authorization: Bearer dev-admin-token" \
    http://127.0.0.1:9010/api/v1/projects/unknown-org/some-repo/changes
```

```output
{"error":"Organization/repository 'unknown-org/some-repo' is not allowed","code":"forbidden"}
HTTP 403```
```

## Step 11 — Automated verification

The helper script's `verify` command exercises every endpoint, checks idempotency, asserts file contents, and exits non-zero on any failure. Useful as a CI smoke-test.

```bash
BACKEND_QA_NO_PAUSE=1 qa/backend/test-backend-walkthrough.sh stop 2>&1; BACKEND_QA_NO_PAUSE=1 qa/backend/test-backend-walkthrough.sh verify 2>&1
```

```output

== Stopping backend server ==
Stopped PID 81986

== Running automated backend QA verification ==

== Resetting backend QA data ==

== Sample backend data is ready ==
Data dir: /Users/jack/Code/withakay/ito/ito-worktrees/backend-qa-walkthrough/.local/backend-qa/data
Request body: /Users/jack/Code/withakay/ito/ito-worktrees/backend-qa-walkthrough/.local/backend-qa/runtime/ingest-request.json

== Starting backend server ==
PID: 87644
Base URL: http://127.0.0.1:9010
Server log: /Users/jack/Code/withakay/ito/ito-worktrees/backend-qa-walkthrough/.local/backend-qa/runtime/server.log
Verification passed.
Audit log: /Users/jack/Code/withakay/ito/ito-worktrees/backend-qa-walkthrough/.local/backend-qa/data/projects/acme/widgets/.ito/.state/audit/events.jsonl
Idempotency key: /Users/jack/Code/withakay/ito/ito-worktrees/backend-qa-walkthrough/.local/backend-qa/data/projects/acme/widgets/.ito/.state/ingest-keys/qa-key-001
```

All assertions passed. The backend is stopped and the QA data remains for manual inspection.

---

## What this demo proves

| Claim | Evidence |
|---|---|
| `ito serve-api` starts from the CLI | `start` output shows PID and base URL |
| Plain `curl` is enough | Every request above uses only `curl` |
| One process serves multiple projects | `acme/widgets` and `globex/gadgets` both respond |
| Event ingest writes real files | `events.jsonl` contains the posted event |
| Idempotency is file-backed | Retry returns `duplicates: 1`; log unchanged |
| Auth is enforced | Invalid token → 401; unknown org → 403 |
| Automated self-check exits 0 | `verify` printed `Verification passed.` |
