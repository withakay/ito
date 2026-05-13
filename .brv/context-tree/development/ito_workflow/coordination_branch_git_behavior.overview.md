## Key points
- Coordination branch bootstrapping uses an **empty-tree root commit** when the remote branch does not already exist.
- If `git mktree` or `git commit-tree` returns **empty stdout**, the output is rejected so an empty hash is never pushed.
- Initialization must create a **root commit without `-p`**, and the pushed refspec must be of the form `<commit_hash>:refs/heads/<coordination-branch>`.
- Branch setup distinguishes between **Ready** (remote branch already exists) and **Created** (branch had to be initialized).
- Reservation flows use a **temporary detached worktree** to reserve changes safely, preventing leakage of the caller’s HEAD/history.
- Git push failures are explicitly classified, including **non-fast-forward, protected branch, remote rejected, remote missing, remote not configured**, and generic command failures.
- Branch name validation and worktree cleanup are part of the coordination git helper’s safety rules.

## Structure / sections summary
- **Reason**: States the goal of documenting coordination branch bootstrapping and reservation rules from `git.rs`.
- **Raw Concept**: Summarizes the implementation changes:
  - empty-tree bootstrap for missing remote branches
  - rejection of empty command output
  - coordination git error classification
  - reservation worktree flow
- **Flow**: Describes the operational sequence:
  - detect missing remote branch
  - create empty tree
  - create root commit
  - push init refspec
  - otherwise fetch and reserve via detached temporary worktree
- **Narrative**
  - **Structure**: The module exposes fetch, push, reservation, and branch-setup helpers, plus core wrappers and cleanup for temporary worktrees.
  - **Dependencies**: Relies on git worktree checks, remote fetch/push commands, branch-name validation, and temporary worktree cleanup.
  - **Highlights**: Clarifies the Ready/Created result states and the push-failure classification scheme.
  - **Rules**: Lists hard constraints for initialization and push behavior.
  - **Examples**: Gives a concrete bootstrap sequence from fetch through push.
- **Facts**: Enumerates notable named conventions and project facts:
  - bootstrap must not use the caller’s HEAD
  - missing remote branch initialization via empty tree + root commit
  - empty stdout rejection
  - reservation safety via ensure/fetch/checkout
  - explicit git error kind classification

## Notable entities, patterns, or decisions
- **Empty-tree-based bootstrap**: A deliberate design choice to initialize coordination branches without relying on existing history.
- **Detached temporary worktree**: Used to isolate reservation operations and protect caller state.
- **Error taxonomy**: Remote missing / remote not configured / non-fast-forward / protected branch / remote rejected / command failures.
- **Gitignore marker pattern**: `^# Ito coordination worktree symlinks$` for coordination symlink handling.
- **Safety convention**: Coordination branch initialization must not use the caller’s HEAD, preserving implementation-history isolation.