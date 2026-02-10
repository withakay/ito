# Tasks for: 003-04_optimize-test-execution

## Execution Notes

- **Tool**: OpenCode
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 003-04_optimize-test-execution
ito tasks next 003-04_optimize-test-execution
ito tasks start 003-04_optimize-test-execution 1.1
ito tasks complete 003-04_optimize-test-execution 1.1
```

______________________________________________________________________

## Baseline

Measured 2026-02-10 (3 runs, median):

- `cargo test --workspace`: **5.51s**
- `cargo nextest run --workspace`: **3.06s** (after optimizations)

## Wave 1

- **Depends On**: None

### Task 1.1: Record baseline test timing

- **Files**: (none — measurement only)
- **Dependencies**: None
- **Action**: Run `time cargo test --workspace` 3 times from `ito-rs/`, record the median wall-clock time as the baseline. Document the number in a comment at the top of this file.
- **Verify**: `time cargo test --workspace`
- **Done When**: Baseline recorded
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 1.2: Eliminate `thread::sleep` in `ito-core/src/list.rs` test

- **Files**: `ito-rs/crates/ito-core/src/list.rs`, `ito-rs/crates/ito-core/Cargo.toml`
- **Dependencies**: None
- **Action**: Add `filetime` to `ito-core` dev-dependencies. Replace `std::thread::sleep(Duration::from_millis(15))` at line ~606 with `filetime::set_file_mtime()` calls that set explicit distinct timestamps on the fixture files.
- **Verify**: `cargo test -p ito-core -- list_changes_sorts_by_name_and_recent`
- **Done When**: Test passes without any `thread::sleep`, mtime ordering verified
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 1.3: Eliminate `thread::sleep` in `ito-cli/tests/list_regression.rs`

- **Files**: `ito-rs/crates/ito-cli/tests/list_regression.rs`, `ito-rs/crates/ito-cli/Cargo.toml`
- **Dependencies**: None
- **Action**: Add `filetime` to `ito-cli` dev-dependencies. Replace the two `thread::sleep(Duration::from_millis(20))` calls in `make_repo()` with explicit `filetime::set_file_mtime()` calls on the fixture directories/files.
- **Verify**: `cargo test -p ito-cli -- list_`
- **Done When**: Tests pass without any `thread::sleep`, sort ordering verified
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 1.4: Reduce sleep duration in timeout test

- **Files**: `ito-rs/crates/ito-core/src/ralph/validation.rs`
- **Dependencies**: None
- **Action**: Change `"sleep 10"` to `"sleep 0.1"` in the `shell_timeout_is_failure` test at line ~398.
- **Verify**: `cargo test -p ito-core -- shell_timeout_is_failure`
- **Done When**: Test passes with shorter sleep, still detects timeout correctly
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 1.5: Reduce `run_with_timeout` poll interval

- **Files**: `ito-rs/crates/ito-core/src/process.rs`
- **Dependencies**: None
- **Action**: Change `thread::sleep(Duration::from_millis(50))` at line 220 to `thread::sleep(Duration::from_millis(10))`.
- **Verify**: `cargo test -p ito-core -- shell_timeout_is_failure`
- **Done When**: Test passes, timeout still detected correctly, test runs faster
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add nextest configuration

- **Files**: `ito-rs/.config/nextest.toml`
- **Dependencies**: None
- **Action**: Create `.config/nextest.toml` with default profile settings (fail-fast = false, status-level = "pass", slow timeout = 60s).
- **Verify**: `cargo nextest run --workspace` (if nextest installed)
- **Done When**: Nextest config file exists and is respected
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 2.2: Update Makefile to prefer nextest

- **Files**: `Makefile`
- **Dependencies**: Task 2.1
- **Action**: Update `rust-test` and `rust-test-timed` targets to detect `cargo nextest` on PATH and use it when available, falling back to `cargo test`. Keep the `RUSTFLAGS` settings.
- **Verify**: `make test` uses nextest when available
- **Done When**: Makefile updated, both paths work
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 2.3: Add `[profile.test]` optimisation

- **Files**: `ito-rs/Cargo.toml`
- **Dependencies**: None
- **Action**: Add `[profile.test]` with `opt-level = 1` to the workspace Cargo.toml.
- **Verify**: `cargo test --workspace` still passes
- **Done When**: Test profile optimisation active, no build regressions
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add crate-level TIA script

- **Files**: `ito-rs/tools/test-affected.sh`
- **Dependencies**: None
- **Action**: Create a shell script that: (1) runs `git diff --name-only` to find changed files, (2) maps file paths to crate names, (3) expands to transitive dependents using the known crate dependency graph, (4) runs tests for affected crates only.
- **Verify**: `bash ito-rs/tools/test-affected.sh` runs successfully
- **Done When**: Script correctly identifies affected crates and runs their tests
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 3.2: Add `make test-affected` target

- **Files**: `Makefile`
- **Dependencies**: Task 3.1
- **Action**: Add a `test-affected` target that calls the TIA script.
- **Verify**: `make test-affected`
- **Done When**: Target works and only tests affected crates
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 3.3: Verify 25% reduction target

- **Files**: (none — measurement only)
- **Dependencies**: None
- **Action**: Run `time cargo test --workspace` (and `time cargo nextest run --workspace` if available) 3 times from `ito-rs/`, record median. Compare against baseline from Task 1.1. Target: <= 75% of baseline. (Requires Wave 2 complete.)
- **Verify**: Timing comparison
- **Done When**: 25% reduction achieved or documented why not
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 3.4: Run full quality gate

- **Files**: (none)
- **Dependencies**: Task 3.3
- **Action**: Run `make check && make test` to verify all changes pass quality gates.
- **Verify**: `make check && make test`
- **Done When**: Clean build, all tests pass, no clippy warnings
- **Updated At**: 2026-02-10
- **Status**: [x] complete
