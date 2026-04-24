<!-- ITO:START -->
# Tasks for: 012-05_worktree-lifecycle-and-init

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 012-05_worktree-lifecycle-and-init
ito tasks next 012-05_worktree-lifecycle-and-init
ito tasks start 012-05_worktree-lifecycle-and-init 1.1
ito tasks complete 012-05_worktree-lifecycle-and-init 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add `WorktreeInitConfig` to config types and schema

- **Files**: `ito-rs/crates/ito-config/src/config/types.rs`
- **Dependencies**: None
- **Action**: Define `WorktreeInitConfig { include: Vec<String> }` with `Default`, `Serialize`, `Deserialize`, `JsonSchema`. Add `init: WorktreeInitConfig` field to `WorktreesConfig` with `#[serde(default)]`. Regenerate or update the JSON schema file.
- **Verify**: `cargo test -p ito-config`, `cargo check --workspace`
- **Done When**: Config round-trips through JSON without errors; `worktrees.init.include` is visible in schema; existing config files without `init` still load cleanly.
- **Requirements**: `config:worktrees-init-config`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 1.2: Implement include-file resolution logic in `ito-core`

- **Files**: `ito-rs/crates/ito-core/src/coordination_worktree.rs` (or a new `worktree_init.rs`)
- **Dependencies**: Task 1.1
- **Action**: Implement `resolve_include_patterns(config: &WorktreesConfig, worktree_root: &Path) -> Vec<PathBuf>` that merges globs from `config.worktrees.init.include` and a `.worktree-include` file at `worktree_root`. Parse `.worktree-include` with `#`-comment and blank-line stripping. Expand globs against the source root. Return matched paths.
- **Verify**: Unit tests covering: config-only, file-only, union, missing file, comment/blank handling, glob expansion.
- **Done When**: All unit tests pass; `cargo clippy` clean.
- **Requirements**: `worktree-init-files:config-include`, `worktree-init-files:file-include`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 1.3: Implement worktree initialization (copy + symlinks)

- **Files**: `ito-rs/crates/ito-core/src/coordination_worktree.rs`
- **Dependencies**: Task 1.2
- **Action**: Implement `init_worktree(source_root: &Path, dest_root: &Path, config: &WorktreesConfig) -> Result<()>` that: (1) copies matched include files (overwrite-safe, relative paths preserved), (2) creates coordination-branch symlinks. Order: copy first, then symlinks. Idempotent — no error if already initialized.
- **Verify**: Integration test using `ito-test-support` mock repos: create worktree, verify files copied, verify symlinks present, re-run and verify no error.
- **Done When**: Tests pass; idempotent re-run succeeds; `cargo clippy` clean.
- **Requirements**: `worktree-init-files:config-include`, `worktree-init-files:file-include`, `worktree-init-files:init-ordering`, `worktree-init-files:idempotent`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement `worktree ensure` core operation

- **Files**: `ito-rs/crates/ito-core/src/coordination_worktree.rs` (or new `worktree_ensure.rs`)
- **Dependencies**: None
- **Action**: Implement `ensure_worktree(change_id: &str, config: &ItoConfig, repo_paths: &RepoPaths) -> Result<PathBuf>` that: (1) derives expected worktree path from strategy+layout, (2) if path exists and is a valid git worktree returns it, (3) if absent creates the worktree (branch from `default_branch`) then calls `init_worktree`. Returns the resolved path.
- **Verify**: Integration tests: path-exists case, path-absent case (creation + init), worktrees-disabled case (returns cwd). Test `BareControlSiblings` and `CheckoutSiblings` strategies.
- **Done When**: All three scenario tests pass; path is absolute; `cargo clippy` clean.
- **Requirements**: `worktree-lifecycle:existence-check`, `worktree-lifecycle:strategy-aware-creation`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 2.2: Add `ito worktree ensure` CLI sub-command

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/` (new `worktree.rs` or existing worktree file)
- **Dependencies**: Task 2.1
- **Action**: Add `ito worktree ensure --change <id>` sub-command that calls `ensure_worktree` and prints the resolved path to stdout (only that line). Informational/progress messages go to stderr. Handle `worktrees.enabled = false` by printing the current working directory.
- **Verify**: `cargo test -p ito-cli`; manual smoke test: `ito worktree ensure --change 012-05_worktree-lifecycle-and-init` returns a path.
- **Done When**: stdout contains only the path; stderr has progress messages when creating; exit 0 in all valid cases.
- **Requirements**: `worktree-lifecycle:path-reporting`, `worktree-lifecycle:existence-check`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Update apply instruction template to include worktree ensure step

- **Files**: `ito-rs/crates/ito-templates/src/instructions.rs` (or relevant template file)
- **Dependencies**: None
- **Action**: When rendering the `apply` instruction artifact and `worktrees.enabled` is `true`, prepend a step: "Run `ito worktree ensure --change <id>` and use the returned path as your working directory for all file operations." When `worktrees.enabled` is `false`, omit the step.
- **Verify**: Template rendering tests in `ito-rs/crates/ito-templates/tests/worktree_template_rendering.rs` covering enabled and disabled cases.
- **Done When**: Enabled case renders the step; disabled case does not; `cargo test -p ito-templates` passes.
- **Requirements**: `worktree-lifecycle:apply-instruction-guidance`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 3.2: End-to-end smoke test

- **Files**: `ito-rs/crates/ito-test-support/tests/` or `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: Task 3.1
- **Action**: Write an integration test that: sets up a mock repo with worktrees enabled + an include list, runs `ito worktree ensure --change <id>` via PTY/process, asserts the path exists and is a valid worktree, asserts include files are present.
- **Verify**: `cargo test --workspace`
- **Done When**: Test passes in CI; `make check` green.
- **Requirements**: `worktree-lifecycle:existence-check`, `worktree-init-files:config-include`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending
<!-- ITO:END -->
