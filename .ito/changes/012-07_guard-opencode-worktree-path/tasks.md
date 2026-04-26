<!-- ITO:START -->
# Tasks for: 012-07_guard-opencode-worktree-path

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 012-07_guard-opencode-worktree-path
ito tasks next 012-07_guard-opencode-worktree-path
ito tasks start 012-07_guard-opencode-worktree-path 1.1
ito tasks complete 012-07_guard-opencode-worktree-path 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Design CLI validation command shape

- **Files**: `ito-rs/crates/ito-cli/src/app/*.rs`, `ito-rs/crates/ito-core/src/**/*.rs`, `.ito/changes/012-07_guard-opencode-worktree-path/design.md`
- **Dependencies**: None
- **Action**: Choose the final command name and flags for validating the current worktree for a change, including JSON output for hook callers.
- **Verify**: `cargo test -p ito-cli worktree -- --nocapture`
- **Done When**: The command contract is reflected in CLI parsing tests or command-level tests.
- **Requirements**: cli-config:validate-current-change-worktree
- **Updated At**: 2026-04-26
- **Status**: [x] complete

### Task 1.2: Implement worktree validation logic

- **Files**: `ito-rs/crates/ito-core/src/**/*.rs`, `ito-rs/crates/ito-cli/src/app/*.rs`
- **Dependencies**: Task 1.1
- **Action**: Implement checks for worktree enablement, main/control checkout detection, expected worktree path resolution, and branch/path matching against the full change ID.
- **Verify**: `cargo test -p ito-core worktree` and `cargo test -p ito-cli worktree`
- **Done When**: Validation passes for matching worktrees, fails for main/control, reports mismatches, and no-ops when worktrees are disabled.
- **Requirements**: cli-config:validate-current-change-worktree
- **Updated At**: 2026-04-26
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Integrate OpenCode pre-tool guard

- **Files**: `ito-rs/crates/ito-templates/assets/adapters/opencode/ito-skills.js`, `.opencode/plugins/ito-skills.js`
- **Dependencies**: None
- **Action**: Have the plugin call the validation command before relevant tool use, cache success briefly, and surface actionable failure messages.
- **Verify**: Run the plugin unit or fixture tests for `ito-skills.js`, or add focused tests if no coverage exists.
- **Done When**: The plugin blocks or warns from main/control, allows matching change worktrees, and remains fast through TTL caching.
- **Requirements**: cli-artifact-workflow:opencode-worktree-pretool-guard
- **Updated At**: 2026-04-26
- **Status**: [x] complete

### Task 2.2: Update installed guidance and setup notes

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.opencode/commands/ito-project-setup.md`, `ito-rs/crates/ito-templates/assets/adapters/opencode/ito-skills.js`
- **Dependencies**: Task 2.1
- **Action**: Document the worktree guard environment knobs, expected behavior, and escape hatch if one is needed for debugging.
- **Verify**: `ito update --help` and template rendering/install tests that cover OpenCode assets.
- **Done When**: Installed OpenCode guidance accurately describes the new guard.
- **Requirements**: cli-artifact-workflow:opencode-worktree-pretool-guard
- **Updated At**: 2026-04-26
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Validate proposal and full checks

- **Files**: `.ito/changes/012-07_guard-opencode-worktree-path/**`
- **Dependencies**: None
- **Action**: Validate the Ito proposal and run the project check target after implementation.
- **Verify**: `ito validate 012-07_guard-opencode-worktree-path --strict` and `make check`
- **Done When**: The proposal validates and project checks pass or have documented follow-up blockers.
- **Requirements**: cli-config:validate-current-change-worktree, cli-artifact-workflow:opencode-worktree-pretool-guard
- **Updated At**: 2026-04-25
- **Status**: [ ] pending
<!-- ITO:END -->
