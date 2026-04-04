# Planning: Centralize Change Proposals onto Coordination Branch Worktree

> **Status:** Pre-proposal planning document
> **Date:** 2026-04-04
> **Recommended schema:** spec-driven
> **Candidate module:** 012_git-worktrees (or new module if scope is too broad)

## Problem Statement

When an agent starts creating a change proposal on `main`, the system may realize mid-creation that it should be on a worktree. This leaves remnants of the change on `main` and creates sync issues. Additionally, multiple agents working in parallel on different branches have no shared view of change proposals, specs, modules, or audit state — each branch has its own copy, leading to conflicts and stale data.

## Desired Outcome

All mutable Ito artifacts live on the coordination branch (`ito/internal/changes`) via a dedicated worktree stored at a central location on the user's system. The project's `.ito/` directory contains symlinks to this central worktree. This means:

1. No change artifacts ever land on `main` or feature branches
2. All agents/branches see the same proposals, specs, modules, state, and audit data
3. The coordination branch is the single source of truth for all evolving Ito content
4. The mechanism is Git-native (worktree + branch) with no backend server required

## Scope

### What moves to the coordination branch worktree

| Directory | Purpose | Moves? |
|-----------|---------|--------|
| `.ito/changes/` | Change proposals + archive | **Yes** |
| `.ito/specs/` | Main truth specifications | **Yes** |
| `.ito/modules/` | Module definitions | **Yes** |
| `.ito/workflows/.state/` | Change allocations, locks | **Yes** |
| `.ito/audit/` | Audit event logs | **Yes** |
| `.ito/config.json` | Project configuration | **No** — stays local |
| `.ito/AGENTS.md` | Agent instructions | **No** — stays local (managed by `ito init`) |
| `.ito/project.md` | Project conventions | **TBD** — could go either way |
| `.ito/architecture.md` | Architecture guide | **TBD** — could go either way |

### What's in scope for this change

- New storage mode: `coordination_worktree` (or similar name)
- Central worktree storage at XDG-compliant path (e.g., `~/.local/share/ito/<org>/<repo>/`)
- Symlink creation from `.ito/{changes,specs,modules,workflows,audit}` to central worktree
- Git ignore entries for the symlinks
- Configuration to select and customize this mode
- `ito init` flow to set up the worktree and symlinks
- Path resolution changes in `repo_paths.rs` to follow symlinks
- Repository adapter changes to write through the symlink transparently

### Explicitly out of scope

- **Sync-to-main** — no mechanism to merge coordination branch content back to main
- **Backend server changes** — this mode operates independently of the backend API
- **Replacing existing modes** — filesystem, sqlite, and backend modes remain available
- **Migration tooling** — converting existing local `.ito/` content to the new mode (follow-up)

## Design Concepts

### Central Worktree Location

```
~/.local/share/ito/
└── <org>/
    └── <repo>/
        ├── .ito/
        │   ├── changes/
        │   ├── specs/
        │   ├── modules/
        │   ├── workflows/
        │   └── audit/
        └── .git  (worktree metadata)
```

Path resolution order:
1. Explicit config: `changes.coordination_branch.worktree_path`
2. XDG: `$XDG_DATA_HOME/ito/<org>/<repo>/` (defaults to `~/.local/share/ito/...`)
3. Fallback: `~/.ito/worktrees/<org>/<repo>/`

The `<org>/<repo>` namespace comes from `backend.project.org` / `backend.project.repo` config, or is derived from the git remote URL.

### Symlink Structure

```
.ito/
├── config.json              (real file, local)
├── AGENTS.md                (real file, local)
├── changes -> ~/.local/share/ito/withakay/ito/.ito/changes
├── specs -> ~/.local/share/ito/withakay/ito/.ito/specs
├── modules -> ~/.local/share/ito/withakay/ito/.ito/modules
├── workflows -> ~/.local/share/ito/withakay/ito/.ito/workflows
└── audit -> ~/.local/share/ito/withakay/ito/.ito/audit
```

Symlinks MUST be added to `.gitignore`.

### Configuration Model

```json
{
  "changes": {
    "coordination_branch": {
      "enabled": true,
      "name": "ito/internal/changes",
      "storage": "worktree",
      "worktree_path": null
    }
  }
}
```

`storage` values:
- `"worktree"` **(new default)** — dedicated worktree at central location with symlinks
- `"embedded"` — current behavior, artifacts live in-tree on whatever branch you're on
- (backend mode overrides both when `backend.enabled = true`)

### Worktree Lifecycle

**Init (`ito init`):**
1. Check if coordination branch exists locally or on remote
2. Create/fetch the coordination branch
3. Create worktree at central path, checked out to coordination branch
4. Create symlinks from `.ito/` to the worktree
5. Add symlink targets to `.gitignore`

**Ongoing operations:**
- All reads/writes go through the symlinks — transparent to `std::fs`
- Repository adapters (`FsChangeRepository`, `FsSpecRepository`, `FsModuleRepository`) shouldn't need changes if symlinks are followed transparently
- Git operations on coordination branch target the worktree

**Teardown:**
- Remove symlinks, remove worktree with `git worktree remove`, optionally remove central directory

### Relationship to Existing Worktree Strategies

The coordination worktree is **independent** of `checkout_subdir` / `checkout_siblings` / `bare_control_siblings`. Those strategies govern feature work isolation. This governs where `.ito/` content lives. They coexist.

## Affected Code Areas

| File | Change |
|------|--------|
| `ito-config/src/config/types.rs` | Add `storage` field to `CoordinationBranchConfig`, add `CoordinationStorage` enum |
| `ito-core/src/repo_paths.rs` | Coordination worktree path resolution, symlink target computation |
| `ito-core/src/git.rs` | Worktree create/remove for coordination branch |
| `ito-cli/src/commands/init.rs` | Worktree setup + symlink creation in init flow |
| `ito-cli/src/app/worktree_wizard.rs` | Possibly extend wizard for coordination worktree setup |
| `ito-core/src/change_repository.rs` | Verify transparent symlink behavior |
| `ito-core/src/spec_repository.rs` | Verify transparent symlink behavior |
| `ito-core/src/module_repository.rs` | Verify transparent symlink behavior |
| `.gitignore` | Add symlink patterns |

## Affected Specs

- `change-coordination-branch` — major additions (worktree storage, symlinks, lifecycle)
- `cli-init` — new init steps for worktree + symlink setup
- `cli-config` — new configuration options
- `global-config` — XDG path resolution for central storage

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Symlinks not portable (Windows) | Use junctions on Windows; detect OS in init |
| Central path doesn't exist on fresh clone | `ito init` creates it; first command detects and prompts |
| Multiple repos with same name | Namespace by `<org>/<repo>`, not just repo name |
| Worktree gets into bad state | `ito cleanup` / `git worktree repair` |
| Existing projects lose data on upgrade | Migration out of scope; document manual steps |

## Open Questions

1. Should `project.md` / `architecture.md` live on the coordination branch or stay local?
2. Should the coordination worktree auto-commit on every write, or batch commits?
3. How should `ito init --upgrade` handle transition from embedded to worktree storage?
4. Should there be a `ito status` command showing coordination worktree health?

## Suggested Module

**Module 012 (git-worktrees)** is the natural fit. Its scope (`cli-init, cli-artifact-workflow, cli-config, global-config`) covers most affected specs. The `change-coordination-branch` spec would need to be added to module scope.

## Next Steps

1. Fix the audit validation blocker (#190 / #191)
2. Choose module (012 or new)
3. `ito create change "centralize-change-proposals" --module <id> --schema spec-driven`
4. Generate proposal.md, design.md, tasks.md, and spec deltas from this planning document
5. Validate with `ito validate <change-id> --strict`
