# Change: Optimize Test Execution Speed

## Why

The full test suite takes ~5.5s wall-clock time. While not catastrophic, there are clear wins available: tests sleeping unnecessarily for filesystem timestamp ordering, a timeout test spawning `sleep 10` when `sleep 0.1` suffices, a 50ms poll interval in `run_with_timeout` that adds latency to timeout tests, and the process-per-test overhead of `cargo test` vs `cargo-nextest` which runs each test binary in a separate process for true parallelism. The target is a 25% reduction in test execution time (from ~5.5s to ~4.1s).

## What Changes

### Adopt `cargo-nextest` as the primary test runner

- Add `cargo-nextest` as the recommended test runner — it runs test binaries as separate processes, giving true per-binary parallelism and isolating process-global state mutations (PATH, CWD, env vars) that currently require Mutex guards.
- Update `Makefile` targets (`test`, `test-timed`) to use `cargo nextest run` when available, falling back to `cargo test`.
- Add a `.config/nextest.toml` configuration file to the `ito-rs/` workspace.

### Eliminate unnecessary test sleeps

- **`ito-core/src/list.rs` test**: Replace `thread::sleep(15ms)` with explicit `filetime::set_file_mtime()` calls so that timestamp-dependent sort ordering tests don't need real wall-clock delays.
- **`ito-cli/tests/list_regression.rs`**: Same pattern — replace `thread::sleep(20ms)` x2 with explicit mtime manipulation. Eliminates ~120ms of sleeping across 3 tests.
- **`ito-core/src/ralph/validation.rs` test**: Change `"sleep 10"` to `"sleep 0.1"` in the `shell_timeout_is_failure` test. The test only needs a process that outlives a 50ms timeout — `sleep 10` is 100x longer than needed.

### Reduce `run_with_timeout` poll interval for tests

- Reduce the poll interval in `SystemProcessRunner::run_with_timeout` from 50ms to 10ms. This makes timeout tests complete faster without meaningfully affecting production CPU usage (the function is only used for short-lived validation commands).

### Add crate-level Test Impact Analysis (TIA) script

- Add a `tools/test-affected.sh` script that uses `git diff --name-only` + workspace crate directory mapping to determine which crates were affected by changes, then runs `cargo nextest run -p <affected-crate>...` (or `cargo test -p ...` as fallback) plus all transitive dependents.
- Add a `make test-affected` Makefile target for easy invocation.

### Add `[profile.test]` optimisation

- Add `opt-level = 1` for the test profile. This trades slightly longer compilation for faster test execution — particularly beneficial for tests that do I/O-heavy operations (file creation, process spawning). The incremental compilation overhead is minimal because `opt-level = 1` doesn't trigger full optimisation passes.

## Capabilities

### Modified Capabilities

- `qa-testing-area`: Add requirements for test execution performance targets, nextest adoption, and TIA tooling.

## Impact

- **Test execution time**: Target 25% reduction (5.5s → ~4.1s wall-clock).
- **Developer tooling**: `cargo-nextest` becomes a recommended (not required) dependency. Fallback to `cargo test` preserved.
- **CI**: Faster feedback loop. TIA script enables partial test runs on PRs.
- **New workspace dependency**: `filetime` added to `[dev-dependencies]` for `ito-core` and `ito-cli`.
- **No runtime behaviour changes**: All changes affect test infrastructure and build configuration only.
