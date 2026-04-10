<!-- ITO:START -->
## Context

Ito artifacts (changes, specs, modules, workflow state, audit) currently live in `.ito/` on whatever branch is checked out. This means starting a change on `main`, switching to a worktree, and having orphaned artifacts on `main`. Parallel agents on different branches each see their own copy. The coordination branch (`ito/internal/changes`) exists for reservation but isn't used as a storage surface.

This design centralizes mutable Ito artifacts onto the coordination branch via a dedicated worktree at a central system path, with symlinks from the project's `.ito/` directory.

### Current State

- `CoordinationBranchConfig` in `ito-config/src/config/types.rs` has `enabled` (bool) and `name` (String)
- `repo_paths.rs` builds all `.ito/` paths from a single `ito_path` root
- `git.rs` has `fetch_coordination_branch` and `reserve_change_on_coordination_branch`
- Repository adapters (`FsChangeRepository`, etc.) use `std::fs` which follows symlinks transparently
- Three persistence modes: Filesystem, Sqlite, Remote (backend)

### Stakeholders

- Solo developers using Ito locally (primary)
- Teams with multiple agents running in parallel (primary)
- Backend-mode users (unaffected — backend overrides)

## Goals / Non-Goals

**Goals:**

- Single source of truth for all mutable Ito artifacts across branches and worktrees
- No remnants left on `main` when starting change proposals
- Git-native solution (worktree + branch) — no backend server required
- Transparent to existing repository adapters via symlinks
- New default for `ito init` on fresh projects

**Non-Goals:**

- Sync-to-main — no mechanism to merge coordination branch content back to main (separate proposal)
- Automatic migration of existing projects — handled via agent instruction
- Backend server changes — backend mode overrides this entirely
- Multi-machine sync — that's what `git push/pull` on the coordination branch is for

## Decisions

### Decision 1: Symlinks over logical path redirection

**Choice**: Use filesystem symlinks from `.ito/{changes,specs,modules,...}` → worktree.

**Alternatives considered**:
- **Logical path redirection in `repo_paths.rs`**: Change all path builders to point to the worktree. Requires changes to every path consumer. Risk of missed paths.
- **Git sparse checkout on coordination branch**: Complex, fragile, poor ergonomics.

**Rationale**: Symlinks are transparent to `std::fs`, meaning all existing repository adapters work without modification. The change surface is limited to init/setup rather than every path consumer. `.gitignore` handles the symlinks cleanly.

### Decision 2: XDG-compliant central path with org/repo namespace

**Choice**: Store worktrees at `$XDG_DATA_HOME/ito/<org>/<repo>/` (defaulting to `~/.local/share/ito/<org>/<repo>/`).

**Alternatives considered**:
- **Under the project directory** (e.g., `.ito/.worktree/`): Defeats the purpose — still on the current branch.
- **Under `~/.ito/worktrees/`**: Non-standard, doesn't follow XDG.
- **Under the bare repo root**: Only works for bare-repo layouts.

**Rationale**: XDG is the standard for user data on Linux/macOS. Namespace by org/repo prevents collisions. Explicit `worktree_path` override covers edge cases.

### Decision 3: Auto-commit on every write

**Choice**: Commit to the coordination branch after every write operation.

**Alternatives considered**:
- **Batch commits on command completion**: Risk of data loss if process dies mid-operation.
- **Manual commits**: Requires user discipline, defeats automation.

**Rationale**: The coordination branch is internal — history cleanliness is irrelevant. Auto-commit ensures no data loss on process interruption. KISS.

### Decision 4: Worktree storage as additive mode, not replacement

**Choice**: Add `storage: "worktree" | "embedded"` to `CoordinationBranchConfig`. Default to `"worktree"` for new projects. `"embedded"` preserves current behavior.

**Alternatives considered**:
- **Replace filesystem mode entirely**: Breaking change, migration burden.
- **Separate top-level mode**: Conflates with `PersistenceMode` (Filesystem/Sqlite/Remote).

**Rationale**: Additive is safe. Existing projects keep working. The storage field is orthogonal to PersistenceMode — it controls WHERE filesystem artifacts live, not whether they're filesystem-backed.

### Decision 5: Org/repo derived from git remote URL with config override

**Choice**: Parse `origin` remote URL to extract `<org>/<repo>`. Override with `backend.project.org` / `backend.project.repo` if set.

**Rationale**: Most projects have a remote. Config override handles edge cases (forks, renamed repos, private registries).

### Decision 6: Windows uses junctions

**Choice**: Use NTFS junctions instead of symlinks on Windows.

**Rationale**: Symlinks on Windows require elevated privileges or developer mode. Junctions work without elevation for directories.

## Risks / Trade-offs

- **Symlink breakage on fresh clone**: Worktree won't exist on a fresh clone. Mitigation: `ito init` detects and sets up the worktree. Clear error message with fix instructions when symlinks are broken.
- **Cross-platform complexity**: macOS/Linux symlinks vs Windows junctions. Mitigation: Abstract behind a `create_dir_link` function that picks the right mechanism.
- **Git worktree limits**: Some Git operations don't work well with many worktrees. Mitigation: This is just one additional worktree — minimal impact.
- **Stale worktree after repo move/rename**: If the project directory moves, the worktree path in Git metadata becomes invalid. Mitigation: `git worktree repair` handles this.
- **Auto-commit performance**: Every write triggers a commit. Mitigation: Commits are local-only (no push), fast for small files. The coordination branch typically has small text files.
<!-- ITO:END -->
