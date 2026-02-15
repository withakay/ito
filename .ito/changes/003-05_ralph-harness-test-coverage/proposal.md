## Why

The `ralph` and `harness` modules in `ito-core` are critical infrastructure — ralph orchestrates the AI agent loop and harness manages process execution for all supported coding agents. Current line coverage is uneven: `validation.rs` at 62.8%, `runner.rs` at 68.3%, `types.rs` at 69.6%, `streaming_cli.rs` at 69.6%, and `state.rs` at 83.8%. Several files have zero inline unit tests (`prompt.rs`, `runner.rs`, all CLI harness adapters). The project enforces an 80% floor and targets 100%. Closing these gaps reduces regression risk and makes future refactoring safer — particularly for `runner.rs` which is at 963 lines and approaching the 1000-line hard limit.

## What Changes

- Add unit tests for `ralph/prompt.rs` covering preamble construction, context loading, change-ID resolution, and module context loading.
- Add unit tests for `ralph/state.rs` covering load/save round-trips, `is_safe_change_id_segment` edge cases (empty, too-long, backslash), and append-context no-op on whitespace-only input.
- Add unit tests for `ralph/validation.rs` covering `extract_commands_from_markdown`, all JSON pointer paths in `extract_commands_from_json_value`, `normalize_commands_value` variants (array, null, non-string), `truncate_for_context`, and the `run_extra_validation` code path.
- Add unit tests for `ralph/runner.rs` private helpers: `completion_promise_found` edge cases (multiple tags, nested tags, empty token), `infer_module_from_change` (no-hyphen input), `render_validation_result`, and `render_harness_failure`.
- Add unit tests for `harness/types.rs` covering `HarnessName::FromStr` (all variants plus aliases plus errors), `HarnessName::as_str`, `HarnessName::Display`, and `HarnessRunResult::is_retriable` for each retriable and non-retriable code.
- Add unit tests for `harness/stub.rs` covering `from_env_or_default` with an explicit path argument, `streams_output()` returning false, and `name()` returning `Stub`.
- Add unit tests for CLI harness adapters (`opencode.rs`, `claude_code.rs`, `codex.rs`, `github_copilot.rs`) covering `build_args` with `allow_all=false` and verifying `harness_name()` return values.
- Add integration test coverage for `streaming_cli.rs` inactivity timeout and process-kill behaviour using a minimal sleep script.
- Raise line coverage for all ralph and harness files to at least 80%, targeting 90%+.

## Capabilities

### New Capabilities

_(none — this change adds tests only, no new user-facing capabilities)_

### Modified Capabilities

- `qa-testing-area`: This change exercises and validates the existing testing infrastructure spec (no requirement changes, but coverage data will demonstrate compliance with the performance and efficiency requirements).

## Impact

- **Code**: `ito-rs/crates/ito-core/src/ralph/` (inline `#[cfg(test)]` modules), `ito-rs/crates/ito-core/src/harness/` (inline `#[cfg(test)]` modules), `ito-rs/crates/ito-core/tests/` (new or extended integration test files).
- **Dependencies**: No new crate dependencies expected. Tests use existing `ito-test-support`, `tempfile`, and `serde_json`.
- **APIs**: No API changes. No behavioural changes to production code.
- **Risk**: Low. Test-only changes. Some private helpers in `runner.rs` may need `pub(crate)` visibility or extraction to a `#[cfg(test)]` helper to enable direct unit testing.
