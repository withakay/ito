<!-- ITO:START -->
# Tasks for: 025-08_coordination-worktree-storage

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 025-08_coordination-worktree-storage
ito tasks next 025-08_coordination-worktree-storage
ito tasks start 025-08_coordination-worktree-storage 1.1
ito tasks complete 025-08_coordination-worktree-storage 1.1
```

______________________________________________________________________

## Wave 1: Config and Path Foundation

- **Depends On**: None

### Task 1.1: Add CoordinationStorage enum and config fields

- **Files**: `ito-rs/crates/ito-config/src/config/types.rs`
- **Dependencies**: None
- **Action**: Add `CoordinationStorage` enum (`Worktree`, `Embedded`) with serde rename to lowercase. Add `storage` field (default: `Worktree`) and optional `worktree_path: Option<String>` to `CoordinationBranchConfig`.
- **Verify**: `cargo test -p ito-config`
- **Done When**: Config deserializes with and without the new fields, defaults correctly, round-trips through JSON
- **Requirements**: ito-config-crate:coordination-storage-enum, ito-config-crate:coordination-branch-storage-fields
- **Updated At**: 2026-04-04
- **Status**: [x] complete

### Task 1.2: Add org/repo resolution from git remote

- **Files**: `ito-rs/crates/ito-core/src/repo_paths.rs`, `ito-rs/crates/ito-core/src/git.rs`
- **Dependencies**: None
- **Action**: Add function to parse `<org>/<repo>` from `origin` remote URL (supports SSH and HTTPS formats). Add fallback to `backend.project.org`/`backend.project.repo` config values.
- **Verify**: `cargo test -p ito-core`
- **Done When**: Correctly parses org/repo from `git@github.com:org/repo.git`, `https://github.com/org/repo.git`, and falls back to config
- **Requirements**: cascading-config:xdg-worktree-path
- **Updated At**: 2026-04-04
- **Status**: [x] complete

### Task 1.3: Add XDG worktree path resolution

- **Files**: `ito-rs/crates/ito-core/src/repo_paths.rs`
- **Dependencies**: Task 1.1, Task 1.2
- **Action**: Add `coordination_worktree_path()` function. Resolution order: explicit `worktree_path` config → `$XDG_DATA_HOME/ito/<org>/<repo>/` → `~/.local/share/ito/<org>/<repo>/`. Use org/repo from Task 1.2.
- **Verify**: `cargo test -p ito-core`
- **Done When**: Path resolution follows precedence order, respects XDG_DATA_HOME env var, handles missing org/repo gracefully
- **Requirements**: cascading-config:xdg-worktree-path, coordination-worktree:worktree-creation
- **Updated At**: 2026-04-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 2: Worktree and Symlink Core

- **Depends On**: Wave 1

### Task 2.1: Add coordination worktree create/remove to git.rs

- **Files**: `ito-rs/crates/ito-core/src/git.rs`
- **Dependencies**: None
- **Action**: Add `create_coordination_worktree(project_root, branch, target_path)` — creates/fetches the coordination branch, creates the worktree at target_path. Add `remove_coordination_worktree(target_path)` for teardown. Handle: branch exists locally, branch exists on remote only, branch doesn't exist anywhere (create orphan).
- **Verify**: `cargo test -p ito-core`
- **Done When**: Worktree is created and checked out to coordination branch in all three scenarios. Remove cleans up worktree.
- **Requirements**: coordination-worktree:worktree-creation, coordination-worktree:teardown
- **Updated At**: 2026-04-04
- **Status**: [x] complete

### Task 2.2: Add symlink wiring functions

- **Files**: `ito-rs/crates/ito-core/src/repo_paths.rs` or new `ito-rs/crates/ito-core/src/coordination.rs`
- **Dependencies**: None
- **Action**: Add functions to: (1) create symlinks from `.ito/{changes,specs,modules,workflows,audit}` to worktree equivalents, (2) move existing content to worktree before symlinking, (3) add symlink paths to `.gitignore`. Use junctions on Windows. Handle idempotency (don't duplicate .gitignore entries).
- **Verify**: `cargo test -p ito-core`
- **Done When**: Symlinks created correctly on macOS/Linux, .gitignore updated, existing content moved without loss
- **Requirements**: coordination-worktree:symlink-wiring
- **Updated At**: 2026-04-04
- **Status**: [x] complete

### Task 2.3: Add auto-commit on write

- **Files**: `ito-rs/crates/ito-core/src/git.rs` or `ito-rs/crates/ito-core/src/coordination.rs`
- **Dependencies**: Task 2.1
- **Action**: Add `auto_commit_coordination(worktree_path, message)` that stages all changes in the worktree and commits with a descriptive message. Integrate into write paths or provide a hook for callers.
- **Verify**: `cargo test -p ito-core`
- **Done When**: Changes are committed to coordination branch after writes. Commit messages include operation type and artifact name.
- **Requirements**: coordination-worktree:auto-commit
- **Updated At**: 2026-04-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 3: Init Integration

- **Depends On**: Wave 2

### Task 3.1: Wire coordination worktree setup into ito init

- **Files**: `ito-rs/crates/ito-cli/src/commands/init.rs`, `ito-rs/crates/ito-core/src/installers/mod.rs`
- **Dependencies**: None
- **Action**: For new projects: resolve worktree path, create coordination worktree, create symlinks, update .gitignore, write storage mode to config. Add `--no-coordination-worktree` flag to skip. Ensure `ito init --upgrade` does NOT touch storage mode.
- **Verify**: `cargo test -p ito-cli --test init_more`
- **Done When**: Fresh `ito init` creates worktree + symlinks by default. `--no-coordination-worktree` skips. `--upgrade` doesn't modify existing storage.
- **Requirements**: cli-init:coordination-worktree-setup, coordination-worktree-migration:no-auto-migrate, change-coordination-branch:provisioning, change-coordination-branch:storage-mode-selection, cli-init:tmux-preference, cli-init:no-tmux-flag
- **Updated At**: 2026-04-04
- **Status**: [x] complete

### Task 3.2: Add health check for missing/broken worktree

- **Files**: `ito-rs/crates/ito-core/src/coordination.rs` or `ito-rs/crates/ito-core/src/validate.rs`
- **Dependencies**: None
- **Action**: Add detection for: (1) storage mode is worktree but worktree directory missing, (2) symlinks exist but targets are broken. Produce actionable error messages following the "What/Why/How" pattern.
- **Verify**: `cargo test -p ito-core`
- **Done When**: Missing worktree produces clear error with `ito init` suggestion. Broken symlink produces error with target path and fix.
- **Requirements**: coordination-worktree:health-check
- **Updated At**: 2026-04-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 4: Agent Instruction and Integration

- **Depends On**: Wave 3

### Task 4.1: Add migration agent instruction

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/instructions/migrate-to-coordination-worktree.md` (or similar)
- **Dependencies**: None
- **Action**: Write the agent instruction template that guides an LLM through migrating an existing project from embedded to worktree storage. Include: prerequisites, step-by-step, validation, rollback.
- **Verify**: `ito agent instruction migrate-to-coordination-worktree` produces output
- **Done When**: Instruction covers full migration flow and warns about in-flight changes
- **Requirements**: coordination-worktree-migration:agent-instruction
- **Updated At**: 2026-04-08
- **Status**: [x] complete

### Task 4.2: Wire auto-commit into repository adapters

- **Files**: `ito-rs/crates/ito-core/src/change_repository.rs`, `ito-rs/crates/ito-core/src/module_repository.rs`, `ito-rs/crates/ito-core/src/task_repository.rs`
- **Dependencies**: None
- **Action**: After write operations in filesystem repository adapters, trigger auto-commit by checking `changes.coordination_branch.storage` config and verifying the resolved artifact path is under the configured coordination worktree root.
- **Verify**: `cargo test -p ito-core`
- **Done When**: Creating a change, updating tasks, or modifying specs triggers a coordination branch commit when in worktree mode
- **Requirements**: coordination-worktree:auto-commit
- **Updated At**: 2026-04-08
- **Status**: [x] complete

### Task 4.3: Verify repository adapters work through symlinks

- **Files**: `ito-rs/crates/ito-core/tests/` (new integration test)
- **Dependencies**: None
- **Action**: Write integration tests that set up a coordination worktree with symlinks and verify that `FsChangeRepository`, `FsSpecRepository`, `FsModuleRepository`, and `FsTaskRepository` can read and write through the symlinks correctly.
- **Verify**: `cargo test -p ito-core --test coordination_worktree`
- **Done When**: All CRUD operations work transparently through symlinks
- **Requirements**: coordination-worktree:symlink-wiring
- **Updated At**: 2026-04-08
- **Status**: [x] complete

______________________________________________________________________

## Wave 5: Cross-Platform and Polish

- **Depends On**: Wave 4

### Task 5.1: Windows junction support

- **Files**: `ito-rs/crates/ito-core/src/coordination.rs`
- **Dependencies**: None
- **Action**: Abstract symlink creation behind `create_dir_link(src, dst)` that uses junctions on Windows and symlinks elsewhere. Add `#[cfg(windows)]` / `#[cfg(unix)]` branches.
- **Verify**: `cargo test -p ito-core` (Unix path), manual verify on Windows if available
- **Done When**: `create_dir_link` compiles on both platforms and uses the appropriate mechanism
- **Requirements**: coordination-worktree:symlink-wiring
- **Updated At**: 2026-04-08
- **Status**: [x] complete

### Task 5.2: Update module scope for 025

- **Files**: `.ito/modules/025_repository-backends/module.md`
- **Dependencies**: None
- **Action**: Add `coordination-worktree`, `coordination-worktree-migration` to the module's scope list and add `025-08_coordination-worktree-storage` to the changes checklist.
- **Verify**: `ito validate module 025`
- **Done When**: Module validation passes with the new scope and change
- **Updated At**: 2026-04-08
- **Status**: [x] complete

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave
- Wave N depends on all prior waves completing
- Task dependencies within a wave are fine; cross-wave deps use the wave dependency
- Checkpoint waves require human approval before proceeding
<!-- ITO:END -->
