# Tasks for: 001-20_init-update-flag

## Execution Notes

- **Tool**: OpenCode
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking

```bash
ito tasks status 001-20_init-update-flag
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add --update flag to CLI args

- **Files**: ito-rs/crates/ito-cli/src/cli.rs
- **Dependencies**: None
- **Action**:
  Add `--update` (`-u`) flag to `InitArgs` struct with description.
- **Verify**: `cargo build -p ito-cli`
- **Done When**: Flag appears in `ito init --help`
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 1.2: Pass --update through CLI handlers

- **Files**: ito-rs/crates/ito-cli/src/app/init.rs
- **Dependencies**: 1.1
- **Action**:
  Update `handle_init` to parse `--update`/`-u` flag. Update `handle_init_clap` to pass it through as `--update` in argv. Pass `update` to `InitOptions::new`.
- **Verify**: `cargo build -p ito-cli`
- **Done When**: Flag reaches `InitOptions`
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 1.3: Add update field to InitOptions

- **Files**: ito-rs/crates/ito-core/src/installers/mod.rs, ito-rs/crates/ito-cli/src/app/update.rs
- **Dependencies**: 1.2
- **Action**:
  Add `update: bool` field to `InitOptions` struct. Update `InitOptions::new` to accept 3 args. Fix `ito update` handler to pass `false` for the new parameter.
- **Verify**: `cargo build -p ito-cli`
- **Done When**: All callers compile
- **Updated At**: 2026-02-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update write_one() for --update behavior

- **Files**: ito-rs/crates/ito-core/src/installers/mod.rs
- **Dependencies**: None
- **Action**:
  In `write_one()`: when `opts.update` is true and a non-marker file exists, return `Ok(())` (skip) instead of erroring. When `opts.update` is true and a marker-managed file exists without markers, skip the error check and let `update_file_with_markers` handle it (prepends managed block).
- **Verify**: `cargo test -p ito-core`
- **Done When**: Non-marker files preserved, marker files get managed block
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 2.2: Update install_agent_templates() for --update behavior

- **Files**: ito-rs/crates/ito-core/src/installers/mod.rs
- **Dependencies**: None
- **Action**:
  In `install_agent_templates()` Init mode: when `opts.update` is true and the agent file exists, update only the model field (like Update mode) instead of skipping entirely.
- **Verify**: `cargo test -p ito-core`
- **Done When**: Agent templates get model field updated without full overwrite
- **Updated At**: 2026-02-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add integration tests

- **Files**: ito-rs/crates/ito-cli/tests/init_more.rs
- **Dependencies**: None
- **Action**:
  Add three integration tests: (1) init --update preserves user files and creates missing ones, (2) init --update on fresh repo creates all files, (3) init --update doesn't error on existing AGENTS.md without markers.
- **Verify**: `cargo test --test init_more`
- **Done When**: All 11 tests pass (8 existing + 3 new)
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 3.2: Update CLI snapshots

- **Files**: ito-rs/crates/ito-cli/tests/snapshots/
- **Dependencies**: 3.1
- **Action**:
  Accept updated snapshots for init help and help-all that now include the `--update` flag.
- **Verify**: `cargo test --test cli_snapshots`
- **Done When**: All snapshot tests pass
- **Updated At**: 2026-02-08
- **Status**: [x] complete
