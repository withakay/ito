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
- **Updated At**: 2026-02-15
- **Status**: [ ] pending

### Task 2.2: Update instruction templates to emit absolute paths

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/**`
- **Dependencies**: Task 2.1
- **Action**: Replace relative path output with absolute paths derived from project root/worktree context.
- **Verify**: `rg "(^|\\s)(\\.|\\.\\.)/" ito-rs/crates/ito-templates/assets/instructions`
- **Done When**: Instruction templates render absolute paths everywhere paths are displayed.
- **Updated At**: 2026-02-15
- **Status**: [ ] pending

### Task 2.3: Audit project templates and skills for relative paths

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`, `ito-rs/crates/ito-templates/assets/skills/**/SKILL.md`
- **Dependencies**: Task 2.1
- **Action**: Scan project templates and skills for relative path instructions that need absolute paths.
- **Verify**: `rg "(^|\\s)(\\.|\\.\\.)/" ito-rs/crates/ito-templates/assets/default/project ito-rs/crates/ito-templates/assets/skills`
- **Done When**: All relative path occurrences in project templates and skills are identified.
- **Updated At**: 2026-02-15
- **Status**: [ ] pending

### Task 2.4: Update project templates and skills to emit absolute paths

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`, `ito-rs/crates/ito-templates/assets/skills/**/SKILL.md`
- **Dependencies**: Task 2.3
- **Action**: Replace relative path output with absolute paths derived from project root/worktree context.
- **Verify**: `rg "(^|\\s)(\\.|\\.\\.)/" ito-rs/crates/ito-templates/assets/default/project ito-rs/crates/ito-templates/assets/skills`
- **Done When**: Project templates and skills render absolute paths everywhere paths are displayed.
- **Updated At**: 2026-02-15
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2
- **Goal**: CLI text output normalization

### Task 3.1: Audit CLI text output for relative paths

- **Files**: `ito-rs/crates/ito-cli/src/commands/**`, `ito-rs/crates/ito-cli/src/app/**`, `ito-rs/crates/ito-cli/src/output/**`
- **Dependencies**: None
- **Action**: Inventory commands and error paths that emit filesystem paths in text output (list/show/validate/tasks and errors).
- **Verify**: `rg "(^|\\s)(\\.|\\.\\.)/" ito-rs/crates/ito-cli/src`
- **Done When**: All text output locations that emit paths are identified with intended absolute formatting.
- **Updated At**: 2026-02-15
- **Status**: [ ] pending

### Task 3.2: Update CLI text output to absolute paths

- **Files**: `ito-rs/crates/ito-cli/src/commands/**`, `ito-rs/crates/ito-cli/src/app/**`, `ito-rs/crates/ito-cli/src/output/**`
- **Dependencies**: Task 3.1
- **Action**: Normalize text output path formatting to use absolute paths derived from project root.
- **Verify**: `cargo test -p ito-cli`
- **Done When**: CLI text output consistently uses absolute paths and tests pass.
- **Updated At**: 2026-02-15
- **Status**: [ ] pending

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3
- **Goal**: JSON output and tests

### Task 4.1: Ensure JSON output path fields are absolute

- **Files**: `ito-rs/crates/ito-cli/src/commands/**`, `ito-rs/crates/ito-cli/src/output/**`
- **Dependencies**: None
- **Action**: Normalize JSON output to emit absolute filesystem paths for all path fields.
- **Verify**: `cargo test -p ito-cli`
- **Done When**: All `--json` outputs use absolute paths and tests pass.
- **Updated At**: 2026-02-15
- **Status**: [ ] pending

### Task 4.2: Add/update tests for absolute path rendering

- **Files**: `ito-rs/crates/ito-cli/tests/**`, `ito-rs/crates/ito-templates/tests/**`
- **Dependencies**: Task 4.1
- **Action**: Add or update tests/snapshots to assert absolute paths in template rendering and CLI outputs.
- **Verify**: `cargo test -p ito-cli && cargo test -p ito-templates`
- **Done When**: Tests cover absolute path expectations and pass.
- **Updated At**: 2026-02-15
- **Status**: [ ] pending

______________________________________________________________________

## Wave 5

- **Depends On**: Wave 4
- **Goal**: Validation

### Task 5.1: Run checks and validate change

- **Files**: `ito-rs/**`, `.ito/changes/019-01_absolute-paths-in-output/**`
- **Dependencies**: None
- **Action**: Run repo checks and validate the change.
- **Verify**: `make check && ./target/debug/ito validate 019-01 --strict`
- **Done When**: Checks and validation pass without errors.
- **Updated At**: 2026-02-15
- **Status**: [ ] pending

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
