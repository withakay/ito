# Tasks for: 002-12_add-ralph-error-threshold

## Wave 1

- **Depends On**: None
- **Goal**: Add a bounded, configurable harness error retry policy to Ralph and document it

### Task 1.1: Add configurable harness error threshold in core Ralph loop

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`, `ito-rs/crates/ito-core/src/ralph/mod.rs`
- **Dependencies**: None
- **Action**: Add `error_threshold` to `RalphOptions`, default it to 10, count non-zero harness exits, and fail when threshold is reached unless fail-fast mode is enabled.
- **Verify**: `cargo test --manifest-path ito-rs/Cargo.toml -p ito-core --test ralph`
- **Done When**: Ralph exits after threshold non-zero harness failures and keeps fail-fast semantics with `--exit-on-error`
- **Updated At**: 2026-02-07
- **Status**: [x] complete

### Task 1.2: Add CLI flag wiring for threshold control

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/app/ralph.rs`
- **Dependencies**: Task 1.1
- **Action**: Add `--error-threshold <n>` and wire parsing/forwarding to core Ralph options and argument passthrough.
- **Verify**: `make build`
- **Done When**: CLI flag is available and passed through to Ralph loop options
- **Updated At**: 2026-02-07
- **Status**: [x] complete

### Task 1.3: Update tests and snapshots for new behavior

- **Files**: `ito-rs/crates/ito-core/tests/ralph.rs`, `ito-rs/crates/ito-cli/tests/snapshots/*ralph*.snap`, `ito-rs/crates/ito-cli/tests/snapshots/*help*.snap`
- **Dependencies**: Task 1.1, Task 1.2
- **Action**: Add threshold behavior tests and refresh CLI help snapshots for the new flag.
- **Verify**: `make test-coverage`
- **Done When**: Tests and snapshots validate default threshold and custom threshold behavior
- **Updated At**: 2026-02-07
- **Status**: [x] complete
