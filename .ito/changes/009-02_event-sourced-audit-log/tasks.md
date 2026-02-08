# Tasks for: 009-02_event-sourced-audit-log

## Execution Notes

Use `ito tasks status 009-02` to see current progress.
Use `ito tasks next 009-02` to find the next available task.
Use `ito tasks start 009-02 <id>` to begin a task.
Use `ito tasks complete 009-02 <id>` to mark a task done.

All tasks follow TDD: write a failing test first, then implement, then refactor.
Coverage target: 80%.

---

## Wave 1: Domain Model and Event Types
- **Depends On**: None

### Task 1.1: Define AuditEvent struct and related types in ito-domain
- **Files**: `ito-rs/crates/ito-domain/src/audit/mod.rs`, `ito-rs/crates/ito-domain/src/audit/event.rs`
- **Dependencies**: None
- **Action**:
  Create `ito-rs/crates/ito-domain/src/audit/` module with:
  - `AuditEvent` struct: v (u32), ts (String), entity (String), entity_id (String), scope (Option<String>), op (String), from (Option<String>), to (Option<String>), actor (String), by (String), meta (Option<serde_json::Value>), ctx (EventContext)
  - `EntityType` enum: Task, Change, Module, Wave, Planning, Config (serializes to lowercase)
  - `Actor` enum: Cli, Reconcile, Ralph (serializes to lowercase)
  - `EventContext` struct: session_id (String), harness_session_id (Option<String>), branch (Option<String>), worktree (Option<String>), commit (Option<String>)
  - Schema version constant = 1
  - Operation constants module (ops) with all operation strings
  - `AuditEventBuilder` with auto-populated v, ts, by
  - `WorktreeInfo` struct: path (PathBuf), branch (Option<String>), is_main (bool)
  - `TaggedAuditEvent` struct: event (AuditEvent), source (WorktreeInfo)
  - Serde Serialize/Deserialize derives with snake_case
  - Re-export from audit/mod.rs
- **Verify**: `cargo test -p ito-domain --lib audit`
- **Done When**: AuditEvent serializes to/from JSON matching the spec schema; all fields present; builder works
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 1.2: Define AuditWriter trait in ito-domain
- **Files**: `ito-rs/crates/ito-domain/src/audit/writer.rs`
- **Dependencies**: 1.1
- **Action**:
  Create the `AuditWriter` trait in ito-domain:
  - `fn append(&self, event: &AuditEvent) -> Result<()>` (object-safe, Send + Sync)
  - `NoopAuditWriter` implementation that discards events
  - Re-export from audit/mod.rs
- **Verify**: `cargo test -p ito-domain --lib audit`
- **Done When**: Trait compiles, NoopAuditWriter passes basic test, trait is object-safe
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 1.3: Implement state materialization from events
- **Files**: `ito-rs/crates/ito-domain/src/audit/materialize.rs`
- **Dependencies**: 1.1
- **Action**:
  Implement `materialize_state(events: &[AuditEvent]) -> AuditState` that:
  - Replays events chronologically to build a HashMap<(entity_type, entity_id), current_status>
  - Tracks last-seen status per entity from `to` field
  - Returns `AuditState` struct with entity statuses and event count
- **Verify**: `cargo test -p ito-domain --lib audit::materialize`
- **Done When**: Given a sequence of events, produces correct entity status map
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 1.4: Implement reconciliation diff logic in ito-domain
- **Files**: `ito-rs/crates/ito-domain/src/audit/reconcile.rs`
- **Dependencies**: 1.3
- **Action**:
  Implement `compute_drift(audit_state: &AuditState, file_state: &FileState) -> Vec<Drift>` where:
  - `FileState` is a HashMap<(entity_type, entity_id), current_status> built from parsed files
  - `Drift` enum: Missing (in files, not in log), Diverged { entity, log_status, file_status }, Extra (in log, not in files)
  - Pure function, no I/O
  Implement `generate_compensating_events(drifts: &[Drift], scope: Option<&str>) -> Vec<AuditEvent>` that creates reconcile events.
- **Verify**: `cargo test -p ito-domain --lib audit::reconcile`
- **Done When**: Correctly identifies drift between materialized audit state and file state
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 1.5: Implement EventContext resolution logic
- **Files**: `ito-rs/crates/ito-domain/src/audit/context.rs`
- **Dependencies**: 1.1
- **Action**:
  Implement `EventContext` resolution:
  - `resolve_session_id(ito_path: &Path) -> String`: read/create `.state/audit/.session` UUID
  - `resolve_harness_session_id() -> Option<String>`: check env vars `ITO_HARNESS_SESSION_ID`, `CLAUDE_SESSION_ID`, `OPENCODE_SESSION_ID`, `CODEX_SESSION_ID`
  - `resolve_git_context() -> GitContext { branch, worktree, commit }`: run git commands, return None on failure
  - `resolve_user_identity() -> String`: git config user.name or $USER, formatted as @lowercase-hyphenated
  - `EventContext::resolve(ito_path: &Path) -> EventContext`: compose all above
- **Verify**: `cargo test -p ito-domain --lib audit::context`
- **Done When**: EventContext populates session_id (always), harness_session_id (when env available), git fields (when in git repo)
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

---

## Wave 2: Core Infrastructure (Writer, Reader, Reconcile Engine)
- **Depends On**: Wave 1

### Task 2.1: Implement FsAuditWriter in ito-core
- **Files**: `ito-rs/crates/ito-core/src/audit/mod.rs`, `ito-rs/crates/ito-core/src/audit/writer.rs`
- **Dependencies**: None
- **Action**:
  Create `ito-rs/crates/ito-core/src/audit/` module with `FsAuditWriter`:
  - Implements `AuditWriter` trait
  - Constructor takes `ito_path: &Path`, computes log path as `{ito_path}/.state/audit/events.jsonl`
  - `append` serializes event to JSON + newline, appends via OpenOptions::create(true).append(true)
  - Creates parent directories on first write
  - Best-effort: returns Ok even if write fails (logs warning via tracing)
- **Verify**: `cargo test -p ito-core --lib audit`
- **Done When**: Events appended to JSONL file; reading back produces same events; best-effort on failure
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 2.2: Implement audit log reader in ito-core
- **Files**: `ito-rs/crates/ito-core/src/audit/reader.rs`
- **Dependencies**: 2.1
- **Action**:
  Implement `read_audit_events(ito_path: &Path) -> Result<Vec<AuditEvent>>`:
  - Read JSONL line by line, parse each as AuditEvent
  - Skip malformed lines with warnings
  Implement `read_audit_events_filtered(ito_path, filter)`:
  - Filter criteria: entity, scope, op
- **Verify**: `cargo test -p ito-core --lib audit::reader`
- **Done When**: Reads and parses JSONL; filters work; malformed lines skipped
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 2.3: Implement FileState builder and reconcile orchestrator
- **Files**: `ito-rs/crates/ito-core/src/audit/reconcile.rs`
- **Dependencies**: 2.1, 2.2
- **Action**:
  Implement `build_file_state(ito_path, change_id) -> Result<FileState>`:
  - Parse tasks.md for the change, extract task statuses
  Implement `run_reconcile(ito_path, change_id: Option<&str>, fix: bool) -> Result<ReconcileReport>`:
  - Read events.jsonl, materialize state, build file state, compute drift
  - If fix=true, generate and append compensating events
  - Return ReconcileReport { drifts, events_written, scoped_to }
  - Support change-scoped and project-wide reconciliation
- **Verify**: `cargo test -p ito-core --lib audit::reconcile`
- **Done When**: Full round-trip: emit events, change files, reconcile detects and fixes drift
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 2.4: Implement worktree discovery for audit streaming
- **Files**: `ito-rs/crates/ito-core/src/audit/worktree.rs`
- **Dependencies**: None
- **Action**:
  Implement `discover_worktrees(ito_path: &Path) -> Result<Vec<WorktreeInfo>>`:
  - Call `git worktree list --porcelain` to enumerate worktrees
  - Resolve `.ito/.state/audit/events.jsonl` path for each
  - Return only worktrees where events.jsonl exists
  Implement `aggregate_worktree_events(worktrees) -> Result<Vec<(WorktreeInfo, Vec<AuditEvent>)>>`
- **Verify**: `cargo test -p ito-core --lib audit::worktree`
- **Done When**: Discovers worktrees, resolves event files, aggregates events
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

---

## Wave 3: CLI Integration (Emit Events, Commands, Validation)
- **Depends On**: Wave 2

### Task 3.1: Wire AuditWriter into CLI command context
- **Files**: `ito-rs/crates/ito-cli/src/runtime.rs`, `ito-rs/crates/ito-cli/src/app/run.rs`
- **Dependencies**: None
- **Action**:
  Add `FsAuditWriter` creation to the CLI startup path:
  - Add `audit_writer()` method to Runtime that lazily creates FsAuditWriter when ito_path is available
  - Return `Option<&dyn AuditWriter>` (None if ito_path not found)
  - Add `EventContext` resolution (cached per CLI invocation)
- **Verify**: `cargo build -p ito-cli`
- **Done When**: CLI compiles with audit writer threaded through Runtime
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.2: Emit audit events from task mutation commands
- **Files**: `ito-rs/crates/ito-cli/src/commands/tasks.rs`
- **Dependencies**: 3.1
- **Action**:
  Add audit event emission after each successful task mutation:
  - `tasks start`: emit status_change (from: "pending", to: "in-progress")
  - `tasks complete`: emit status_change (from: <previous>, to: "complete")
  - `tasks shelve`: emit status_change (from: <previous>, to: "shelved")
  - `tasks unshelve`: emit status_change (from: "shelved", to: "pending")
  - `tasks add`: emit create (to: "pending")
  Events emitted AFTER successful file write. Best-effort (warn on failure, don't block).
- **Verify**: `cargo test -p ito-cli -- tasks`
- **Done When**: Running task mutations appends events to events.jsonl
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.3: Emit audit events from change creation and archive
- **Files**: `ito-rs/crates/ito-cli/src/commands/create.rs`, `ito-rs/crates/ito-cli/src/app/archive.rs`
- **Dependencies**: 3.1
- **Action**:
  Add audit event emission:
  - `create change`: emit change.create + module.change_added
  - `create module`: emit module.create
  - `archive`: emit change.archive + module.change_completed (BEFORE directory move)
  Best-effort on all emissions.
- **Verify**: `cargo test -p ito-cli -- create && cargo test -p ito-cli -- archive`
- **Done When**: Creating/archiving changes emits audit events
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.4: Emit audit events from config and plan commands
- **Files**: `ito-rs/crates/ito-cli/src/commands/config.rs`, `ito-rs/crates/ito-cli/src/commands/plan.rs`
- **Dependencies**: 3.1
- **Action**:
  Add audit event emission:
  - `config set`: emit config.set (from: old_value, to: new_value)
  - `config unset`: emit config.unset (from: old_value)
  - `plan decision/blocker/question/note/focus`: emit corresponding planning events
  Best-effort on all emissions.
- **Verify**: `cargo build -p ito-cli`
- **Done When**: Config and plan mutations emit audit events
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.5: Implement `ito audit` CLI subcommand group
- **Files**: `ito-rs/crates/ito-cli/src/commands/audit.rs`, `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: 3.1
- **Action**:
  Add `ito audit` command group with subcommands:
  - `ito audit log [--change <id>] [--entity <type>] [--op <op>] [--limit N] [--json]`
  - `ito audit reconcile [--change <id>] [--fix] [--dry-run] [--yes] [--json]`
  - `ito audit validate [--strict] [--check-state] [--json]`
  - `ito audit stats [--change <id>] [--json]`
  - `ito audit stream [--change <id>] [--entity <type>] [--op <op>] [--last N] [--poll-interval ms] [--no-worktrees] [--json]`
  Register in CLI arg parser and command dispatch.
- **Verify**: `cargo test -p ito-cli -- audit`
- **Done When**: All five subcommands work with their flags
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.6: Integrate audit validation into `ito validate --changes`
- **Files**: `ito-rs/crates/ito-cli/src/app/validate.rs`, `ito-rs/crates/ito-core/src/validate/mod.rs`
- **Dependencies**: 3.5
- **Action**:
  Add `validate_change_audit(ito_path, change_id) -> Vec<ValidationIssue>` to ito-core::validate:
  - Read events, materialize state, compare to file state
  - Produce ValidationIssue items for missing/diverged events
  Wire into handle_validate() when --changes is specified.
  Add --audit flag for audit-only validation.
  Add --no-audit flag to skip audit checking.
- **Verify**: `cargo test -p ito-core --lib validate && cargo test -p ito-cli -- validate`
- **Done When**: `ito validate --changes` reports audit drift; `--audit` and `--no-audit` flags work
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.7: Integrate audit validation into Ralph completion loop
- **Files**: `ito-rs/crates/ito-core/src/ralph/validation.rs`, `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: 3.6
- **Action**:
  Add `check_audit_consistency(ito_path, change_id) -> ValidationResult` to ralph::validation.
  Wire into validate_completion() as additional validation step.
  If audit drift detected, inject failure into next iteration prompt.
- **Verify**: `cargo test -p ito-core --lib ralph`
- **Done When**: Ralph refuses COMPLETE if audit events diverge from file state
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.8: Integrate audit pre-check into `ito archive`
- **Files**: `ito-rs/crates/ito-cli/src/app/archive.rs`
- **Dependencies**: 3.6
- **Action**:
  After existing task completion check in handle_archive(), add audit consistency check:
  - Call validate_change_audit(ito_path, change_id)
  - If issues exist, warn and prompt for confirmation
  - --no-validate skips audit validation too
  - On archive success, emit change.archive event (from task 3.3)
- **Verify**: `cargo test -p ito-cli -- archive`
- **Done When**: `ito archive` warns about audit drift before proceeding
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.9: Implement live event streaming with worktree support
- **Files**: `ito-rs/crates/ito-core/src/audit/stream.rs`
- **Dependencies**: 2.4, 3.5
- **Action**:
  Implement poll-based file watcher for audit stream:
  - Default 500ms poll interval, configurable
  - Track file offset, read new lines on change
  - Worktree discovery and multi-file watching
  - TaggedAuditEvent with source worktree
  - --last N for startup events
  - Graceful handling of removed worktrees
  - Periodic worktree re-discovery (30s)
- **Verify**: `cargo test -p ito-core --lib audit::stream`
- **Done When**: New events appear on stream; worktree events interleaved; stream recovers from file changes
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 3.10: Implement semantic validation in `ito audit validate`
- **Files**: `ito-rs/crates/ito-core/src/audit/validate.rs`
- **Dependencies**: 3.5
- **Action**:
  Implement semantic validation checks:
  - No duplicate create events per (entity, entity_id, scope) tuple
  - Non-create events reference prior creates (warning, not error)
  - Valid status transitions for tasks (pending->in-progress, pending->shelved, in-progress->complete, in-progress->shelved, shelved->pending)
  Wire into the `ito audit validate` handler.
- **Verify**: `cargo test -p ito-core --lib audit::validate`
- **Done When**: `ito audit validate` catches duplicate creates, orphaned events, and invalid transitions
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

---

## Wave 4: Agent Instructions and Git Integration
- **Depends On**: Wave 3

### Task 4.1: Add audit guidance to agent instruction templates
- **Files**: `ito-rs/crates/ito-templates/assets/skills/`, `ito-rs/crates/ito-templates/assets/default/project/.ito/AGENTS.md`
- **Dependencies**: None
- **Action**:
  Update agent instruction templates to include:
  - "MUST use `ito tasks start/complete/shelve` CLI commands for state changes"
  - "Run `ito audit reconcile --fix` after any direct file edits to tasks.md"
  - "Run `ito audit validate` at session start to verify log integrity"
  - "Run `ito audit reconcile` before `ito archive` to ensure consistency"
- **Verify**: `cargo test -p ito-templates`
- **Done When**: `ito init` installs instructions that guide LLMs to use CLI and reconcile after direct edits
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Task 4.2: Add .gitignore rules for audit state files
- **Files**: `ito-rs/crates/ito-core/src/installers/mod.rs`
- **Dependencies**: None
- **Action**:
  Ensure `.ito/.gitignore` does NOT exclude `.state/audit/events.jsonl` (it should be git-tracked).
  Add explicit `!.state/audit/` un-ignore if `.state/` is broadly gitignored.
  Add `.state/audit/.session` to gitignore (session ID is process-local).
- **Verify**: Check .gitignore rules against .state/audit/ paths
- **Done When**: events.jsonl is tracked by git; .session is ignored
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

---

## Checkpoints

### Checkpoint 1: Review core domain model and event schema
- **Type**: checkpoint (requires human approval)
- **Dependencies**: 1.1, 1.2, 1.3, 1.4, 1.5
- **Action**: Review AuditEvent schema, operation variants, materialization logic, and reconciliation diff algorithm before proceeding to I/O layer
- **Done When**: User confirms domain model is correct
- **Updated At**: 2026-02-08
- **Status**: [ ] pending

### Checkpoint 2: Verify end-to-end audit trail
- **Type**: checkpoint (requires human approval)
- **Dependencies**: 3.2, 3.3, 3.4, 3.5
- **Action**: Manually test: create a change, start/complete tasks, archive, then run `ito audit log` and `ito audit reconcile` to verify the full pipeline works
- **Done When**: User confirms audit trail is complete and reconciliation works
- **Updated At**: 2026-02-08
- **Status**: [ ] pending
