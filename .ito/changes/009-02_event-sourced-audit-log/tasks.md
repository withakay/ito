# Tasks for: 009-02_event-sourced-audit-log

## Execution Notes

Use `ito tasks status 009-02` to see current progress.
Use `ito tasks next 009-02` to find the next available task.
Use `ito tasks start 009-02 <id>` to begin a task.
Use `ito tasks complete 009-02 <id>` to mark a task done.

All tasks follow TDD: write a failing test first, then implement, then refactor.
Coverage target: 80%.

## Wave 1: Domain Model and Event Types
- **Depends On**: None

### Task 1.1: Define AuditEvent struct and AuditOperation enum in ito-domain
- **Files**: `ito-rs/crates/ito-domain/src/audit/mod.rs`, `ito-rs/crates/ito-domain/src/audit/event.rs`
- **Dependencies**: None
- **Action**:
  Create `ito-rs/crates/ito-domain/src/audit/` module with:
  - `AuditEvent` struct: v (u32), op (AuditOperation), entity_type (String), entity_id (String), change_id (Option<String>), ts (DateTime<Utc>), actor (AuditActor), data (serde_json::Value)
  - `AuditOperation` enum: TaskStatusChanged, TaskCreated, ChangeCreated, ChangeArchived, ModuleCreated, ModuleUpdated, WaveCompleted, SpecsUpdated, Reconciled
  - `AuditActor` struct: kind (ActorKind enum: Cli/Reconcile/Ralph/Agent), name (Option<String>)
  - Serde Serialize/Deserialize derives with snake_case
  - Constructor `AuditEvent::new(op, entity_type, entity_id, change_id, actor, data)` that stamps ts=Utc::now() and v=1
  - Re-export from audit/mod.rs
- **Verify**: `cargo test -p ito-domain --lib audit`
- **Done When**: AuditEvent serializes to/from JSON matching the spec schema; all fields present
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 1.2: Define AuditWriter trait in ito-domain
- **Files**: `ito-rs/crates/ito-domain/src/audit/writer.rs`
- **Dependencies**: 1.1
- **Action**:
  Create the `AuditWriter` trait in ito-domain:
  ```rust
  pub trait AuditWriter: Send + Sync {
      fn write_event(&self, event: &AuditEvent) -> Result<()>;
  }
  ```
  Add a `NoopAuditWriter` implementation that discards events (for testing and when audit is disabled).
  Re-export from audit/mod.rs.
- **Verify**: `cargo test -p ito-domain --lib audit`
- **Done When**: Trait compiles, NoopAuditWriter passes basic test
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 1.3: Implement state materialization from events
- **Files**: `ito-rs/crates/ito-domain/src/audit/materialize.rs`
- **Dependencies**: 1.1
- **Action**:
  Implement `materialize_state(events: &[AuditEvent]) -> AuditState` that:
  - Replays events chronologically to build a HashMap<(entity_type, entity_id), current_status>
  - Tracks last-seen status per entity from `data.status` or `data.new_status` fields
  - Returns `AuditState` struct with entity statuses and event count
  This is used by reconciliation to compare audit log state vs file state.
- **Verify**: `cargo test -p ito-domain --lib audit::materialize`
- **Done When**: Given a sequence of events, produces correct entity status map; handles out-of-order gracefully
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 1.4: Implement reconciliation diff logic in ito-domain
- **Files**: `ito-rs/crates/ito-domain/src/audit/reconcile.rs`
- **Dependencies**: 1.3
- **Action**:
  Implement `compute_drift(audit_state: &AuditState, file_state: &FileState) -> Vec<Drift>` where:
  - `FileState` is a HashMap<(entity_type, entity_id), current_status> built from parsed files
  - `Drift` enum: Missing (in files, not in log), Diverged { entity, log_status, file_status }, Extra (in log, not in files)
  - Pure function, no I/O
  Implement `generate_reconciliation_events(drifts: &[Drift], actor: &AuditActor) -> Vec<AuditEvent>` that creates Reconciled events for each drift.
- **Verify**: `cargo test -p ito-domain --lib audit::reconcile`
- **Done When**: Correctly identifies drift between materialized audit state and file state
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

## Wave 2: Core Infrastructure (Writer, File I/O)
- **Depends On**: Wave 1

### Task 2.1: Implement JsonlAuditWriter in ito-core
- **Files**: `ito-rs/crates/ito-core/src/audit/mod.rs`, `ito-rs/crates/ito-core/src/audit/writer.rs`
- **Dependencies**: None
- **Action**:
  Create `ito-rs/crates/ito-core/src/audit/` module with `JsonlAuditWriter`:
  - Implements `AuditWriter` trait
  - Constructor takes `ito_path: &Path`, computes log path as `{ito_path}/.state/audit/events.jsonl`
  - `write_event` serializes event to JSON + newline, appends to file using `OpenOptions::new().create(true).append(true)`
  - Creates parent directories if they don't exist
  - Best-effort: returns Ok even if write fails (logs warning via tracing, never panics)
  - Add `read_events(path: &Path) -> Result<Vec<AuditEvent>>` standalone function that reads and parses JSONL, skipping malformed lines with warnings
- **Verify**: `cargo test -p ito-core --lib audit`
- **Done When**: Events are appended to JSONL file atomically; reading back produces same events; malformed lines are skipped
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 2.2: Implement FileState builder from existing repositories
- **Files**: `ito-rs/crates/ito-core/src/audit/reconcile.rs`
- **Dependencies**: None
- **Action**:
  Implement `build_file_state(change_repo, task_repo, module_repo, ito_path) -> Result<FileState>` that:
  - Iterates all changes via ChangeRepository, extracts (change_id, work_status)
  - Iterates all tasks per change via TaskRepository, extracts (task_id, status) scoped to change
  - Iterates all modules via ModuleRepository
  - Returns `FileState` compatible with domain reconcile::compute_drift
  This bridges the existing repository pattern to the audit reconciliation system.
- **Verify**: `cargo test -p ito-core --lib audit::reconcile`
- **Done When**: FileState correctly reflects current task/change/module statuses from disk
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 2.3: Implement full reconcile orchestrator in ito-core
- **Files**: `ito-rs/crates/ito-core/src/audit/reconcile.rs`
- **Dependencies**: 2.1, 2.2
- **Action**:
  Implement `run_reconcile(ito_path, change_id: Option<&str>, fix: bool) -> Result<ReconcileReport>` that:
  1. Reads events.jsonl via `read_events`
  2. Materializes audit state
  3. Builds file state (scoped to change_id if provided)
  4. Computes drift
  5. If `fix=true`, generates reconciliation events and appends them to the log
  6. Returns `ReconcileReport { drifts, events_written, scoped_to }`
- **Verify**: `cargo test -p ito-core --lib audit::reconcile`
- **Done When**: Full round-trip: emit events, manually change files, reconcile detects and fixes drift
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

## Wave 3: CLI Integration (Emit Events from Commands)
- **Depends On**: Wave 2

### Task 3.1: Wire AuditWriter into CLI command context
- **Files**: `ito-rs/crates/ito-cli/src/commands/mod.rs`, `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: None
- **Action**:
  Add `JsonlAuditWriter` creation to the CLI startup path:
  - Create writer when ito_path is available
  - Pass as `Option<&dyn AuditWriter>` to command handlers that mutate state
  - If ito_path is not found (e.g., `ito init`), pass None
  This is the single integration point; individual commands receive the writer.
- **Verify**: `cargo build -p ito-cli`
- **Done When**: CLI compiles with audit writer threaded through; no functional change yet
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 3.2: Emit audit events from task mutation commands
- **Files**: `ito-rs/crates/ito-cli/src/commands/tasks.rs`
- **Dependencies**: 3.1
- **Action**:
  Add audit event emission to these task command handlers:
  - `tasks start`: emit TaskStatusChanged { old: "pending", new: "in-progress", task_id, change_id }
  - `tasks complete`: emit TaskStatusChanged { old: current_status, new: "complete", task_id, change_id }
  - `tasks shelve`: emit TaskStatusChanged { old: current_status, new: "shelved", task_id, change_id }
  - `tasks unshelve`: emit TaskStatusChanged { old: "shelved", new: "pending", task_id, change_id }
  - `tasks add`: emit TaskCreated { task_id, wave, change_id, name }
  - `tasks init`: emit TaskCreated for each initial task in the template
  Events emitted AFTER successful file write (best-effort: if event write fails, command still succeeds).
- **Verify**: `cargo test -p ito-cli -- tasks && test -f .ito/.state/audit/events.jsonl`
- **Done When**: Running `ito tasks start/complete/shelve/unshelve/add` appends events to JSONL
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 3.3: Emit audit events from change and archive commands
- **Files**: `ito-rs/crates/ito-cli/src/commands/change.rs`, `ito-rs/crates/ito-core/src/archive.rs`
- **Dependencies**: 3.1
- **Action**:
  Add audit event emission to:
  - `create change`: emit ChangeCreated { change_id, module_id }
  - `archive`: emit ChangeArchived { change_id, archive_path } and SpecsUpdated { specs_updated: [...] }
  - `create module`: emit ModuleCreated { module_id, name }
  Events emitted AFTER successful mutation.
- **Verify**: `cargo test -p ito-cli -- change && cargo test -p ito-cli -- archive`
- **Done When**: Creating changes, archiving changes, and creating modules all emit audit events
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 3.4: Implement `ito audit` CLI subcommand
- **Files**: `ito-rs/crates/ito-cli/src/commands/audit.rs`, `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: 3.1
- **Action**:
  Add `ito audit` command group with subcommands:
  - `ito audit log [--change <id>] [--last N] [--json]`: display recent events, optionally filtered
  - `ito audit reconcile [change_id] [--fix] [--json]`: run reconciliation, show drifts, optionally fix
  - `ito audit validate [--json]`: validate JSONL integrity (parseable, monotonic timestamps, known ops)
  Register in CLI arg parser.
- **Verify**: `cargo test -p ito-cli -- audit`
- **Done When**: All three subcommands work; `--json` outputs structured JSON; `--fix` emits reconciliation events
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

## Wave 4: Agent Instructions and Template Updates
- **Depends On**: Wave 3

### Task 4.1: Add audit reconciliation to agent instruction templates
- **Files**: `ito-rs/crates/ito-templates/assets/skills/`, `ito-rs/crates/ito-templates/assets/default/project/.ito/AGENTS.md`
- **Dependencies**: None
- **Action**:
  Update agent instruction templates to include:
  - "MUST use `ito tasks start/complete/shelve` CLI commands for state changes" guidance
  - "Run `ito audit reconcile --fix` after any direct file edits to tasks.md" guidance
  - "Run `ito audit validate` at session start to verify log integrity" guidance
  - "Run `ito audit reconcile` before `ito archive` to ensure consistency" guidance
  Keep instructions concise and actionable.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: `ito init` installs instructions that guide LLMs to use CLI and reconcile after direct edits
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 4.2: Add .gitignore exclusion for audit state cache files
- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/.gitignore`
- **Dependencies**: None
- **Action**:
  Ensure the `.ito/.gitignore` template does NOT exclude `.state/audit/events.jsonl` (it should be git-tracked).
  Verify that `.state/audit/` path is not caught by existing `.state/` exclusion rules.
  If `.state/` is broadly gitignored, add explicit `!.state/audit/` un-ignore rule.
  The events.jsonl MUST be committed to git for merge-based collaboration.
- **Verify**: Check `.gitignore` rules against `.state/audit/events.jsonl` path
- **Done When**: `events.jsonl` is tracked by git; other `.state/` contents remain ignored
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

## Checkpoints

### Checkpoint: Review core domain model and event schema
- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 1 tasks
- **Action**: Review AuditEvent schema, AuditOperation variants, materialization logic, and reconciliation diff algorithm before proceeding to I/O layer
- **Done When**: User confirms domain model is correct
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Checkpoint: Verify end-to-end audit trail before agent instruction updates
- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 3 tasks
- **Action**: Manually test: create a change, start/complete tasks, archive, then run `ito audit log` and `ito audit reconcile` to verify the full pipeline works. Also test direct file edit -> reconcile flow.
- **Done When**: User confirms audit trail is complete and reconciliation works
- **Updated At**: 2026-02-06
- **Status**: [ ] pending
