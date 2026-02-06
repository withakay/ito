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
  "meta": {}                            // Optional operation-specific metadata
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

**Reconciliation**: A new `ito audit reconcile [--change <id>]` command that:
1. Reads the current file-on-disk state (tasks.md, module.md, etc.)
2. Materializes expected state from the audit log
3. Reports any divergence as diagnostics
4. Optionally emits compensating `reconcile` events to bring the log in sync with the file state

**Validation**: A new `ito audit validate [--change <id>]` command that:
1. Checks structural integrity of the event log (valid JSON, required fields, monotonic timestamps)
2. Validates that the materialized state matches file-on-disk state
3. Suitable for CI integration (`--strict` mode fails on warnings)

**Agent instructions**: Updated LLM instruction templates that:
1. Direct agents to use `ito tasks start/complete/shelve` (CLI path) as the primary mutation method
2. When direct edits are unavoidable, instruct agents to run `ito audit reconcile` afterward
3. Add `ito audit validate` as a pre-commit or pre-archive verification step

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
- `audit-log` -- Core audit log infrastructure (event schema, JSONL writer, file management)
- `audit-reconcile` -- State drift detection and compensating event generation
- `audit-validate` -- Structural and semantic validation of the event log
- `cli-audit` -- CLI commands: `ito audit log`, `ito audit reconcile`, `ito audit validate`

### Modified
- `cli-tasks` -- Emit audit events on task mutations
- `change-creation` -- Emit audit events on change creation
- `cli-archive` -- Emit audit events on archive
- `cli-config` -- Emit audit events on config mutations
- `cli-plan` -- Emit audit events on planning state mutations
- `ito-domain` -- Add audit event domain types
- `ito-core` -- Add audit writer and reconciliation engine
- `ito-templates` -- Update LLM instructions with audit-aware guidance

## Impact

- **Low risk to existing behavior**: Audit logging is additive -- all existing mutation paths continue to work unchanged. Event emission is best-effort (failures logged but never block the primary operation, matching the `execution-logs` precedent).
- **Storage growth**: JSONL is compact; a typical change lifecycle (20 tasks, 5 waves) produces ~50-100 events (~5-10KB). Archival rotation can be added later if needed.
- **Performance**: Single `append` syscall per event. No locking needed for single-writer CLI usage. File-based advisory locking available if concurrent writers become a concern.
- **Git-friendliness**: Append-only JSONL merges cleanly in git (keep both sides + deduplicate by timestamp+entity_id+op). The `.gitignore` pattern does NOT exclude this file -- the audit log is intentionally version-controlled for team visibility.
- **LLM workflow change**: Agents gain a new "reconcile after direct edit" instruction. This is a behavioral nudge, not a hard requirement -- the system degrades gracefully if reconciliation is skipped (the log is simply incomplete until next reconcile).
- **Testing**: The audit writer will be injected via trait (matching `FileSystem` pattern in `FsTaskRepository`) for deterministic testing without real I/O.
