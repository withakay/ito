<!-- ITO:START -->
# Tasks for: 012-08_replace-raw-git-worktrees-with-worktrunk

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 012-08_replace-raw-git-worktrees-with-worktrunk
ito tasks next 012-08_replace-raw-git-worktrees-with-worktrunk
ito tasks start 012-08_replace-raw-git-worktrees-with-worktrunk 1.1
ito tasks complete 012-08_replace-raw-git-worktrees-with-worktrunk 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add Worktrunk Invocation Tests

- **Files**: `ito-rs/crates/ito-core/src/worktree_ensure_tests.rs`, `ito-rs/crates/ito-core/tests/worktree_ensure_e2e.rs`
- **Dependencies**: None
- **Action**: Add failing tests proving `ito worktree ensure` invokes `wt switch --create` with a local path configuration that preserves the resolved `ito-worktrees/<change-id>` target.
- **Verify**: `cargo test -p ito-core worktree_ensure`
- **Done When**: Tests fail against the current raw `git worktree add` implementation for the expected reason.
- **Requirements**: `worktree-lifecycle:strategy-aware-creation`, `worktree-lifecycle:local-worktrunk-path-config`, `worktree-lifecycle:worktrunk-failure-diagnostics`
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 1.2: Add Template Rendering Tests

- **Files**: `ito-rs/crates/ito-templates/tests/worktree_template_rendering.rs`, `ito-rs/crates/ito-templates/src/instructions_tests.rs`
- **Dependencies**: None
- **Action**: Add failing tests proving generated worktree instructions render Worktrunk commands and local `ito-worktrees` path configuration guidance instead of raw `git worktree add` snippets.
- **Verify**: `cargo test -p ito-templates worktree_template_rendering`
- **Done When**: Tests fail against current templates because raw git snippets are still rendered.
- **Requirements**: `worktree-aware-template-rendering:agents-md-rendered-with-worktree-context`
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 1.3: Add Ralph Detection Tests

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner/tests.rs`, `ito-rs/crates/ito-core/src/ralph/runner/runner_tests.rs`
- **Dependencies**: None
- **Action**: Add failing tests proving Ralph prefers Worktrunk structured listing for change worktree resolution and falls back to git porcelain when Worktrunk listing is unavailable.
- **Verify**: `cargo test -p ito-core ralph_worktree`
- **Done When**: Tests fail against the current git-porcelain-only detection path.
- **Requirements**: `ralph-worktree-awareness:worktree-detection-uses-git-porcelain-output`
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement Worktrunk-Backed Ensure

- **Files**: `ito-rs/crates/ito-core/src/worktree_ensure.rs`, supporting process/config helpers as needed
- **Dependencies**: None
- **Action**: Replace raw worktree creation with Worktrunk invocation, including deterministic path configuration and actionable diagnostics for missing or failing `wt`.
- **Verify**: `cargo test -p ito-core worktree_ensure`
- **Done When**: Worktree ensure tests pass and stdout remains only the resolved worktree path through the CLI boundary.
- **Requirements**: `worktree-lifecycle:strategy-aware-creation`, `worktree-lifecycle:local-worktrunk-path-config`, `worktree-lifecycle:worktrunk-failure-diagnostics`
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.2: Render Worktrunk Worktree Instructions

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/worktrees.md.j2`, `ito-rs/crates/ito-templates/src/project_templates.rs`, template tests as needed
- **Dependencies**: None
- **Action**: Update worktree guidance templates to describe Worktrunk workflow and local path configuration while preserving portable, marker-managed AGENTS content.
- **Verify**: `cargo test -p ito-templates worktree_template_rendering`
- **Done When**: Rendered instructions contain Worktrunk commands, preserve `ito-worktrees` guidance, and no longer instruct agents to run raw `git worktree add` for normal change worktrees.
- **Requirements**: `worktree-aware-template-rendering:agents-md-rendered-with-worktree-context`
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.3: Prefer Worktrunk Listing In Ralph

- **Files**: `ito-rs/crates/ito-core/src/ralph/runner.rs`, Ralph runner tests as needed
- **Dependencies**: None
- **Action**: Update Ralph worktree resolution to use Worktrunk structured listing when available and retain git porcelain fallback.
- **Verify**: `cargo test -p ito-core ralph_worktree`
- **Done When**: Ralph resolves matching Worktrunk worktrees, excludes bare/control entries, and preserves fallback behavior.
- **Requirements**: `ralph-worktree-awareness:worktree-detection-uses-git-porcelain-output`
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Run Quality and Validation Gates

- **Files**: `.ito/changes/012-08_replace-raw-git-worktrees-with-worktrunk/**`, affected Rust/template files
- **Dependencies**: None
- **Action**: Validate the Ito change, run targeted tests, then run the repository quality gate.
- **Verify**: `ito validate 012-08_replace-raw-git-worktrees-with-worktrunk --strict`, `make check`
- **Done When**: Ito validation and quality checks pass, or any remaining failures are documented with root cause.
- **Requirements**: `worktree-lifecycle:strategy-aware-creation`, `worktree-lifecycle:local-worktrunk-path-config`, `worktree-lifecycle:worktrunk-failure-diagnostics`, `worktree-aware-template-rendering:agents-md-rendered-with-worktree-context`, `ralph-worktree-awareness:worktree-detection-uses-git-porcelain-output`
- **Updated At**: 2026-04-25
- **Status**: [ ] pending
<!-- ITO:END -->
