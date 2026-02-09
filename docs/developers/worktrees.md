# Git Worktree Guide

This repository uses a **bare-repo-with-worktrees** layout. There is no checked-out tree at the bare repo root — all work happens inside worktrees.

## Layout

```
ito/                              # bare/control repo root
├── .bare/                        # git object store
├── .git                          # gitdir pointer → .bare
├── main/                         # locked worktree for the main branch
└── ito-worktrees/                # feature/change worktrees
    └── <branch-name>/
```

- **`.bare/`** — the actual git object store (equivalent to a normal `.git/` directory).
- **`.git`** — a plain text file containing `gitdir: ./.bare`, which tells git where the object store lives.
- **`main/`** — the primary worktree, always checked out on the `main` branch. This is where you do day-to-day work and where CI runs.
- **`ito-worktrees/`** — contains one worktree per feature branch, managed by Ito or created manually.

## The `main` worktree is locked

The `main` worktree is protected with `git worktree lock` to prevent accidental removal. **Do not unlock, remove, or prune it.**

You can verify the lock:

```bash
git worktree list --porcelain
```

Look for `locked Primary worktree - do not remove` on the `main` entry.

If the lock is ever missing, restore it:

```bash
git worktree lock main --reason "Primary worktree - do not remove"
```

### Why lock matters

Git worktree metadata lives in `.bare/worktrees/<name>/`. If this metadata is deleted (via `git worktree remove`, `git worktree prune`, or manually), the worktree's `.git` file becomes a dangling pointer and git commands inside that directory will fail with:

```
fatal: not a git repository: /path/to/.bare/worktrees/main
```

Locking prevents `git worktree remove` and `git worktree prune` from touching the worktree.

## Creating a feature worktree

Always create feature worktrees under `ito-worktrees/`:

```bash
git worktree add ito-worktrees/<branch-name> -b <branch-name>
```

To create a worktree from an existing remote branch:

```bash
git fetch origin <branch-name>
git worktree add ito-worktrees/<branch-name> <branch-name>
```

## Working in a worktree

Each worktree is a fully independent working directory. You can have different branches checked out simultaneously and run builds, tests, or servers in parallel.

**Run git commands from inside a worktree** (e.g., `main/` or `ito-worktrees/<branch>/`), not from the bare repo root. The bare root has no index or working tree, so most git commands will fail or behave unexpectedly there.

```bash
# Good
cd main && git status

# Good
cd ito-worktrees/my-feature && git log

# Bad — bare root has no working tree
cd ito && git status
```

## Cleaning up after merge

After a feature branch is merged, remove its worktree and branch:

```bash
git worktree remove ito-worktrees/<branch-name> 2>/dev/null || true
git branch -d <branch-name> 2>/dev/null || true
git worktree prune
```

**Never clean up the `main` worktree.** Only remove feature worktrees under `ito-worktrees/`.

## Recovering a broken worktree

If a worktree stops working with `fatal: not a git repository`, the metadata in `.bare/worktrees/<name>/` is likely missing. To fix it:

1. Check what the worktree's `.git` file expects:
   ```bash
   cat <worktree-path>/.git
   # e.g., gitdir: /path/to/.bare/worktrees/main
   ```

2. Recreate the metadata directory and its required files:
   ```bash
   mkdir -p .bare/worktrees/<name>

   # Point back to the worktree's .git file
   echo "/absolute/path/to/<worktree>/.git" > .bare/worktrees/<name>/gitdir

   # Set the branch
   echo "ref: refs/heads/<branch>" > .bare/worktrees/<name>/HEAD

   # Point to the shared object store
   echo "../.." > .bare/worktrees/<name>/commondir
   ```

3. Verify it works:
   ```bash
   cd <worktree-path> && git status
   ```

4. Re-lock if it was a protected worktree:
   ```bash
   git worktree lock <name> --reason "Primary worktree - do not remove"
   ```

## Quick reference

| Task | Command |
|---|---|
| List worktrees | `git worktree list` |
| List with lock info | `git worktree list --porcelain` |
| Create feature worktree | `git worktree add ito-worktrees/<name> -b <name>` |
| Remove feature worktree | `git worktree remove ito-worktrees/<name>` |
| Lock a worktree | `git worktree lock <name> --reason "reason"` |
| Unlock a worktree | `git worktree unlock <name>` |
| Prune stale metadata | `git worktree prune` |
