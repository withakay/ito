# Tasks for: 000-08_init-coordination-branch-setup

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential waves
- **Tracking**: Use `ito tasks` commands for status transitions

```bash
ito tasks status 000-08_init-coordination-branch-setup
ito tasks next 000-08_init-coordination-branch-setup
ito tasks start 000-08_init-coordination-branch-setup 1.1
ito tasks complete 000-08_init-coordination-branch-setup 1.1
ito tasks show 000-08_init-coordination-branch-setup
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add init flag plumbing for coordination setup

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/app/init.rs`
- **Dependencies**: None
- **Action**:
  Add a new `--setup-coordination-branch` init option and wire it into init flow parsing.
- **Verify**: `cargo test -p ito-cli init_help_prints_usage`
- **Done When**: `ito init --help` includes the new option and parsing works in both clap and legacy paths
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 1.2: Implement coordination branch provisioning behavior

- **Files**: `ito-rs/crates/ito-core/src/git.rs`, `ito-rs/crates/ito-cli/src/app/init.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Add reusable setup logic that checks whether the configured coordination branch exists on `origin`, creates it when missing, and returns deterministic errors for missing remote/auth failures.
- **Verify**: `cargo test -p ito-core`
- **Done When**: init setup flow can report ready/created and fail with actionable guidance on remote setup errors
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add integration tests for init setup option

- **Files**: `ito-rs/crates/ito-cli/tests/init_more.rs`, `ito-rs/crates/ito-cli/tests/support/mod.rs`
- **Dependencies**: None
- **Action**:
  Add tests covering branch already exists, branch creation from missing remote ref, and failure when `origin` is not configured.
- **Verify**: `cargo test -p ito-cli init_`
- **Done When**: tests cover success and failure paths for `--setup-coordination-branch`
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 2.2: Document init coordination setup option

- **Files**: `docs/config.md`
- **Dependencies**: Task 2.1
- **Action**:
  Document when to use `ito init --setup-coordination-branch`, prerequisites, and expected outcomes.
- **Verify**: `rg "setup-coordination-branch" docs/config.md`
- **Done When**: docs include an explicit section for coordination branch setup at init time
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Wave 3 (Validation)

- **Depends On**: Wave 2

### Task 3.1: Validate change artifacts and implementation

- **Type**: checkpoint
- **Files**: `.ito/changes/000-08_init-coordination-branch-setup/**`, `ito-rs/crates/ito-cli/**`, `ito-rs/crates/ito-core/**`, `docs/config.md`
- **Dependencies**: Task 2.2
- **Action**:
  Run strict change validation and relevant Rust tests/checks for touched crates.
- **Verify**: `ito validate 000-08_init-coordination-branch-setup --strict && cargo test -p ito-cli init_ && cargo test -p ito-core`
- **Done When**: validation passes and targeted tests pass
- **Updated At**: 2026-02-11
- **Status**: [x] complete
