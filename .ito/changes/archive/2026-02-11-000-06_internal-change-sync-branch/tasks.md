# Tasks for: 000-06_internal-change-sync-branch

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential waves
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use `ito tasks start|complete|shelve|unshelve` for status changes

```bash
ito tasks status 000-06_internal-change-sync-branch
ito tasks next 000-06_internal-change-sync-branch
ito tasks start 000-06_internal-change-sync-branch 1.1
ito tasks complete 000-06_internal-change-sync-branch 1.1
ito tasks show 000-06_internal-change-sync-branch
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add coordination-branch config model and defaults

- **Files**: `ito-rs/crates/ito-config/src/config/mod.rs`, `ito-rs/crates/ito-core/src/config/defaults.rs`, `ito-rs/crates/ito-core/src/config.rs`
- **Dependencies**: None
- **Action**:
  Add `changes.coordination_branch.enabled` and `changes.coordination_branch.name` to config structs, merge behavior, and default values (`true`, `ito/internal/changes`). Ensure config validation rejects invalid branch names with clear errors.
- **Verify**: `cargo test --workspace -p ito-config -p ito-core`
- **Done When**: New keys are loaded from config, defaults apply when absent, invalid values fail with actionable messages
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 1.2: Add configuration command coverage for new keys

- **Files**: `ito-rs/crates/ito-cli/src/commands/config.rs`, `ito-rs/crates/ito-cli/tests/config_more.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Extend config set/get support and tests for `changes.coordination_branch.enabled` and `changes.coordination_branch.name` so users can override defaults without editing files manually.
- **Verify**: `cargo test --workspace -p ito-cli config`
- **Done When**: `ito config set/get` works for both keys and tests cover valid/invalid values
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement coordination branch sync primitives

- **Files**: `ito-rs/crates/ito-core/src/git.rs`, `ito-rs/crates/ito-core/src/change_repository.rs`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: None
- **Action**:
  Add reusable sync helpers for fetch/rebase/push against the configured coordination branch, with deterministic error mapping for branch protection and non-fast-forward conflicts.
- **Verify**: `cargo test --workspace -p ito-core`
- **Done When**: Core exposes stable sync functions with tests for success, conflict, and protected-branch failure paths
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 2.2: Wire sync into `ito create change` reservation flow

- **Files**: `ito-rs/crates/ito-cli/src/commands/create.rs`, `ito-rs/crates/ito-core/src/create/mod.rs`, `ito-rs/crates/ito-cli/tests/create_more.rs`
- **Dependencies**: Task 2.1
- **Action**:
  Update create-change flow to pre-sync coordination branch and push proposal metadata immediately after creation when coordination mode is enabled.
- **Verify**: `cargo test --workspace -p ito-cli create`
- **Done When**: Create flow performs pre-sync and reservation push in enabled mode; disabled mode preserves existing behavior
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Sync before apply instruction generation

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-cli/tests/instructions_more.rs`
- **Dependencies**: None
- **Action**:
  Ensure `ito agent instruction apply --change <id>` performs coordination-branch sync before reading change artifacts when coordination mode is enabled.
- **Verify**: `cargo test --workspace -p ito-cli instructions`
- **Done When**: Apply instructions reflect synced change state and tests confirm enabled/disabled behavior
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 3.2: Sync before task start mutation

- **Files**: `ito-rs/crates/ito-cli/src/commands/tasks.rs`, `ito-rs/crates/ito-cli/tests/tasks_more.rs`
- **Dependencies**: Task 3.1
- **Action**:
  Run coordination-branch sync prior to `ito tasks start <change-id> <task-id>` status mutation. Ensure failure messaging is actionable and does not corrupt task files.
- **Verify**: `cargo test --workspace -p ito-cli tasks`
- **Done When**: Task-start path syncs first in enabled mode and preserves current behavior when disabled
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Wave 4 (Review)

- **Depends On**: Wave 3

### Task 4.1: End-to-end verification and docs alignment

- **Type**: checkpoint (requires human approval before archive)
- **Files**: `.ito/changes/000-06_internal-change-sync-branch/**`, `ito-rs/crates/ito-cli/tests/`, `ito-rs/crates/ito-core/tests/`
- **Dependencies**: Task 3.2
- **Action**:
  Run targeted and workspace verification, validate the change with strict mode, and review user-facing output text for clarity around conflict remediation.
- **Verify**: `ito validate 000-06_internal-change-sync-branch --strict && cargo test --workspace`
- **Done When**: Validation passes, tests pass, and reviewer confirms behavior matches proposal/spec
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
