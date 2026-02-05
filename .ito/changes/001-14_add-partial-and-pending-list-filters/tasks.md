# Tasks for: 001-14_add-partial-and-pending-list-filters

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-14_add-partial-and-pending-list-filters
ito tasks next 001-14_add-partial-and-pending-list-filters
ito tasks start 001-14_add-partial-and-pending-list-filters 1.1
ito tasks complete 001-14_add-partial-and-pending-list-filters 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add --partial and --pending CLI flags

- **Files**: ito-rs/crates/ito-cli/src/cli.rs
- **Dependencies**: None
- **Action**:
  Add `--partial` and `--pending` boolean flags to the `ListArgs` struct. Configure clap to make them mutually exclusive with `--completed` and each other using conflict groups.
- **Verify**: `cargo build --package ito-cli`
- **Done When**: CLI compiles with new flags visible in `ito list --help`
- **Updated At**: 2026-02-03
- **Status**: [ ] pending

### Task 1.2: Implement filtering logic

- **Files**: ito-rs/crates/ito-cli/src/app/list.rs
- **Dependencies**: Task 1.1
- **Action**:
  Implement the filtering logic in the list command:
  - `--partial`: filter where `completed > 0 && completed < total && total > 0`
  - `--pending`: filter where `completed == 0 && total > 0`
  Ensure changes with no tasks (total == 0) are excluded from both filters.
- **Verify**: `cargo test --package ito-cli`
- **Done When**: Filtering logic implemented and existing tests pass
- **Updated At**: 2026-02-03
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add unit tests for new filters

- **Files**: ito-rs/crates/ito-cli/src/app/list.rs
- **Dependencies**: None
- **Action**:
  Add unit tests covering:
  - `--partial` returns only changes with 1 to N-1 tasks complete
  - `--pending` returns only changes with 0/N tasks complete
  - Both filters exclude changes with no tasks
  - Mutual exclusivity error messages
- **Verify**: `cargo test --package ito-cli`
- **Done When**: All new tests pass
- **Updated At**: 2026-02-03
- **Status**: [ ] pending

### Task 2.2: Run make check

- **Files**: None
- **Dependencies**: Task 2.1
- **Action**:
  Run `make check` to verify code quality (fmt, clippy, tests).
- **Verify**: `make check`
- **Done When**: All checks pass
- **Updated At**: 2026-02-03
- **Status**: [ ] pending

### Task 2.3: Update agent instructions and skill templates

- **Files**:
  - ito-rs/crates/ito-templates/assets/default/project/AGENTS.md
  - ito-rs/crates/ito-templates/assets/skills/ito-archive/SKILL.md
- **Dependencies**: Task 1.2
- **Action**:
  Update the default agent instructions and relevant skill templates to reference the new `ito list` progress filters (`--pending`, `--partial`, `--completed`).
- **Verify**: `cargo build --package ito-templates`
- **Done When**: Templates mention the new flags and build succeeds
- **Updated At**: 2026-02-03
- **Status**: [ ] pending

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
