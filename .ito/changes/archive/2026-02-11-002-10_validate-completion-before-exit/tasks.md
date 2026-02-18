# Tasks for: 002-10_validate-completion-before-exit

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (waves must complete before next wave)
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use the tasks CLI to drive status updates

```bash
ito tasks status 002-10_validate-completion-before-exit
ito tasks next 002-10_validate-completion-before-exit
ito tasks start 002-10_validate-completion-before-exit 1.1
ito tasks complete 002-10_validate-completion-before-exit 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None
- **Goal**: Add CLI flags and update RalphOptions

### Task 1.1: Add validation CLI flags to RalphArgs

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: None
- **Action**:
  Add two new fields to `RalphArgs`:
  - `--validation-command <cmd>` (optional string)
  - `--skip-validation` (boolean flag)
  Update the clap derive to include these flags with appropriate help text.
- **Verify**: `cargo build --workspace`
- **Done When**: CLI accepts both flags without errors
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 1.2: Add validation fields to RalphOptions

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: None
- **Action**:
  Add to `RalphOptions` struct:
  - `validation_command: Option<String>` (extra validation command)
  - `skip_validation: bool`
- **Verify**: `cargo build --workspace`
- **Done When**: Struct compiles with new fields
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 1.3: Wire CLI flags to RalphOptions

- **Files**: `ito-rs/crates/ito-cli/src/app/ralph.rs`
- **Dependencies**: Task 1.1, Task 1.2
- **Action**:
  Update `ralph_args_to_argv` and the manual flag parsing in `handle_ralph` to:
  - Parse `--validation-command` flag
  - Parse `--skip-validation` flag
  - Pass values to `RalphOptions`
- **Verify**: `cargo build --workspace`
- **Done When**: Flags parsed and passed to core
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1
- **Goal**: Implement Ito-native task validation

### Task 2.1: Create validation module

- **Files**: `ito-rs/crates/ito-core/src/ralph/validation.rs`
- **Dependencies**: None
- **Action**:
  Create a new module `validation.rs` with:
  - `ValidationResult` struct: `{ success: bool, message: String, output: Option<String> }`
  - `ValidationStep` enum: `TaskStatus`, `ProjectCheck`, `ExtraCommand`
  - Export from `mod.rs`
- **Verify**: `cargo build --workspace`
- **Done When**: Module compiles and is exported
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 2.2: Implement check_task_completion function

- **Files**: `ito-rs/crates/ito-core/src/ralph/validation.rs`
- **Dependencies**: Task 2.1
- **Action**:
  Create function `check_task_completion(ito_path: &Path, change_id: &str) -> Result<ValidationResult>`:
  - Use `ito_domain::tasks::TaskRepository` to get task counts
  - Check if all tasks are complete or shelved
  - Return success with summary, or failure with list of incomplete tasks
- **Verify**: `cargo test --workspace`
- **Done When**: Function correctly reports task completion status
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 2.3: Implement run_project_validation function

- **Files**: `ito-rs/crates/ito-core/src/ralph/validation.rs`
- **Dependencies**: Task 2.1
- **Action**:
  Create function `run_project_validation(ito_path: &Path, timeout: Duration) -> Result<ValidationResult>`:
  - Read validation commands from project configuration (ito.json, .ito/config.json, AGENTS.md, CLAUDE.md)
  - Execute configured validation commands
  - Capture stdout/stderr
  - Enforce 5-minute timeout per command
  - Return success if all pass, failure with output otherwise
  - If no validation configured, warn and return success (graceful degradation)
- **Verify**: Unit test with mock commands
- **Done When**: Function reads config and handles success, failure, timeout, and no-config cases
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 2.4: Implement run_extra_validation function

- **Files**: `ito-rs/crates/ito-core/src/ralph/validation.rs`
- **Dependencies**: Task 2.1
- **Action**:
  Create function `run_extra_validation(command: &str, timeout: Duration) -> Result<ValidationResult>`:
  - Execute the command via shell
  - Capture stdout/stderr
  - Enforce 5-minute timeout
  - Return success or failure with output
- **Verify**: Unit test with mock command
- **Done When**: Function handles success, failure, and timeout cases
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2
- **Goal**: Integrate validation into the completion flow

### Task 3.1: Add validation failure state tracking

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: None
- **Action**:
  Add a field to track validation failure output between iterations:
  - `last_validation_failure: Option<String>` (or similar)
  - This will be used to inject context into the next iteration
- **Verify**: `cargo build --workspace`
- **Done When**: State can track validation failure output
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 3.2: Integrate validation into completion check

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: Task 3.1
- **Action**:
  Modify the completion check in `run_ralph` (around line 203):
  - If `skip_validation` is true, accept completion immediately (with warning)
  - Otherwise, run validation in order:
    1. `check_task_completion` (if change_id provided)
    2. `run_project_validation`
    3. `run_extra_validation` (if validation_command provided)
  - If all pass, exit loop
  - If any fails, store failure output and continue to next iteration
- **Verify**: `cargo test --workspace`
- **Done When**: Loop validates before exiting
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 3.3: Update prompt builder for validation failure context

- **Files**: `ito-rs/crates/ito-core/src/ralph/prompt.rs`
- **Dependencies**: Task 3.1
- **Action**:
  Modify `build_ralph_prompt` and `BuildPromptOptions` to:
  - Accept optional `validation_failure: Option<String>`
  - When present, include a section labeled `## Validation Failure (completion rejected)`
  - Include the failure output and explain that the loop continues until validation passes
- **Verify**: `cargo test --workspace`
- **Done When**: Failed validation output appears in next iteration's prompt
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3
- **Goal**: Testing and documentation

### Task 4.1: Add unit tests for task completion validation

- **Files**: `ito-rs/crates/ito-core/src/ralph/validation.rs`
- **Dependencies**: None
- **Action**:
  Write tests for `check_task_completion`:
  - All tasks complete -> success
  - All tasks complete or shelved -> success
  - Some tasks pending -> failure with task list
  - No change-id -> skip (handled at call site)
- **Verify**: `cargo test --workspace -- ralph::validation`
- **Done When**: Task validation scenarios covered
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 4.2: Add unit tests for project validation

- **Files**: `ito-rs/crates/ito-core/src/ralph/validation.rs`
- **Dependencies**: None
- **Action**:
  Write tests for `run_project_validation`:
  - Configured command passes -> success
  - Configured command fails -> failure with output
  - Timeout -> failure with timeout message
  - No validation configured -> graceful skip with warning
- **Verify**: `cargo test --workspace -- ralph::validation`
- **Done When**: Project validation scenarios covered
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 4.3: Add integration test with stub harness

- **Files**: `ito-rs/crates/ito-cli/tests/ralph_smoke.rs` or new test file
- **Dependencies**: None
- **Action**:
  Create an integration test that:
  - Uses stub harness to emit completion promise
  - Sets up a change with incomplete tasks
  - Verifies loop continues after task validation fails
  - Marks tasks complete and verifies loop can exit
- **Verify**: `cargo test --workspace ralph`
- **Done When**: Integration test passes
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 4.4: Update CLI help text and documentation

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-core/src/ralph/validation.rs`
- **Dependencies**: None
- **Action**:
  - Ensure help text for flags clearly explains their purpose
  - Add doc comments to all public functions in validation module
- **Verify**: `cargo doc --no-deps` and `cargo run -- ralph --help`
- **Done When**: Help text and docs are clear and complete
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 4.5: Run full validation

- **Files**: All modified files
- **Dependencies**: Task 4.1, Task 4.2, Task 4.3, Task 4.4
- **Action**:
  Run full project validation:
  - `make check` (lint + format)
  - `make test` (all tests)
  - `ito validate 002-10_validate-completion-before-exit --strict`
- **Verify**: All commands pass
- **Done When**: No errors or warnings
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
