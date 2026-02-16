# Tasks for: 019-01_absolute-paths-in-output

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 019-01_absolute-paths-in-output
ito tasks next 019-01_absolute-paths-in-output
ito tasks start 019-01_absolute-paths-in-output 1.1
ito tasks complete 019-01_absolute-paths-in-output 1.1
ito tasks shelve 019-01_absolute-paths-in-output 1.1
ito tasks unshelve 019-01_absolute-paths-in-output 1.1
ito tasks show 019-01_absolute-paths-in-output
```

______________________________________________________________________

## Wave 1

- **Depends On**: None
- **Goal**: Completed groundwork

### Task 1.1: Add project_root context and update worktree instruction templates (complete)

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-templates/assets/instructions/agent/apply.md.j2`, `ito-rs/crates/ito-templates/assets/instructions/agent/worktrees.md.j2`
- **Dependencies**: None
- **Action**: Ensure `WorktreeConfig` carries `project_root` for all strategies and worktree instruction templates emit absolute paths.
- **Verify**: `rg "project_root" ito-rs/crates/ito-cli/src/app/instructions.rs ito-rs/crates/ito-templates/assets/instructions/agent/apply.md.j2 ito-rs/crates/ito-templates/assets/instructions/agent/worktrees.md.j2`
- **Done When**: Worktree config includes `project_root` and worktree instruction templates render absolute paths.
- **Updated At**: 2026-02-15
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1
- **Goal**: Template audits and updates

### Task 2.1: Audit instruction templates for remaining relative paths

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/**`
- **Dependencies**: None
- **Action**: Scan instruction templates for relative path usage and record locations needing absolute paths.
- **Verify**: `rg "(^|\\s)(\\.|\\.\\.)/" ito-rs/crates/ito-templates/assets/instructions`
- **Done When**: A list of relative path occurrences is captured with intended absolute replacements.
- **Updated At**: 2026-02-16
- **Status**: [x] complete

### Task 2.2: Update instruction templates to emit absolute paths

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/**`
- **Dependencies**: Task 2.1
- **Action**: Replace relative path output with absolute paths derived from project root/worktree context.
- **Verify**: `rg "(^|\\s)(\\.|\\.\\.)/" ito-rs/crates/ito-templates/assets/instructions`
- **Done When**: Instruction templates render absolute paths everywhere paths are displayed.
- **Updated At**: 2026-02-16
- **Status**: [x] complete

### Task 2.3: Audit project templates and skills for relative paths

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`, `ito-rs/crates/ito-templates/assets/skills/**/SKILL.md`
- **Dependencies**: Task 2.1
- **Action**: Scan project templates and skills for machine-specific absolute paths (or use of `project_root`) that would make committed files non-portable.
- **Verify**: `rg "project_root|/Users/|[A-Za-z]:\\\\" ito-rs/crates/ito-templates/assets/default/project ito-rs/crates/ito-templates/assets/skills`
- **Done When**: Any absolute-path embedding in committed templates is identified.
- **Updated At**: 2026-02-16
- **Status**: [x] complete

### Task 2.4: Update project templates and skills to remain portable

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`, `ito-rs/crates/ito-templates/assets/skills/**/SKILL.md`
- **Dependencies**: Task 2.3
- **Action**: Remove any embedded absolute paths from committed templates (use repo-relative paths). If an absolute path is needed at runtime, prefer instructing callers to use `ito path ...`.
- **Verify**: `rg "project_root|/Users/|[A-Za-z]:\\\\" ito-rs/crates/ito-templates/assets/default/project ito-rs/crates/ito-templates/assets/skills`
- **Done When**: Project templates and skills do not embed machine-specific absolute paths.
- **Updated At**: 2026-02-16
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2
- **Goal**: CLI text output normalization

### Task 3.1: Audit CLI text output for relative paths

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/**`, `ito-rs/crates/ito-cli/src/app/instructions.rs`
- **Dependencies**: None
- **Action**: Generate agent-facing instruction artifacts and identify any relative filesystem paths that should be absolute.
- **Verify**: `./target/debug/ito agent instruction apply --change 019-01_absolute-paths-in-output | rg "(^|\\s)(\\.|\\.\\.)/" || true`
- **Done When**: Any remaining relative path occurrences in agent-facing output are identified (or explicitly documented as relative exceptions).
- **Updated At**: 2026-02-16
- **Status**: [x] complete

### Task 3.2: Update CLI text output to absolute paths

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/**`, `ito-rs/crates/ito-cli/src/app/instructions.rs`
- **Dependencies**: Task 3.1
- **Action**: Update agent-facing instruction templates so any displayed filesystem paths are absolute (and recommend using `ito path ...` when helpful).
- **Verify**: `cargo test -p ito-cli`
- **Done When**: Agent-facing instruction output uses absolute paths where paths are displayed and tests pass.
- **Updated At**: 2026-02-16
- **Status**: [x] complete

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3
- **Goal**: JSON output and tests

### Task 4.1: Ensure JSON output path fields are absolute

- **Files**: `ito-rs/crates/ito-cli/src/commands/path.rs`
- **Dependencies**: None
- **Action**: Ensure `ito path ... --json` outputs absolute filesystem paths.
- **Verify**: `cargo test -p ito-cli --test path_more`
- **Done When**: `ito path ... --json` outputs are absolute and tests pass.
- **Updated At**: 2026-02-16
- **Status**: [x] complete

### Task 4.2: Add/update tests for absolute path rendering

- **Files**: `ito-rs/crates/ito-cli/tests/**`, `ito-rs/crates/ito-templates/tests/**`
- **Dependencies**: Task 4.1
- **Action**: Add or update tests/snapshots to assert portability of committed templates and absolute paths in agent-facing output.
- **Verify**: `cargo test -p ito-cli && cargo test -p ito-templates`
- **Done When**: Tests cover the intended expectations and pass.
- **Updated At**: 2026-02-16
- **Status**: [x] complete

______________________________________________________________________

## Wave 5

- **Depends On**: Wave 4
- **Goal**: Validation

### Task 5.1: Run checks and validate change

- **Files**: `ito-rs/**`, `.ito/changes/019-01_absolute-paths-in-output/**`
- **Dependencies**: None
- **Action**: Run repo checks and validate the change.
- **Verify**: `cargo test -p ito-cli && cargo test -p ito-templates && ./target/debug/ito validate 019-01 --strict`
- **Done When**: Focused tests and validation pass without errors.
- **Updated At**: 2026-02-16
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)

## Wave Guidelines

- Waves group related tasks that can be executed in parallel
- Task dependencies must be complete before starting dependent tasks
- Wave dependencies are declared via `- **Depends On**: ...`
- Task dependencies MUST be within the same wave
- Checkpoint waves require human approval before proceeding

## Task Structure

Each task should include:

- **ID**: Unique identifier (wave.task)
- **Files**: Which files this task affects
- Dependencies field: Other tasks that must complete first (or "None")
- **Action**: What to implement or do
- **Verify**: Command to verify completion (optional but recommended)
- **Done When**: Acceptance criteria
- **Updated At**: Date of last status change (YYYY-MM-DD)
- **Status**: Current status (pending/in-progress/complete/shelved)
