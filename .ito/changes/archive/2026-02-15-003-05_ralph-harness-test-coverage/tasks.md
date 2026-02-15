# Tasks for: 003-05_ralph-harness-test-coverage

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Parallel within waves, sequential across waves
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 003-05
ito tasks next 003-05
ito tasks start 003-05 <task-id>
ito tasks complete 003-05 <task-id>
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add unit tests for `HarnessName` parsing and display

- **Files**: `ito-rs/crates/ito-core/src/harness/types.rs`
- **Dependencies**: None
- **Action**:
  Add `#[cfg(test)]` unit tests for:
  - `FromStr` for all valid variants (`"opencode"`, `"claude"`, `"codex"`, `"copilot"`, `"github-copilot"`, `"stub"`)
  - `FromStr` error case for invalid input
  - `as_str()` for each variant
  - `Display` formatting for each variant
  - `HarnessNameParseError` Display output
- **Verify**: `cargo test -p ito-core -- harness::types`
- **Done When**: All `HarnessName` conversion paths are unit-tested; `types.rs` line coverage >= 80%
- **Updated At**: 2026-02-15
- **Status**: [x] complete

### Task 1.2: Add unit tests for `HarnessRunResult::is_retriable`

- **Files**: `ito-rs/crates/ito-core/src/harness/types.rs`
- **Dependencies**: None
- **Action**:
  Add unit tests covering:
  - Each code in `RETRIABLE_EXIT_CODES` (128, 129, 130, 131, 134, 136, 137, 139, 141, 143) returns `true`
  - Non-retriable codes (0, 1, 2, 127, 132, 144, 255) return `false`
  - `exit_code = -1` (timeout) returns `false`
- **Verify**: `cargo test -p ito-core -- harness::types::tests::is_retriable`
- **Done When**: `is_retriable()` is fully exercised with direct unit tests
- **Updated At**: 2026-02-15
- **Status**: [x] complete

### Task 1.3: Add unit tests for `StubHarness` traits and edge cases

- **Files**: `ito-rs/crates/ito-core/src/harness/stub.rs`
- **Dependencies**: None
- **Action**:
  Add inline `#[cfg(test)]` tests covering:
  - `name()` returns `HarnessName::Stub`
  - `streams_output()` returns `false`
  - `from_env_or_default` with an explicit `Some(path)` argument
  - `run()` sets `timed_out = false`
  - `run()` sets a non-zero `duration`
- **Verify**: `cargo test -p ito-core -- harness::stub`
- **Done When**: `stub.rs` line coverage >= 95%
- **Updated At**: 2026-02-15
- **Status**: [x] complete

### Task 1.4: Add unit tests for CLI harness adapter `build_args`

- **Files**: `ito-rs/crates/ito-core/src/harness/opencode.rs`, `ito-rs/crates/ito-core/src/harness/claude_code.rs`, `ito-rs/crates/ito-core/src/harness/codex.rs`, `ito-rs/crates/ito-core/src/harness/github_copilot.rs`
- **Dependencies**: None
- **Action**:
  Add inline `#[cfg(test)]` tests in each file covering:
  - `harness_name()` returns correct variant
  - `binary()` returns correct binary name
  - `build_args` with `allow_all = true` includes the right flag
  - `build_args` with `allow_all = false` omits the flag
  - Verify model and prompt placement in args
- **Verify**: `cargo test -p ito-core -- harness`
- **Done When**: Each CLI harness adapter file has >= 80% line coverage
- **Updated At**: 2026-02-15
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: None

### Task 2.1: Add unit tests for `ralph/state.rs`

- **Files**: `ito-rs/crates/ito-core/src/ralph/state.rs`
- **Dependencies**: None
- **Action**:
  Add `#[cfg(test)]` unit tests covering:
  - `load_state` / `save_state` round-trip with `tempfile`
  - `is_safe_change_id_segment` edge cases: empty string, 256+ char string, backslash, forward slash, `..`, and valid IDs
  - `append_context` with whitespace-only input (should be no-op)
  - `ralph_state_json_path` and `ralph_context_path` return correct paths
  - `load_context` returns empty string when file does not exist
- **Verify**: `cargo test -p ito-core -- ralph::state`
- **Done When**: `state.rs` line coverage >= 90%
- **Updated At**: 2026-02-15
- **Status**: [x] complete

### Task 2.2: Add unit tests for `ralph/validation.rs` — command extraction

- **Files**: `ito-rs/crates/ito-core/src/ralph/validation.rs`
- **Dependencies**: None
- **Action**:
  Add `#[cfg(test)]` unit tests covering:
  - `extract_commands_from_markdown`: detects `make check`, `make test`; ignores other lines
  - `extract_commands_from_json_value` for all 10 JSON pointer paths (not just the first one)
  - `normalize_commands_value`: string, array of strings, null, non-string types (bool, number, object)
  - `truncate_for_context`: output under 12KB unchanged, output over 12KB truncated with marker
- **Verify**: `cargo test -p ito-core -- ralph::validation`
- **Done When**: `validation.rs` line coverage >= 80%
- **Updated At**: 2026-02-15
- **Status**: [x] complete

### Task 2.3: Add unit tests for `ralph/validation.rs` — run_extra_validation

- **Files**: `ito-rs/crates/ito-core/src/ralph/validation.rs`
- **Dependencies**: Task 2.2
- **Action**:
  Add tests for `run_extra_validation`:
  - Successful command returns `ValidationResult { success: true }`
  - Failing command returns `ValidationResult { success: false }` with output
  - Test the discovery priority order: `ito.json` > `config.json` > `AGENTS.md` > `CLAUDE.md`
- **Verify**: `cargo test -p ito-core -- ralph::validation`
- **Done When**: `run_extra_validation` and `discover_project_validation_commands` priority paths are tested
- **Updated At**: 2026-02-15
- **Status**: [x] complete

### Task 2.4: Add unit tests for `ralph/prompt.rs`

- **Files**: `ito-rs/crates/ito-core/src/ralph/prompt.rs`
- **Dependencies**: None
- **Action**:
  Add `#[cfg(test)]` unit tests covering:
  - `build_prompt_preamble`: contains iteration count, max iterations, completion promise token, context section when provided, validation failure section when provided, absent sections when `None`
  - `load_change_context`: returns proposal content when file exists, returns empty/error when missing
  - `load_module_context`: returns module content when file exists, returns empty when missing
  - `resolve_change_id`: NotFound, Ambiguous, Unique resolution paths
- **Verify**: `cargo test -p ito-core -- ralph::prompt`
- **Done When**: `prompt.rs` line coverage >= 80%
- **Updated At**: 2026-02-15
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add unit tests for `ralph/runner.rs` private helpers

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: None
- **Action**:
  To test private helpers, add a `#[cfg(test)] mod tests` block inside `runner.rs`. Test:
  - `completion_promise_found`: single match, no match, multiple `<promise>` tags (first wins), empty token, promise in stderr only, whitespace around token
  - `infer_module_from_change`: valid input (`"003-05"` -> `"003"`), no-hyphen input (returns full string or error)
  - `render_validation_result`: produces expected markdown for pass and fail cases
  - `render_harness_failure`: produces expected markdown with stderr/stdout included
- **Verify**: `cargo test -p ito-core -- ralph::runner::tests`
- **Done When**: Private helper functions in `runner.rs` have direct unit tests; file coverage improves to >= 75%
- **Updated At**: 2026-02-15
- **Status**: [x] complete

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 1

### Task 4.1: Add integration test for streaming_cli inactivity timeout

- **Files**: `ito-rs/crates/ito-core/tests/harness_opencode.rs` (or new `harness_streaming.rs`)
- **Dependencies**: None
- **Action**:
  Add a test that:
  - Creates a minimal shell script that sleeps indefinitely (respecting the timeout test efficiency spec: sleep <= 10x timeout)
  - Configures a short inactivity timeout (e.g., 1 second)
  - Verifies the harness returns `timed_out = true` and `exit_code = -1`
  - Verifies the process is actually killed
- **Verify**: `cargo test -p ito-core -- harness_streaming`
- **Done When**: Inactivity timeout and process-kill logic in `streaming_cli.rs` is exercised; `streaming_cli.rs` coverage >= 75%
- **Updated At**: 2026-02-15
- **Status**: [x] complete

### Task 4.2: Add integration tests for missing-binary errors on all CLI harnesses

- **Files**: `ito-rs/crates/ito-core/tests/harness_opencode.rs`
- **Dependencies**: None
- **Action**:
  Add tests verifying that each CLI harness (Claude, Codex, Copilot) returns a clear error when the binary is not on PATH. Currently only OpenCode has this test.
- **Verify**: `cargo test -p ito-core -- harness_opencode`
- **Done When**: Missing-binary error path tested for all 4 CLI harnesses
- **Updated At**: 2026-02-15
- **Status**: [x] complete

______________________________________________________________________

## Wave 5

- **Depends On**: Wave 4

### Task 5.1: Verify coverage meets 80% floor for all ralph and harness files

- **Type**: checkpoint
- **Files**: All `ralph/` and `harness/` files
- **Dependencies**: None
- **Action**:
  Run `cargo llvm-cov report --package ito-core` and verify every `ralph/` and `harness/` file meets >= 80% line coverage. If any file is below 80%, add targeted tests to close the gap.
- **Verify**: `cargo llvm-cov report --package ito-core 2>/dev/null | grep -E 'ralph|harness'`
- **Done When**: All ralph and harness files report >= 80% line coverage; `make check` passes
- **Updated At**: 2026-02-15
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
