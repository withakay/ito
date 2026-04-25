<!-- ITO:START -->
# Tasks for: 012-06_enforce-worktree-change-guidance

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 012-06_enforce-worktree-change-guidance
ito tasks next 012-06_enforce-worktree-change-guidance
ito tasks start 012-06_enforce-worktree-change-guidance 1.1
ito tasks complete 012-06_enforce-worktree-change-guidance 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Locate worktree guidance injection points

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/*.md.j2`, `ito-rs/crates/ito-templates/assets/skills/**/SKILL.md`, `ito-rs/crates/ito-templates/src/instructions_tests.rs`
- **Dependencies**: None
- **Action**: Identify every generated instruction or installed skill that tells agents how to create, choose, or work inside worktrees.
- **Verify**: `rg -n "worktree|worktrees|git worktree" ito-rs/crates/ito-templates/assets ito-rs/crates/ito-templates/src/instructions_tests.rs`
- **Done When**: All guidance injection points affected by this proposal are listed before edits begin.
- **Requirements**: cli-artifact-workflow:fresh-change-worktrees
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 1.2: Update generated worktree guidance

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/*.md.j2`, `ito-rs/crates/ito-templates/assets/skills/**/SKILL.md`
- **Dependencies**: Task 1.1
- **Action**: Add concise rules that worktree-enabled changes use fresh per-change worktrees, keep main/control clean, use the full change ID as the branch/worktree stem, avoid one worktree for two changes, and prefix any same-change extra worktrees with the full change ID.
- **Verify**: `ito agent instruction apply --change 012-06_enforce-worktree-change-guidance`
- **Done When**: Rendered worktree-enabled instructions contain the new invariants without changing disabled-worktree behavior.
- **Requirements**: cli-artifact-workflow:fresh-change-worktrees
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update rendering tests

- **Files**: `ito-rs/crates/ito-templates/src/instructions_tests.rs`
- **Dependencies**: None
- **Action**: Add or update tests that assert the full change ID is used for branch/worktree names and that guidance prohibits main/control checkout work when worktrees are enabled.
- **Verify**: `cargo test -p ito-templates instructions_tests`
- **Done When**: Template rendering tests cover the new worktree guidance rules.
- **Requirements**: cli-artifact-workflow:fresh-change-worktrees
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.2: Validate proposal and full checks

- **Files**: `.ito/changes/012-06_enforce-worktree-change-guidance/**`
- **Dependencies**: Task 2.1
- **Action**: Validate the Ito proposal and run the project check target after implementation.
- **Verify**: `ito validate 012-06_enforce-worktree-change-guidance --strict` and `make check`
- **Done When**: The proposal validates and project checks pass or have documented follow-up blockers.
- **Requirements**: cli-artifact-workflow:fresh-change-worktrees
- **Updated At**: 2026-04-25
- **Status**: [ ] pending
<!-- ITO:END -->
