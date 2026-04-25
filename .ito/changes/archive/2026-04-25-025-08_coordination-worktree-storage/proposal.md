<!-- ITO:START -->

**GitHub Issue**: withakay/ito#192 — PR for this change MUST include `Closes #192` to auto-close the issue on merge.

## Why

Change proposals, specs, modules, audit logs, and workflow state currently live inside the project's `.ito/` directory on whatever branch you happen to be on. This causes three problems: (1) starting a change proposal on `main` leaves remnants when the system switches to a worktree, (2) multiple agents working on different branches have no shared view of evolving artifacts, and (3) there is no single source of truth for Ito content across parallel workstreams. Centralizing these artifacts on the coordination branch via a dedicated worktree — stored at a central system location with symlinks from the project — eliminates all three issues without requiring a backend server.

## What Changes

- Add a new `storage` field to `CoordinationBranchConfig` with values `"worktree"` (new default for `ito init`) and `"embedded"` (current behavior)
- Create a dedicated worktree for the coordination branch at an XDG-compliant central path (e.g., `~/.local/share/ito/<org>/<repo>/`)
- Replace `.ito/{changes,specs,modules,workflows,audit}` with symlinks pointing to the central worktree
- Add symlink targets to `.gitignore` during init
- Auto-commit to the coordination branch worktree on every write operation
- Derive `<org>/<repo>` namespace from backend project config or git remote URL
- `ito init` for new projects sets up coordination worktree by default
- `ito init --upgrade` does NOT migrate existing projects (migration handled via agent instruction)
- Add `ito agent instruction migrate-to-coordination-worktree` for guided migration of existing repos

## Capabilities

### New Capabilities

- `coordination-worktree`: Dedicated worktree lifecycle management for the coordination branch — creation, symlink wiring, auto-commit on write, teardown, and health checks
- `coordination-worktree-migration`: Agent instruction prompt for guided migration of existing repos from embedded to worktree storage

### Modified Capabilities

- `change-coordination-branch`: Add worktree storage mode, symlink management, and auto-commit semantics to the existing coordination branch provisioning
- `cli-init`: Add coordination worktree setup and symlink creation to the init flow for new projects
- `ito-config-crate`: Add `CoordinationStorage` enum and `storage`/`worktree_path` fields to coordination branch configuration
- `cascading-config`: Add XDG data path resolution for central worktree location

## Impact

- **Config**: `CoordinationBranchConfig` in `ito-config/src/config/types.rs` gains `storage` and `worktree_path` fields
- **Path resolution**: `ito-core/src/repo_paths.rs` gains coordination worktree path computation and symlink target resolution
- **Git operations**: `ito-core/src/git.rs` gains worktree create/remove for coordination branch, plus auto-commit on write
- **Init flow**: `ito-cli/src/commands/init.rs` orchestrates worktree setup + symlink creation
- **Repository adapters**: `FsChangeRepository`, `FsSpecRepository`, `FsModuleRepository`, `FsTaskRepository` should work transparently through symlinks but need verification
- **Cross-platform**: Windows requires junctions instead of symlinks
- **.gitignore**: Symlink targets must be ignored
- **Agent instructions**: New migration instruction template in embedded assets
<!-- ITO:END -->
