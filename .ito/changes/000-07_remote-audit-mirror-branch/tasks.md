# Tasks for: 000-07_remote-audit-mirror-branch

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential waves
- **Tracking**: Use `ito tasks` commands for status transitions

```bash
ito tasks status 000-07_remote-audit-mirror-branch
ito tasks next 000-07_remote-audit-mirror-branch
ito tasks start 000-07_remote-audit-mirror-branch 1.1
ito tasks complete 000-07_remote-audit-mirror-branch 1.1
ito tasks show 000-07_remote-audit-mirror-branch
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add audit mirror config keys and defaults

- **Files**: `ito-rs/crates/ito-config/src/config/mod.rs`, `ito-rs/crates/ito-core/src/config/defaults.rs`, `ito-rs/crates/ito-core/src/config.rs`
- **Dependencies**: None
- **Action**:
  Add support for `audit.mirror.enabled` and `audit.mirror.branch`, including default values and validation for branch names.
- **Verify**: `cargo test --workspace -p ito-config -p ito-core`
- **Done When**: Config keys parse, merge, validate, and default correctly
- **Updated At**: 2026-02-11
- **Status**: [ ] pending

### Task 1.2: Extend config command support for audit mirror keys

- **Files**: `ito-rs/crates/ito-cli/src/commands/config.rs`, `ito-rs/crates/ito-cli/tests/config_more.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Support `ito config set/get` for audit mirror keys and add tests for valid and invalid branch inputs.
- **Verify**: `cargo test --workspace -p ito-cli config`
- **Done When**: CLI set/get works and tests cover both success and validation failures
- **Updated At**: 2026-02-11
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement best-effort audit mirror sync

- **Files**: `ito-rs/crates/ito-core/src/audit/`, `ito-rs/crates/ito-core/src/git.rs`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: None
- **Action**:
  Implement mirror write/sync path that appends locally first, then attempts branch sync/push to configured audit mirror branch with warning-only behavior on failure.
- **Verify**: `cargo test --workspace -p ito-core`
- **Done When**: Mirror path works in success case and degrades safely on conflicts/offline failures
- **Updated At**: 2026-02-11
- **Status**: [ ] pending

### Task 2.2: Add integration coverage for mirror behavior

- **Files**: `ito-rs/crates/ito-cli/tests/`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: Task 2.1
- **Action**:
  Add tests for enabled/disabled mirror behavior, branch override, and non-blocking failure outcomes.
- **Verify**: `cargo test --workspace -p ito-cli -p ito-core`
- **Done When**: Integration tests verify best-effort semantics and branch separation from change coordination
- **Updated At**: 2026-02-11
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3 (Review)

- **Depends On**: Wave 2

### Task 3.1: Validate proposal artifacts and readiness

- **Type**: checkpoint (requires human approval before implementation)
- **Files**: `.ito/changes/000-07_remote-audit-mirror-branch/**`
- **Dependencies**: Task 2.2
- **Action**:
  Run strict change validation and ensure wording clearly captures best-effort behavior and separation from coordination branch.
- **Verify**: `ito validate 000-07_remote-audit-mirror-branch --strict`
- **Done When**: Validation passes and reviewer approves scope clarity
- **Updated At**: 2026-02-11
- **Status**: [ ] pending
