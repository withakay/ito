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

### Task 1.1: Add `WorktreeInitConfig` and `WorktreeSetupConfig` to config types and schema

- **Files**: `ito-rs/crates/ito-config/src/config/types.rs`
- **Dependencies**: None
- **Action**: Define `WorktreeSetupConfig` as a `#[serde(untagged)]` enum accepting either a
  single string or a `Vec<String>`, with a `to_commands() -> Vec<String>` helper. Define
  `WorktreeInitConfig { include: Vec<String>, setup: Option<WorktreeSetupConfig> }` with
  `Default`. Add `init: WorktreeInitConfig` to `WorktreesConfig` with `#[serde(default)]`.
  Regenerate or update the JSON schema file.
- **Verify**: `cargo test -p ito-config`, `cargo check --workspace`
- **Done When**: String and array `setup` values both deserialize correctly; schema reflects
  the union type; existing configs without `init` still load.
- **Requirements**: `config:worktrees-init-config`
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.2: Implement include-file resolution logic in `ito-core`

- **Files**: `ito-rs/crates/ito-core/src/coordination_worktree.rs` (or a new `worktree_init.rs`)
- **Dependencies**: Task 1.1
- **Action**: Implement `resolve_include_patterns(config: &WorktreesConfig, worktree_root: &Path) -> Vec<PathBuf>` that merges globs from `config.worktrees.init.include` and a `.worktree-include` file at `worktree_root`. Parse `.worktree-include` with `#`-comment and blank-line stripping. Expand globs against the source root. Return matched paths.
- **Verify**: Unit tests covering: config-only, file-only, union, missing file, comment/blank handling, glob expansion.
- **Done When**: All unit tests pass; `cargo clippy` clean.
- **Requirements**: `worktree-init-files:config-include`, `worktree-init-files:file-include`
- **Updated At**: 2026-04-24
- **Status**: [x] complete

### Task 1.3: Implement worktree initialization (copy + symlinks + setup)

- **Files**: `ito-rs/crates/ito-core/src/coordination_worktree.rs`
- **Dependencies**: Task 1.2
- **Action**: Implement `init_worktree(source_root: &Path, dest_root: &Path, config: &WorktreesConfig) -> Result<()>` that: (1) copies matched include files (overwrite-safe, relative paths preserved), (2) creates coordination-branch symlinks, (3) runs setup command(s) from `config.worktrees.init.setup` if present, with `dest_root` as the working directory. Order: copy → symlinks → setup. Idempotent on copy/symlinks; setup always re-runs when called (callers control whether to call `init_worktree` vs `run_setup` separately).
- **Verify**: Integration test using `ito-test-support` mock repos: create worktree, verify files copied, verify symlinks present, verify setup command ran; re-run and verify no error.
- **Done When**: Tests pass; idempotent copy/symlink re-run succeeds; setup command exit code propagated; `cargo clippy` clean.
- **Requirements**: `worktree-init-files:config-include`, `worktree-init-files:file-include`, `worktree-init-files:init-ordering`, `worktree-init-files:idempotent`, `worktree-setup:command-execution`
- **Updated At**: 2026-04-24
- **Status**: [x] complete

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
- **Status**: [x] complete

### Task 2.2: Add `ito worktree ensure` and `ito worktree setup` CLI sub-commands

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/` (new `worktree.rs` or existing worktree file)
- **Dependencies**: Task 2.1
- **Action**: Add two sub-commands:
  - `ito worktree ensure --change <id>`: calls `ensure_worktree` (which runs init+setup on
    new worktrees only), prints the resolved path to stdout. Progress goes to stderr.
  - `ito worktree setup --change <id>`: calls `run_worktree_setup` on an existing worktree;
    exits non-zero if worktree missing; no-op with informational stderr if no setup configured.
- **Verify**: `cargo test -p ito-cli`; manual: `ito worktree setup --change <id>` without prior ensure returns error.
- **Done When**: Both commands behave per spec; stdout/stderr separation correct; exit codes correct.
- **Requirements**: `worktree-lifecycle:path-reporting`, `worktree-lifecycle:existence-check`, `worktree-setup:standalone-rerun`, `worktree-setup:ensure-integration`
- **Updated At**: 2026-04-24
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add `worktree-init` instruction artifact

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/` (new `worktree-init.md.jinja`
  or similar), `ito-rs/crates/ito-cli/src/cli/agent.rs` (or instruction loader)
- **Dependencies**: None
- **Action**: Add `ito agent instruction worktree-init --change <id>` that renders a template
  showing: target worktree path, include patterns, and setup commands (if any). Template renders
  a "no additional setup required" note when setup is absent. Renders a "worktrees not enabled"
  note when `worktrees.enabled = false`.
- **Verify**: `cargo test -p ito-templates`; manual: run the command with and without a
  setup command configured.
- **Done When**: Three variants (setup present, no setup, disabled) each render correctly.
- **Requirements**: `worktree-setup:instruction-artifact`
- **Updated At**: 2026-04-24
- **Status**: [>] in-progress

### Task 3.2: Update apply instruction template to include worktree ensure step

- **Files**: `ito-rs/crates/ito-templates/src/instructions.rs` (or relevant template file)
- **Dependencies**: Task 3.1
- **Action**: When rendering the `apply` instruction artifact and `worktrees.enabled` is `true`, prepend a step: "Run `ito worktree ensure --change <id>` and use the returned path as your working directory for all file operations." When `worktrees.enabled` is `false`, omit the step.
- **Verify**: Template rendering tests in `ito-rs/crates/ito-templates/tests/worktree_template_rendering.rs` covering enabled and disabled cases.
- **Done When**: Enabled case renders the step; disabled case does not; `cargo test -p ito-templates` passes.
- **Requirements**: `worktree-lifecycle:apply-instruction-guidance`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 3.3: End-to-end smoke test

- **Files**: `ito-rs/crates/ito-test-support/tests/` or `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: Task 3.2
- **Action**: Write an integration test that: sets up a mock repo with worktrees enabled + an include list + a setup command (a small script), runs `ito worktree ensure --change <id>` via PTY/process, asserts the path exists and is a valid worktree, asserts include files are present, asserts setup ran (e.g. sentinel file created by the script).
- **Verify**: `cargo test --workspace`
- **Done When**: Test passes in CI; `make check` green.
- **Requirements**: `worktree-lifecycle:existence-check`, `worktree-init-files:config-include`, `worktree-setup:ensure-integration`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending
<!-- ITO:END -->
