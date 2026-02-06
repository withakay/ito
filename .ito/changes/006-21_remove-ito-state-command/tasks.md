# Tasks for: 006-21_remove-ito-state-command

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Tracking**: Use `ito tasks status|next|start|complete`

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Remove CLI state command surface

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/app/run.rs`, `ito-rs/crates/ito-cli/src/commands/mod.rs`
- **Dependencies**: None
- **Action**: Remove `Commands::State`/`StateArgs` and command dispatch, and stop exporting the state command module.
- **Verify**: `cargo test -p ito-cli`
- **Done When**: `ito --help` no longer includes `state` and tests compile.
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 1.2: Remove state implementation and telemetry counters

- **Files**: `ito-rs/crates/ito-cli/src/commands/state.rs`, `ito-rs/crates/ito-cli/src/commands/stats.rs`, `ito-rs/crates/ito-domain/src/state.rs`
- **Dependencies**: Task 1.1
- **Action**: Delete state command implementation and remove built-in `ito.state.*` command ID counters from stats tracking.
- **Verify**: `cargo test -p ito-cli`
- **Done When**: Project builds without state command/domain module references.
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update tests and snapshots for command removal

- **Files**: `ito-rs/crates/ito-cli/tests/**/*.rs`, `ito-rs/crates/ito-cli/tests/snapshots/*.snap`
- **Dependencies**: None
- **Action**: Remove/adjust tests and snapshots that assert `ito state` behavior or include `state` in help output.
- **Verify**: `cargo test -p ito-cli`
- **Done When**: CLI test suite passes with no `state` coverage expectations remaining.
- **Updated At**: 2026-02-06
- **Status**: [ ] pending

### Task 2.2: Validate Ito artifacts for removal change

- **Files**: `.ito/changes/006-21_remove-ito-state-command/**`
- **Dependencies**: Task 2.1
- **Action**: Validate proposal/spec/tasks artifacts for strict schema and requirement formatting compliance.
- **Verify**: `ito validate 006-21_remove-ito-state-command --strict`
- **Done When**: Validation passes with no errors.
- **Updated At**: 2026-02-06
- **Status**: [ ] pending
