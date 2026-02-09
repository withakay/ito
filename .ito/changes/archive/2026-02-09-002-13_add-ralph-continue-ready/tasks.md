# Tasks for: 002-13_add-ralph-continue-ready

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 002-13_add-ralph-continue-ready
ito tasks next 002-13_add-ralph-continue-ready
ito tasks start 002-13_add-ralph-continue-ready 1.1
ito tasks complete 002-13_add-ralph-continue-ready 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add core support for continue-ready selection loop

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`
- **Dependencies**: None
- **Action**:
  - Add a `--continue-ready` execution mode in the core runner.
  - Implement repo-wide eligible change selection (eligible = `ChangeWorkStatus::{Ready, InProgress}`, order = lowest change ID).
  - Add drift-aware revalidation before each change run.
- **Verify**: `cargo test --manifest-path ito-rs/Cargo.toml -p ito-core --test ralph`
- **Done When**: Core tests cover selection, drift, and blocked-work behavior.
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 1.2: Wire CLI flag and validation

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/app/ralph.rs`
- **Dependencies**: Task 1.1
- **Action**:
  - Add `--continue-ready` to `ito ralph` CLI args.
  - Enforce conflicts/mutual exclusions consistent with existing `--continue-module` rules.
  - Ensure help output documents the new mode.
- **Verify**: `cargo test --manifest-path ito-rs/Cargo.toml -p ito-cli`
- **Done When**: CLI parses the flag, routes to core mode, and prints useful errors on invalid combinations.
- **Updated At**: 2026-02-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add CLI integration tests + help snapshot updates

- **Files**: `ito-rs/crates/ito-cli/tests/ralph_smoke.rs`, `ito-rs/crates/ito-cli/tests/snapshots/*ralph*.snap`, `ito-rs/crates/ito-core/tests/ralph.rs`
- **Dependencies**: None
- **Action**:
  - Add tests that create multiple ready changes and assert deterministic progression.
  - Add tests for “blocked work remains” failure mode.
  - Update help snapshots to include `--continue-ready`.
- **Verify**: `make test`
- **Done When**: Tests pass and cover the new flag behavior end-to-end.
- **Updated At**: 2026-02-08
- **Status**: [x] complete
