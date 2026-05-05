## Key points
- Coordination worktree init/sync now **repairs missing `.ito/` links**, and can recreate the target directory when a **correct symlink points to a missing target**.
- The system **rejects ambiguous or unsafe state**: wrong symlink targets and **non-empty duplicate `.ito/` directories** fail explicitly instead of being merged.
- During sync, **symlinks are wired before health checks**, ensuring the worktree layout is established before validation proceeds.
- Missing origin/remote configuration is treated as **non-fatal `RateLimited`** after local repair, rather than blocking the operation outright.
- The behavior is centered on clear remediation: messages are designed to tell users **what happened, why it matters, and how to fix it**.
- Gitignore handling includes a dedicated marker block: `^# Ito coordination worktree symlinks$`.

## Structure / sections summary
- **Reason**: states the goal of documenting coordination worktree symlink repair and sync behavior from `coordination.rs` and `coordination_worktree.rs`.
- **Raw Concept**: lists the core task, changes, files, overall flow, timestamp, and a gitignore pattern marker.
- **Narrative**
  - **Structure**: splits responsibilities between `coordination.rs` and `coordination_worktree.rs`.
  - **Dependencies**: mentions `lexical_normalize`, git worktree commands, `CoordinationStorage`, and shared git metadata.
  - **Highlights**: summarizes repair behavior, explicit failure cases, and sync fallback behavior.
  - **Rules**: enumerates decision rules for symlink states and empty vs non-empty directories.
  - **Examples**: provides concrete scenarios for wrong symlink targets and provisioning in worktree mode.

## Notable entities, patterns, or decisions
- **Files involved**:
  - `ito-rs/crates/ito-core/src/coordination.rs`
  - `ito-rs/crates/ito-core/src/coordination_worktree.rs`
  - `ito-rs/crates/ito-core/src/coordination_tests.rs`
  - `ito-rs/crates/ito-core/src/coordination_worktree_tests.rs`
- **Coordination paths** handled by symlink logic:
  - `.ito/changes`
  - `.ito/specs`
  - `.ito/modules`
  - `.ito/workflows`
  - `.ito/audit`
- **Decision pattern**:
  - Missing symlink → create it.
  - Correct symlink with missing target → repair/recreate target.
  - Wrong symlink target → fail with explicit guidance.
  - Empty real directory at expected symlink location → remove and replace with symlink.
  - Non-empty real directory → fail to avoid silently merging duplicates.
- **Operational flow**:
  - provision/init → resolve worktree path → create/reuse worktree → wire symlinks → update `.gitignore` → health check → fetch → fast-forward → rate-limit check → auto-commit → push → persist sync state.
- **Design choice**:
  - Error and guidance messages follow a **What / Why / How** structure for immediate actionability.