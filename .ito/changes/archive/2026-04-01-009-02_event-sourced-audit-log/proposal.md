# Proposal: Event-Sourced Audit Log

## Why

Ito tracks state across multiple entities (tasks, changes, modules, waves, planning state) but has **no record of how state arrived at its current point**. Every mutation overwrites previous state in-place via markdown string manipulation, making it impossible to:

1. **Audit who changed what and when** -- If an LLM sets a task to "complete" incorrectly, or a manual edit corrupts state, there is no trail to diagnose the problem.
2. **Detect state drift** -- LLMs may edit `tasks.md` directly (bypassing `ito tasks` CLI commands), causing the file-on-disk state to diverge from the intended transition history. Today there is no way to detect or reconcile this drift.
3. **Replay or recover** -- Without an event history, accidental state corruption requires manual reconstruction. An append-only log enables point-in-time replay and recovery.
4. **Enable observability** -- Cross-change and cross-session analytics (e.g., "how long did wave 2 take?", "which tasks were shelved then unshelved?") require structured event data that markdown files cannot provide.

The existing `execution-logs` system (Module 009) records _CLI invocation_ events to a central per-user log. This proposal is complementary: it records _domain state transitions_ at the project level, co-located with the data it describes.

### Design Challenges

The primary complication is **dual mutation paths**:

- **CLI-mediated mutations**: `ito tasks start`, `ito archive`, etc. -- these can emit audit events natively.
- **Direct file edits**: LLMs (or humans) editing `tasks.md`, `module.md`, or other state files directly -- these bypass the CLI and produce no events.

The system must support both paths: promote CLI usage as the primary path (automatic audit logging), and provide a reconciliation mechanism for detecting and recording drift from direct edits.

## What Changes

### New Capability: `audit-log`

An append-only, per-project event log that records every domain state transition across all Ito entities.

**Storage**: `.ito/.state/audit/events.jsonl` -- a single append-only JSONL file per project.

**Event schema** (v1):
```jsonc
{
  "v": 1,                              // Schema version
  "ts": "2026-02-06T14:30:00.000Z",    // UTC timestamp (RFC 3339)
  "entity": "task",                     // Entity type: task | change | module | wave | planning | config
  "entity_id": "2.3",                   // Entity identifier (task id, change id, module id, etc.)
  "scope": "009-02_event-sourced-audit-log", // Scoping context (change_id for tasks/waves, null for global entities)
  "op": "status_change",               // Operation type (see below)
  "from": "pending",                    // Previous state (null for create ops)
  "to": "in-progress",                  // New state
  "actor": "cli",                       // Mutation source: "cli" | "reconcile" | "ralph"
  "by": "@jack",                        // User/agent identity (git user or $USER)
  "meta": {},                           // Optional operation-specific metadata
  "ctx": {                              // Session and git context for traceability
    "session_id": "a1b2c3d4-...",       //   Ito-generated UUID v4 per CLI process group
    "harness_session_id": "ses_abc123", //   Optional: from $ITO_HARNESS_SESSION_ID or $CLAUDE_SESSION_ID
    "branch": "feat/audit-log",         //   Optional: current git branch
    "worktree": "audit-log",            //   Optional: worktree name if not main
    "commit": "3a7f2b1c"               //   Optional: short HEAD commit hash
  }
}
```

**Operation types by entity**:

| Entity | Operations |
|--------|-----------|
| `task` | `create`, `status_change`, `shelve`, `unshelve`, `add` |
| `change` | `create`, `archive` |
| `module` | `create`, `change_added`, `change_completed` |
| `wave` | `unlock` (all predecessor waves complete) |
| `planning` | `decision`, `blocker`, `question`, `note`, `focus_change` |
| `config` | `set`, `unset` |

**Session and git context**: Each event carries an `EventContext` (`ctx` field) with an Ito-generated session ID, optional harness session ID (from `$ITO_HARNESS_SESSION_ID` or `$CLAUDE_SESSION_ID`), git branch, worktree name, and short HEAD commit hash. This enables tracing events back to their originating session and git state without bloating the core event fields. The session ID is persisted to `.state/audit/.session` (gitignored) and reused within a CLI process group.

**Append-only immutability**: The audit log is strictly append-only. Events are never modified, deleted, or rewritten. If a mistake is recorded, a compensating event is appended -- history is never rewritten. This makes the log safe for future migration to database or streaming backends (Kafka, etc.) that enforce the same contract.

**Reconciliation**: A new `ito audit reconcile [--change <id>]` command that:
1. Reads the current file-on-disk state (tasks.md, module.md, etc.)
2. Materializes expected state from the audit log
3. Reports any divergence as diagnostics
4. Optionally appends compensating `reconcile` events to bring the log in sync with the file state (never modifies existing events)

**Integrated validation**: Audit event validation is woven into existing validation flows rather than being a standalone-only step:
1. **`ito validate --changes`**: When validating a change, also validates audit event consistency for that change (structural integrity + state match). Agents get audit validation "for free" as part of their existing validation workflow.
2. **Ralph completion loop**: After checking task completion, also checks audit event consistency for the change. Drift failures are injected into the next iteration prompt.
3. **`ito archive` pre-check**: Before archiving, verifies audit events are consistent. Warns and prompts if drift exists.
4. **`ito audit validate`**: Standalone entry point for full-log validation, CI pipelines, and deep structural checks beyond change-scoped flows.

**Live streaming**: A new `ito audit stream` command that tails the audit log in real-time, displaying events as they're appended. Worktree-aware: discovers all git worktrees and monitors their event files simultaneously, interleaving by timestamp. The stream is informative (a live debugging/monitoring aid), not authoritative -- events from discarded worktree branches will have appeared in the stream but won't exist in merged history.

**Agent instructions**: Updated LLM instruction templates that:
1. Direct agents to use `ito tasks start/complete/shelve` (CLI path) as the primary mutation method
2. When direct edits are unavoidable, instruct agents to run `ito audit reconcile` afterward
3. Audit validation happens automatically as part of `ito validate` -- no separate step needed

### Modified Capabilities

| Capability | Change |
|-----------|--------|
| `cli-tasks` | All task mutation commands (`start`, `complete`, `shelve`, `unshelve`, `add`) emit audit events after successful file write |
| `change-creation` | `create change` emits a `change.create` event |
| `cli-archive` | `archive` emits `change.archive` + `module.change_completed` events |
| `cli-config` | `config set/unset` emits `config.set/unset` events |
| `cli-plan` | Planning state mutations (`decision`, `blocker`, `note`, `focus`, `question`) emit corresponding events |
| `ito-domain` | New `audit` module with event types, serialization, and log writer |
| `ito-core` | New `audit` module with filesystem-backed append writer and reconciliation engine |
| `ito-templates` | Updated agent instruction templates promoting CLI-first mutations and reconciliation |

## Capabilities

### New
- `audit-log` -- Core audit log infrastructure (event schema, JSONL writer, file management, append-only contract)
- `audit-reconcile` -- State drift detection and compensating event generation (append-only)
- `audit-validate` -- Structural and semantic validation of the event log
- `audit-stream` -- Live streaming of audit events with worktree awareness
- `cli-audit` -- CLI commands: `ito audit log`, `ito audit reconcile`, `ito audit validate`, `ito audit stream`

### Modified
- `cli-tasks` -- Emit audit events on task mutations
- `change-creation` -- Emit audit events on change creation
- `cli-archive` -- Emit audit events on archive; audit consistency pre-check
- `cli-config` -- Emit audit events on config mutations
- `cli-plan` -- Emit audit events on planning state mutations
- `cli-validate` -- Embed audit event validation into `ito validate --changes`
- `ito-domain` -- Add audit event domain types, worktree discovery types
- `ito-core` -- Add audit writer, reconciliation engine, stream watcher, validation integration
- `ito-templates` -- Update LLM instructions with audit-aware guidance

## Impact

- **Low risk to existing behavior**: Audit logging is additive -- all existing mutation paths continue to work unchanged. Event emission is best-effort (failures logged but never block the primary operation, matching the `execution-logs` precedent).
- **Storage growth**: JSONL is compact; a typical change lifecycle (20 tasks, 5 waves) produces ~50-100 events (~5-10KB). Archival rotation can be added later if needed.
- **Performance**: Single `append` syscall per event. No locking needed for single-writer CLI usage. File-based advisory locking available if concurrent writers become a concern. Stream command uses filesystem watching with minimal overhead.
- **Git-friendliness**: Append-only JSONL merges cleanly in git (keep both sides). Events are never modified or deleted, only appended. Worktree-based workflows produce per-worktree event files that merge naturally.
- **Validation is transparent**: Audit validation integrates into existing `ito validate`, Ralph completion, and archive pre-checks. Agents don't need to know about audit validation to benefit from it -- drift is caught where validation already happens.
- **LLM workflow change**: Agents gain a new "reconcile after direct edit" instruction. This is a behavioral nudge, not a hard requirement -- the system degrades gracefully if reconciliation is skipped (drift is caught at validation/archive time).
- **Future-proof**: The append-only contract and `AuditWriter` trait abstract over the storage backend. Future migration to a database, Kafka, or other streaming backend requires only a new trait implementation, not changes to the event model or CLI.
- **Worktree support**: `ito audit stream` discovers and monitors all worktrees for a project, enabling cross-session visibility. The stream is informative only -- discarded branches may have been observed but won't appear in merged history.
- **Testing**: The audit writer will be injected via trait (matching `FileSystem` pattern in `FsTaskRepository`) for deterministic testing without real I/O.
