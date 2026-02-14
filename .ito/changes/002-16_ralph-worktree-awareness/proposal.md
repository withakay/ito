<!-- ITO:START -->
## Why

Ralph's outer loop runs the harness, commits, and validates entirely within the process's inherited working directory (`std::env::current_dir()`). When a project uses git worktrees — e.g. the `bare_control_siblings` strategy where each change gets its own worktree — Ralph has no knowledge of the worktree layout. It spawns the harness, runs `git add -A && git commit`, and executes validation commands all in the directory where `ito ralph` was invoked. If that directory is the bare repo root (or a different worktree), the pre-commit hook checks the wrong code, `git add` stages the wrong files, and the harness edits the wrong tree. This makes Ralph unusable in worktree-based workflows unless the user manually `cd`s into the correct worktree first — defeating the purpose of `--change`-driven automation.

## What Changes

- **Resolve the effective working directory for a change**: When `--change` targets a change that has an existing worktree (branch name matches the change ID), Ralph resolves the worktree path and uses it as the working directory for the harness, git commands, and validation.
- **Pass resolved cwd through the pipeline**: `HarnessRunConfig.cwd`, `commit_iteration`, and `run_project_validation` all use the resolved worktree path instead of the inherited process cwd.
- **Detect worktrees via git**: Use `git worktree list --porcelain` (leveraging the existing `discover_worktrees` function in `audit/worktree.rs`) to find a worktree whose branch matches the change ID.
- **Graceful fallback**: When no matching worktree exists, Ralph falls back to the current behaviour (process cwd). This preserves compatibility for non-worktree workflows.
- **No automatic worktree creation**: Ralph detects existing worktrees but does not create them. Creation remains the user's or instruction template's responsibility.

## Capabilities

### New Capabilities

- `ralph-worktree-awareness`: Ralph's ability to detect and use an existing git worktree for a targeted change, ensuring the harness, git operations, and validation run in the correct working directory.

### Modified Capabilities

- `rust-ralph`: The core Ralph runner gains worktree resolution logic that changes where the harness and git commands execute when a matching worktree exists.
- `cli-ralph`: The CLI layer passes worktree configuration context to the core runner to enable worktree resolution.

## Impact

- **`ito-core/src/ralph/runner.rs`**: `run_ralph` gains a worktree resolution step early in execution. `commit_iteration` and the harness `cwd` use the resolved path. `count_git_changes` also uses the resolved path.
- **`ito-core/src/audit/worktree.rs`**: The existing `discover_worktrees` function may be reused or a lighter-weight variant extracted for Ralph's needs.
- **`ito-core/src/ralph/mod.rs`**: `RalphOptions` or a new internal struct carries the resolved worktree path.
- **`ito-cli/src/commands/ralph.rs`**: May need to pass worktree config context from the runtime.
- **`ito-config`**: Worktree configuration (`WorktreesConfig`) is already modelled; Ralph needs read access to `layout.dir_name` and `strategy` to compute expected worktree paths.
- **Existing tests**: New tests for worktree detection and cwd override. Existing tests unaffected (no worktree present = fallback to current behaviour).
- **No user-facing breaking changes**: Default behaviour is unchanged. Worktree-aware behaviour activates only when a matching worktree is detected.
<!-- ITO:END -->
