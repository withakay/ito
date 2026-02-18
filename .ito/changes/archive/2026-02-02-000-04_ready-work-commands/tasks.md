# Tasks for: 000-04_ready-work-commands

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 000-04_ready-work-commands
ito tasks next 000-04_ready-work-commands
ito tasks start 000-04_ready-work-commands 1.1
ito tasks complete 000-04_ready-work-commands 1.1
```

---

## Wave 1 - CLI Arguments & Domain Support

- **Depends On**: None

### Task 1.1: Add --ready flag to ListArgs

- **Files**: ito-rs/crates/ito-cli/src/cli.rs
- **Dependencies**: None
- **Action**:
  Add `#[arg(long)]` `ready: bool` field to `ListArgs` struct.
  Update the doc comment for the `list` command to mention the new flag.
- **Verify**: `cargo build -p ito-cli`
- **Done When**: Code compiles with new flag
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 1.2: Add Ready subcommand to TasksAction enum

- **Files**: ito-rs/crates/ito-cli/src/cli.rs
- **Dependencies**: None
- **Action**:
  Add `Ready` variant to `TasksAction` enum with optional `change_id: Option<String>` and `json: bool` flag.
  Update doc comments with examples.
- **Verify**: `cargo build -p ito-cli`
- **Done When**: Code compiles with new subcommand
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 1.3: Add is_ready() method to ChangeSummary

- **Files**: ito-rs/crates/ito-domain/src/changes/mod.rs
- **Dependencies**: None
- **Action**:
  Add `pub fn is_ready(&self) -> bool` method that returns true when:
  - `has_proposal && has_specs && has_tasks && self.status() == ChangeStatus::InProgress`
- **Verify**: `cargo test -p ito-domain`
- **Done When**: Method exists and tests pass
- **Updated At**: 2026-02-02
- **Status**: [x] complete

---

## Wave 2 - Handler Implementation

- **Depends On**: Wave 1

### Task 2.1: Implement --ready filter in list handler

- **Files**: ito-rs/crates/ito-cli/src/app/list.rs
- **Dependencies**: Task 1.1, Task 1.3
- **Action**:
  In `handle_list_clap()`, when `args.ready` is true, filter changes using `is_ready()`.
  Ensure JSON output respects the filter.
- **Verify**: `cargo run -- list --ready` in a test project
- **Done When**: Only ready changes are shown when flag is used
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 2.2: Implement tasks ready handler for single change

- **Files**: ito-rs/crates/ito-cli/src/commands/tasks.rs (or appropriate tasks handler)
- **Dependencies**: Task 1.2
- **Action**:
  Add handler for `TasksAction::Ready` when change_id is provided.
  Find the earliest incomplete wave and return pending tasks from that wave.
  Support --json output format.
- **Verify**: `cargo run -- tasks ready 000-01_test-change` in a test project
- **Done When**: Shows pending tasks from earliest incomplete wave
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 2.3: Implement tasks ready handler for all changes

- **Files**: ito-rs/crates/ito-cli/src/commands/tasks.rs (or appropriate tasks handler)
- **Dependencies**: Task 2.2
- **Action**:
  Extend handler for `TasksAction::Ready` when change_id is None.
  Iterate all changes, collect ready tasks, group by change.
  Support --json output format.
- **Verify**: `cargo run -- tasks ready` in a test project
- **Done When**: Shows ready tasks grouped by change
- **Updated At**: 2026-02-02
- **Status**: [x] complete

---

## Wave 3 - Testing & Documentation

- **Depends On**: Wave 2

### Task 3.1: Add integration tests for list --ready

- **Files**: ito-rs/crates/ito-cli/tests/cli_smoke.rs or new test file
- **Dependencies**: Task 2.1
- **Action**:
  Add tests for:
  - `list --ready` with ready changes present
  - `list --ready` with no ready changes
  - `list --ready --json` output format
- **Verify**: `cargo test -p ito-cli`
- **Done When**: Tests pass
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 3.2: Add integration tests for tasks ready

- **Files**: ito-rs/crates/ito-cli/tests/cli_smoke.rs or new test file
- **Dependencies**: Task 2.2, Task 2.3
- **Action**:
  Add tests for:
  - `tasks ready <change>` with pending tasks
  - `tasks ready <change>` with no pending tasks
  - `tasks ready` across all changes
  - `tasks ready --json` output format
- **Verify**: `cargo test -p ito-cli`
- **Done When**: Tests pass
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 3.3: Update help snapshots

- **Files**: ito-rs/crates/ito-cli/tests/snapshots/
- **Dependencies**: Task 3.1, Task 3.2
- **Action**:
  Run `cargo insta test --accept` to update help text snapshots.
  Verify the new --ready flag and tasks ready subcommand appear in help.
- **Verify**: `cargo test -p ito-cli`
- **Done When**: All snapshot tests pass
- **Updated At**: 2026-02-02
- **Status**: [x] complete

---

## Wave 4 - Skill Templates Update

- **Depends On**: Wave 3

### Task 4.1: Update ito-workflow skill template

- **Files**: ito-rs/crates/ito-templates/assets/skills/ito-workflow.md
- **Dependencies**: None
- **Action**:
  Add documentation for `ito list --ready` and `ito tasks ready` commands.
  Include usage examples showing how agents can find actionable work.
- **Verify**: `ito init --force` in test project, check skill content
- **Done When**: Skill mentions ready commands with examples
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 4.2: Update ito-tasks skill template

- **Files**: ito-rs/crates/ito-templates/assets/skills/ito-tasks.md
- **Dependencies**: None
- **Action**:
  Add `tasks ready` to the list of available subcommands.
  Document usage pattern for finding next actionable task.
- **Verify**: `ito init --force` in test project, check skill content
- **Done When**: Skill documents tasks ready subcommand
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 4.3: Update ito-apply-change-proposal skill

- **Files**: ito-rs/crates/ito-templates/assets/skills/ito-apply-change-proposal.md
- **Dependencies**: Task 4.1, Task 4.2
- **Action**:
  Reference `ito tasks ready` as the recommended way to find next task.
  Update any workflow examples to use the new command.
- **Verify**: Review skill content manually
- **Done When**: Skill recommends using tasks ready
- **Updated At**: 2026-02-02
- **Status**: [x] complete

---

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
