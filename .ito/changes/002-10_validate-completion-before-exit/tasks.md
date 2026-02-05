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

### Task 1.1: Add validation CLI flags to RalphArgs

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: None
- **Action**:
  Add two new fields to `RalphArgs`:
  - `--validation-command <cmd>` (optional string, default "make check")
  - `--skip-validation` (boolean flag)
  Update the clap derive to include these flags with appropriate help text.
- **Verify**: `cargo build --workspace`
- **Done When**: CLI accepts both flags without errors
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

### Task 1.2: Add validation fields to RalphOptions

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Add to `RalphOptions` struct:
  - `validation_command: Option<String>`
  - `skip_validation: bool`
- **Verify**: `cargo build --workspace`
- **Done When**: Struct compiles with new fields
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

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
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement run_validation function

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: None
- **Action**:
  Create a new function `run_validation(command: &str, timeout: Duration) -> Result<ValidationResult>` that:
  - Executes the command via shell
  - Captures stdout/stderr
  - Enforces 5-minute timeout
  - Returns `ValidationResult { success: bool, output: String, timed_out: bool }`
- **Verify**: Unit test with mock command
- **Done When**: Function handles success, failure, timeout, and missing command cases
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

### Task 2.2: Integrate validation into completion flow

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: Task 2.1
- **Action**:
  Modify the completion check in `run_ralph` (around line 203):
  - If `skip_validation` is true, accept completion immediately (with warning)
  - Otherwise, run validation command
  - If validation passes, exit loop
  - If validation fails, store failure output for context injection and continue
- **Verify**: `cargo test --workspace`
- **Done When**: Loop validates before exiting
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

### Task 2.3: Inject validation failure as context

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`, `ito-rs/crates/ito-core/src/ralph/prompt.rs`
- **Dependencies**: Task 2.2
- **Action**:
  When validation fails:
  - Store the validation output in iteration state
  - Modify `build_ralph_prompt` to include a `## Validation Failure (completion rejected)` section
  - Include the validation command output and explain the loop continues
- **Verify**: `cargo test --workspace`
- **Done When**: Failed validation output appears in next iteration's prompt
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add unit tests for validation logic

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs` or new test file
- **Dependencies**: None
- **Action**:
  Write tests for:
  - Validation success accepts completion
  - Validation failure continues loop
  - Validation timeout treated as failure
  - Missing validation command (graceful degradation)
  - Skip validation flag bypasses check
- **Verify**: `cargo test --workspace -- --test-threads=1`
- **Done When**: All validation scenarios covered
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

### Task 3.2: Add integration test with stub harness

- **Files**: `ito-rs/crates/ito-cli/tests/ralph_smoke.rs` or new test file
- **Dependencies**: Task 3.1
- **Action**:
  Create an integration test that:
  - Uses stub harness to emit completion promise
  - Configures a validation command that fails first, then passes
  - Verifies loop continues after failed validation
  - Verifies loop exits after successful validation
- **Verify**: `cargo test --workspace ralph`
- **Done When**: Integration test passes
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Update CLI help text

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: None
- **Action**:
  Ensure help text for `--validation-command` and `--skip-validation` clearly explains:
  - Default behavior (make check)
  - Purpose (verify completion before exiting)
  - When to use skip-validation
- **Verify**: `cargo run -- ralph --help`
- **Done When**: Help text is clear and complete
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

### Task 4.2: Add public API documentation

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: None
- **Action**:
  Add doc comments to:
  - New `ValidationResult` struct
  - New `run_validation` function
  - Updated `RalphOptions` fields
- **Verify**: `cargo doc --no-deps`
- **Done When**: `make docs` passes without warnings for new code
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

### Task 4.3: Run full validation

- **Files**: All modified files
- **Dependencies**: Task 4.1, Task 4.2
- **Action**:
  Run full project validation:
  - `make check` (lint + format)
  - `make test` (all tests)
  - `ito validate 002-10_validate-completion-before-exit --strict`
- **Verify**: All commands pass
- **Done When**: No errors or warnings
- **Updated At**: 2026-02-05
- **Status**: [ ] pending

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
