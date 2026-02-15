# Tasks: 002-16_ralph-worktree-awareness

## Wave 1: Worktree resolution in ito-core

- [x] 1.1: Extract or expose a `find_worktree_for_branch(branch: &str) -> Option<PathBuf>` function in `ito-core` that parses `git worktree list --porcelain` and returns the worktree path whose branch matches the given name. Reuse or adapt logic from `audit/worktree.rs::discover_worktrees`.
- [x] 1.2: Add unit tests for `find_worktree_for_branch` covering: matching branch found, no match, bare worktree excluded, multiple worktrees.
- [x] 1.3: Add a `resolve_effective_cwd` function in `ralph/runner.rs` that takes `ito_path`, the resolved `change_id`, and the worktree config, and returns the effective working directory. When worktrees are enabled and a matching worktree exists, return the worktree path; otherwise return `std::env::current_dir()`.
- [x] 1.4: Add unit tests for `resolve_effective_cwd` covering: worktree found, no worktree found (fallback), worktrees not enabled (fallback), no change targeted (fallback).

## Wave 2: Thread resolved cwd through Ralph pipeline

- [x] 2.1: Call `resolve_effective_cwd` early in `run_ralph` and store the result. Log the resolved path when `--verbose`.
- [x] 2.2: Pass the resolved cwd to `HarnessRunConfig.cwd` instead of `std::env::current_dir()`.
- [x] 2.3: Pass the resolved cwd to `commit_iteration` (add a `cwd: &Path` parameter) and set `current_dir` on the `git add` and `git commit` `ProcessRequest`s.
- [x] 2.4: Pass the resolved cwd to `count_git_changes` (add a `cwd: &Path` parameter) and set `current_dir` on the `git status` `ProcessRequest`.
- [x] 2.5: Pass the resolved cwd to `run_project_validation` — derive `ito_path` from the worktree's `.ito` directory when a worktree is active.
- [x] 2.6: Update `run_ralph` to use the worktree's `ito_path` for state file writes when a worktree is resolved.

## Wave 3: CLI layer and config plumbing

- [x] 3.1: Load `WorktreesConfig` from the cascading config in the CLI runtime and pass it (or a relevant subset) to `run_ralph` via `RalphOptions` or a new parameter.
- [x] 3.2: Update `RalphOptions` (or add a separate struct) to carry the worktree config needed for resolution (enabled flag, strategy, layout dir name).

## Wave 4: Integration tests and verification

- [x] 4.1: Add an integration test in `ralph_smoke.rs` that sets up a temporary git repo with a worktree for a change, runs `ito ralph --change <id> --harness stub`, and verifies the harness ran in the worktree directory (e.g. by checking state files are written under the worktree's `.ito`).
- [x] 4.2: Add an integration test verifying fallback: no worktree exists, Ralph runs normally in the process cwd.
- [x] 4.3: Run `make check` — all checks pass.
- [x] 4.4: Run `make test` — all tests pass.
