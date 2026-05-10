# Tasks for: 012-02_configurable-worktree-apply-behavior

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 012-02_configurable-worktree-apply-behavior
ito tasks next 012-02_configurable-worktree-apply-behavior
ito tasks start 012-02_configurable-worktree-apply-behavior 1.1
ito tasks complete 012-02_configurable-worktree-apply-behavior 1.1
ito tasks shelve 012-02_configurable-worktree-apply-behavior 1.1
ito tasks unshelve 012-02_configurable-worktree-apply-behavior 1.1
ito tasks show 012-02_configurable-worktree-apply-behavior
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Extend worktree config schema and defaults

- **Files**: `ito-rs/crates/ito-config/src/config/types.rs`, `ito-rs/crates/ito-config/src/config/defaults.rs`, `ito-rs/crates/ito-config/src/config/mod.rs`
- **Dependencies**: None
- **Action**:
  Add nested `worktrees` configuration keys (`enabled`, `strategy`, `layout.base_dir`, `layout.dir_name`, `apply.enabled`, `apply.integration_mode`, `apply.copy_from_main`, `apply.setup_commands`, `default_branch`) with defaults and backward-compatible loading. Add legacy key aliases (`worktrees.defaultBranch` → `default_branch`, `worktrees.localFiles` → `apply.copy_from_main`) with deprecation warnings.
- **Verify**: `make test`
- **Done When**: Config load/save tests cover new keys, legacy alias migration, and backward compatibility
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 1.2: Add CLI config support for nested worktree keys

- **Files**: `ito-rs/crates/ito-cli/src/commands/config.rs`, `ito-rs/crates/ito-core/src/config.rs`, `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Update `ito config set/get` handling so nested `worktrees.*` paths can be set, retrieved, and validated (including enum validation for integration mode and workflow strategy).
- **Verify**: `make test`
- **Done When**: CLI config tests pass for all supported `worktrees.*` keys
- **Updated At**: 2026-02-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Make apply instructions configuration-aware

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-core/src/workflow/mod.rs`, `ito-rs/crates/ito-templates/src/instructions.rs`, `ito-rs/crates/ito-templates/assets/instructions/agent/apply.md.j2`
- **Dependencies**: None
- **Action**:
  Inject worktree setup instructions only when `worktrees.enabled` and `worktrees.apply.enabled` are true, including workflow strategy path resolution, stable worktree path setup, copy-from-main steps, and setup command rendering.
- **Verify**: `make test`
- **Done When**: Instruction generation tests cover enabled/disabled paths and deterministic output
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 2.2: Add integration guidance selection and ask-user step

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/apply.md.j2`, `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-core/src/workflow/mod.rs`
- **Dependencies**: Task 2.1
- **Action**:
  Add apply-instruction sections for `commit_pr` and `merge_parent` modes and include explicit prompts for the agent/operator to confirm missing workflow strategy and integration path when instructions are executed.
- **Verify**: `make test`
- **Done When**: Generated apply instructions include mode-specific guidance and ask-user text
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 2.3: Add post-merge cleanup guidance

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/apply.md.j2`, `ito-rs/crates/ito-core/src/workflow/mod.rs`
- **Dependencies**: Task 2.2
- **Action**:
  Add deterministic cleanup steps for removing merged worktrees and local branches, with safe ordering and guardrails.
- **Verify**: `make test`
- **Done When**: Apply instructions include cleanup section when worktree apply mode is enabled
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 2.4: Add interactive worktree setup wizard to `ito init`

- **Files**: `ito-rs/crates/ito-cli/src/app/init.rs`, `ito-rs/crates/ito-core/src/installers/mod.rs`, `ito-rs/crates/ito-config/src/config/mod.rs`
- **Dependencies**: None
- **Action**:
  Add an interactive worktree setup wizard to the `ito init` flow. The wizard always runs in interactive mode and asks: (1) whether to enable worktrees, (2) which strategy to use (presenting `checkout_subdir` as recommended default, plus `checkout_siblings` and `bare_control_siblings`), and (3) which integration mode to prefer (`commit_pr` recommended, or `merge_parent`). If the user declines enablement, only `worktrees.enabled=false` is persisted. All answers are auto-persisted to config. The CLI prints the config file path and written keys after persistence. Non-interactive mode (`--no-interactive`) skips the wizard and uses defaults (disabled).
- **Verify**: `make test`
- **Done When**: `ito init` tests cover interactive wizard (enable yes/no), strategy selection, integration mode selection, config persistence, config path display, and non-interactive skip
- **Updated At**: 2026-02-08
- **Status**: [x] complete

### Task 2.5: Add interactive worktree setup wizard to `ito update`

- **Files**: `ito-rs/crates/ito-cli/src/app/update.rs`, `ito-rs/crates/ito-core/src/installers/mod.rs`, `ito-rs/crates/ito-config/src/config/mod.rs`
- **Dependencies**: Task 2.4
- **Action**:
  Add the same interactive worktree setup wizard to the `ito update` flow, but only trigger it when `worktrees.strategy` is not yet set in config (first upgrade scenario). If config already has a strategy set, skip the wizard entirely and preserve existing config. Non-interactive mode skips the wizard. Same persistence and display behavior as init.
- **Verify**: `make test`
- **Done When**: `ito update` tests cover wizard trigger on missing config, skip on existing config, config persistence, and non-interactive skip
- **Updated At**: 2026-02-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: End-to-end verification and docs touch-ups

- **Files**: `.ito/specs/global-config/spec.md`, `.ito/specs/cli-config/spec.md`, `.ito/specs/cli-artifact-workflow/spec.md`, `.ito/specs/cli-init/spec.md`, `.ito/specs/cli-update/spec.md`, `ito-rs/crates/ito-cli/src/cli.rs` (help text)
- **Dependencies**: None
- **Action**:
  Validate generated instruction artifacts against specs, update any user-facing docs/help for the new worktree config keys, and run quality gates.
- **Verify**: `make check`
- **Done When**: Checks pass and instruction output reflects configured worktree behavior
- **Updated At**: 2026-02-08
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
