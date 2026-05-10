# Design: Event-Sourced Audit Log

## Decision Record

### D1: Single JSONL file vs. date-partitioned files

**Options:**
- A) Single `events.jsonl` per project (simpler, one file to read/append)
- B) Date-partitioned `YYYY-MM-DD.jsonl` files (like Spool)

**Decision: A -- Single file.**

Rationale: Ito projects are small-to-medium scope (tens of changes, hundreds of tasks). A typical project lifecycle generates hundreds, not millions, of events. A single file is simpler to reason about, simpler to merge in git, and avoids the complexity of multi-file materialization. If a project grows large enough to warrant partitioning, it can be added later by splitting on size thresholds.

### D2: Event log location

**Options:**
- A) `.ito/.state/audit/events.jsonl` (under the existing `.state/` convention)
- B) `.ito/audit/events.jsonl` (top-level)
- C) Per-change event files (`.ito/changes/<id>/events.jsonl`)

**Decision: A -- Under `.state/`.**

Rationale: `.state/` already holds runtime state (Ralph state, change allocations). The audit log is derived operational state, not a user-authored artifact. Keeping it under `.state/` maintains the separation between authored content (specs, proposals, tasks) and operational state. Unlike `.state/ralph/` which is per-change, the audit log is global because events span entities (modules, config, planning) that are not scoped to a single change.

### D3: Git-tracked vs. gitignored

**Decision: Git-tracked.**

Rationale: The audit log is valuable for team visibility and code review. Reviewers can see what state transitions occurred during implementation. Unlike execution logs (which are per-user telemetry), audit events describe project-level domain transitions that are part of the project's history. Append-only JSONL merges cleanly in git (keep both sides).

### D4: Best-effort vs. transactional event emission

**Decision: Best-effort, matching the `execution-logs` precedent.**

Rationale: The primary operation (e.g., updating `tasks.md`) MUST NOT fail because audit logging fails. Event emission wraps the append in a catch-and-warn: if the write fails, the primary operation succeeds and a warning is emitted to stderr. This prevents the audit system from becoming a reliability liability.

### D5: Actor identification

**Decision: Three actor types: `cli`, `reconcile`, `ralph`.**

- `cli` -- Event emitted as part of a normal CLI command (e.g., `ito tasks start`)
- `reconcile` -- Compensating event emitted by `ito audit reconcile` to record drift
- `ralph` -- Event emitted by the Ralph automation loop

The `by` field captures the human/agent identity using the same resolution as Spool: `git config user.name` falling back to `$USER`, formatted as `@lowercase-hyphenated`.

### D6: Reconciliation strategy

**Options:**
- A) Full replay + diff (materialize expected state from log, diff against file state)
- B) Hash-based change detection (store file hash after each CLI write, compare on reconcile)

**Decision: A -- Full replay + diff.**

Rationale: Hash-based detection requires storing hashes after every write, adding complexity to every mutation path. Full replay is simpler: the reconcile command reads the log, materializes expected state, parses current file state, and reports/emits compensating events for any divergence. The cost is proportional to log size, which is small.

### D7: Reconciliation scope

The reconcile command operates at two levels:

1. **Change-scoped** (`ito audit reconcile --change <id>`) -- Reconciles task and wave state for a specific change against its `tasks.md`.
2. **Project-scoped** (`ito audit reconcile`) -- Reconciles all changes plus module and planning state.

Compensating events use `actor: "reconcile"` and include a `meta.reason` field describing the detected drift (e.g., `"task 2.1 status is 'complete' in file but 'in-progress' in log"`).

### D8: Schema versioning

The event schema includes a `v` field (integer). Version 1 is the initial schema. Future schema changes:
- **Additive** (new optional fields) -- Same version, readers ignore unknown fields.
- **Breaking** (field renames, type changes) -- Bump version. Materialization handles migration by version-switching during replay.

### D9: Concurrency model

**Decision: No locking for v1.**

Rationale: Ito CLI commands are single-writer (one user or agent runs one command at a time). The audit log appends a single JSON line per operation. POSIX `O_APPEND` guarantees atomic appends for writes under PIPE_BUF (4KB on Linux, more than enough for one event). If concurrent writers become a concern (e.g., multiple agents), file-based advisory locking can be added in a follow-up change, following the pattern already researched from Spool.

### D10: What direct-edit scenarios to reconcile

The reconcile engine checks these specific drift cases:

| Entity | Drift Detection |
|--------|----------------|
| Task status | File says `complete` but log says `in-progress` (or vice versa) |
| Task creation | Task exists in file but has no `create` event in log |
| Task deletion | Task has events in log but does not exist in file |
| Wave completion | All tasks in wave are complete in file but no `wave.unlock` event for dependent waves |
| Change existence | Change directory exists but has no `change.create` event |
| Module membership | Change listed in `module.md` but no `module.change_added` event |

Planning state and config mutations are NOT reconciled (they are append-only or replacement operations where drift detection is impractical without content hashing).

### D11: Append-only immutability -- no rewriting history

**Decision: The audit log is strictly append-only. Events are never modified, deleted, or rewritten.**

Rationale: This is a true audit log, not a mutable state store. If an event was emitted in error (e.g., a reconciliation recorded the wrong `from` status), the correct response is to append a compensating event that records the correction -- not to edit or remove the original. This preserves the complete history of what happened and when, including mistakes. The materialization function replays all events in order, so the latest compensating event naturally becomes the "current truth."

Practical implications:
- `ito audit reconcile --fix` appends `Reconciled` events; it never modifies existing lines.
- There is no `ito audit delete` or `ito audit edit` command.
- If the JSONL file is corrupted mid-line (e.g., partial write from crash), the reader skips malformed lines and logs a warning. The next successful write appends a valid line after the corruption.
- Future backends (database, Kafka, etc.) will implement the same append-only contract via the `AuditWriter` trait.

### D12: Integrated validation -- audit checks embedded in existing validation flows

**Options:**
- A) Standalone validation (`ito audit validate` only, separate from `ito validate`)
- B) Integrated validation (audit checks woven into `ito validate`, Ralph completion checks, and archive pre-checks, with `ito audit validate` as the standalone entry point)

**Decision: B -- Integrated validation.**

Rationale: The user already asks LLMs to validate changes rigorously via `ito validate`, Ralph completion loops, and pre-archive checks. Adding a separate audit validation step that agents must remember to call creates a gap. Instead, audit event validation should run as part of the existing validation surfaces:

1. **`ito validate --changes`** (existing command): When validating a change, also validate that audit events for that change are structurally sound and that materialized state matches file-on-disk state. Uses the same `ValidationIssue` / `ValidationReport` infrastructure.
2. **Ralph completion validation** (automated loop): After checking task completion and running project validation, also check audit event consistency for the change being completed. If drift is detected, the failure is injected into the next Ralph iteration prompt.
3. **`ito archive` pre-check** (archive gate): Before archiving a change, verify audit events are consistent. Warn and prompt if drift exists.
4. **`ito audit validate`** (standalone): Remains available for explicit full-log validation, CI pipelines, and deep structural checks that don't belong in the change-scoped flows above.

This means agents get audit validation "for free" when they run `ito validate` -- they don't need to know about the audit system to benefit from it.

### D13: Live stream command with worktree awareness

**Decision: Add `ito audit stream` for real-time event monitoring across worktrees.**

The stream command tails the audit event file(s) and displays new events as they're appended, similar to `tail -f` but with structured formatting.

**Worktree awareness**: When a project uses `git worktree`, each worktree has its own `.ito/.state/audit/events.jsonl`. The stream command discovers all worktrees via `git worktree list --porcelain` and monitors all event files simultaneously, interleaving events by timestamp and tagging each with its worktree name/branch.

**Design constraints:**
- **Stream is informative, not authoritative.** Events from worktrees that are later discarded (branch deleted without merge) will have appeared in the stream but won't exist in the merged mainline log. This is acceptable -- the stream is a live debugging/monitoring aid, not the source of truth.
- **The JSONL file in each worktree IS the source of truth for that worktree.** Events become part of the canonical project history only when they are merged into the main branch via git.
- **File watching**: Use `notify` crate (or poll-based fallback) to watch for file modifications. When a watched file grows, read and parse the new lines.
- **Output format**: Default is human-readable (timestamp, entity, operation, actor, branch/worktree tag). `--json` emits raw JSONL. `--filter` supports entity/scope/op filtering (same as `ito audit log`).
- **Graceful handling**: If a worktree is removed while streaming, the watcher drops it silently. New worktrees created during streaming are NOT auto-discovered (restart required).

### D14: Session and git context in events

**Decision: Each event carries an `EventContext` struct with session ID, git branch, worktree path, and HEAD commit hash.**

The context is captured at event-write time and stored in a `ctx` field on `AuditEvent`. This enables tracing events back to their originating session, branch, and commit range without bloating the core event fields.

**Session ID strategy:**
- Ito generates a session ID (UUID v4) once per CLI process group. The first CLI command in a session generates the ID and writes it to `{ito_path}/.state/audit/.session`. Subsequent commands in the same logical session reuse it.
- If a harness session ID is available (e.g., via `$ITO_HARNESS_SESSION_ID` env var or `$CLAUDE_SESSION_ID`), it is captured in a separate `harness_session_id` field. This enables correlation between audit events and the LLM session that triggered them.
- If no harness session ID is available, the field is `None` -- no degradation occurs.

**Git context fields (all optional, best-effort):**
- `branch` -- Current branch name from `git symbolic-ref --short HEAD` (None if detached)
- `worktree` -- Worktree name/path if not the main worktree (None if main)
- `commit` -- HEAD commit hash (short, 8 chars) from `git rev-parse --short HEAD`

**Design rationale:**
- All context fields are `Option<String>` to handle edge cases (detached HEAD, bare repos, no git).
- Context is captured once per CLI invocation and reused for all events in that invocation (not re-resolved per event).
- The `ctx` field is a nested JSON object, keeping the top-level event schema flat and clean.
- The session file (`.state/audit/.session`) is gitignored -- it's process-local state, not project history.

**Example event with context:**
```json
{
  "v": 1,
  "ts": "2026-02-07T14:30:00Z",
  "entity": "task",
  "entity_id": "2.1",
  "scope": "009-02_event-sourced-audit-log",
  "op": "status_change",
  "from": "pending",
  "to": "in-progress",
  "actor": "cli",
  "by": "@jack",
  "ctx": {
    "session_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "harness_session_id": "ses_abc123",
    "branch": "feat/audit-log",
    "worktree": "audit-log",
    "commit": "3a7f2b1c"
  }
}
```

## Crate Architecture

```
ito-domain/src/audit/
├── mod.rs          -- Module root, re-exports
├── event.rs        -- AuditEvent struct, AuditOperation enum, Entity enum
├── materialize.rs  -- Replay log -> materialized state (HashMap per entity)
└── reconcile.rs    -- Diff materialized state vs file state, produce compensating events

ito-core/src/audit/
├── mod.rs          -- Module root
├── writer.rs       -- AuditWriter trait + FsAuditWriter (append to JSONL)
└── reconcile.rs    -- Orchestration: load log, load file state, call domain reconcile, write events

ito-cli/src/commands/audit.rs
                    -- CLI: `ito audit log`, `ito audit reconcile`, `ito audit validate`
```

### Dependency flow

```
ito-cli -> ito-core -> ito-domain
              |             |
              v             v
         AuditWriter   AuditEvent, materialize, reconcile logic
```

Each CLI mutation command (tasks, archive, create, config, plan) receives an `Option<&dyn AuditWriter>`. When `Some`, it emits the event after successful primary operation. When `None` (e.g., testing without audit), it skips emission. This keeps audit logging opt-in at the call site and avoids global state.

### Integration pattern for existing commands

```rust
// In ito-cli/src/commands/tasks.rs (pseudocode)
fn handle_task_start(change_id, task_id, audit_writer: Option<&dyn AuditWriter>) -> Result<()> {
    // 1. Existing logic: read, validate, update tasks.md
    let old_status = task.status.clone();
    let contents = update_enhanced_task_status(&contents, &task_id, TaskStatus::InProgress, now);
    fs::write(&tasks_path, &contents)?;

    // 2. NEW: Emit audit event (best-effort)
    if let Some(writer) = audit_writer {
        if let Err(e) = writer.emit(AuditEvent {
            entity: Entity::Task,
            entity_id: task_id.clone(),
            scope: Some(change_id.clone()),
            op: AuditOperation::StatusChange,
            from: Some(old_status.to_string()),
            to: Some("in-progress".into()),
            actor: Actor::Cli,
            ..AuditEvent::now()
        }) {
            eprintln!("warning: audit log write failed: {e}");
        }
    }
    Ok(())
}
```

## Agent Instruction Updates

The following instruction templates in `ito-templates` will be updated:

### Task mutation instructions
Current: Agents may edit `tasks.md` directly or use CLI commands.
Updated: Agents MUST use `ito tasks start/complete/shelve/unshelve/add` for all task mutations. If a direct edit is unavoidable, agents MUST run `ito audit reconcile --change <id>` immediately after.

### Pre-archive checklist
Current: Verify all tasks complete.
Updated: Add `ito audit validate --change <id>` to the pre-archive checklist.

### Session start instructions
Add: "Run `ito audit validate` at session start to verify audit log integrity."

## Testing Strategy

- **Unit tests**: `AuditEvent` serialization round-trip, `materialize()` with known event sequences, `reconcile()` with crafted drift scenarios.
- **Integration tests**: CLI commands emit events (capture JSONL output), reconcile detects injected drift, validate catches structural errors.
- **Property tests**: Arbitrary event sequences always produce valid materialized state (no panics).
- **Coverage target**: 80% per the project default.
- **TDD workflow**: RED/GREEN/REFACTOR per project convention.
