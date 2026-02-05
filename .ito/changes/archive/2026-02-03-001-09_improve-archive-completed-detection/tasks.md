# Tasks for: 001-09_improve-archive-completed-detection

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-09_improve-archive-completed-detection
ito tasks next 001-09_improve-archive-completed-detection
ito tasks start 001-09_improve-archive-completed-detection 1.1
ito tasks complete 001-09_improve-archive-completed-detection 1.1
```

______________________________________________________________________

## Wave 1: Core Data Model Changes

- **Depends On**: None

### Task 1.1: Add completed field to ChangeListItem

- **Files**: `ito-rs/crates/ito-core/src/list.rs`
- **Dependencies**: None
- **Action**:
  Add a `completed: bool` field to the `ChangeListItem` struct. The field should be `true` when `completed_tasks == total_tasks && total_tasks > 0`, otherwise `false`. Update the struct's serde attributes to include this field in JSON output.
- **Verify**: `cargo test -p ito-core`
- **Done When**: ChangeListItem struct has completed field and existing tests pass
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 1.2: Update list command JSON serialization

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Update the list command logic (around line 2576-2633) to populate the new `completed` field when building ChangeListItem instances. The logic is: `completed_tasks == total_tasks && total_tasks > 0`.
- **Verify**: `ito list --json | jq '.[0].completed'`
- **Done When**: JSON output includes `"completed": true/false` for each change
- **Updated At**: 2026-02-02
- **Status**: [x] complete

______________________________________________________________________

## Wave 2: CLI Filter Implementation

- **Depends On**: Wave 1

### Task 2.1: Add --completed flag to list command

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: None
- **Action**:
  Add a `--completed` flag to the list command argument parsing. When set, filter the changes list to only include items where `completed == true`. Update the help text constant HELP and any relevant documentation strings.
- **Verify**: `ito list --completed` should only show completed changes
- **Done When**: Running `ito list --completed` filters to completed changes only; `ito list --help` shows the new flag
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 2.2: Handle empty completed list gracefully

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: Task 2.1
- **Action**:
  When `--completed` is used and no completed changes exist, display an informational message like "No completed changes found. Run `ito list` to see all changes." rather than showing an empty table.
- **Verify**: Create a test scenario with no completed changes and run `ito list --completed`
- **Done When**: User sees helpful message instead of empty output when no completed changes exist
- **Updated At**: 2026-02-02
- **Status**: [x] complete

______________________________________________________________________

## Wave 3: Skill Update

- **Depends On**: Wave 2

### Task 3.1: Update ito-archive skill for interactive selection

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.claude/skill/ito-archive/skill.md`
- **Dependencies**: None
- **Action**:
  Update the ito-archive skill to support interactive selection when no change ID is provided:
  1. Add a check at the start: if no change ID argument, run `ito list --completed --json`
  2. If empty result, inform user no changes are ready to archive and suggest `ito list` or `--no-validate`
  3. If non-empty, present the list and ask user to select which change(s) to archive
  4. Preserve existing behavior when change ID is explicitly provided
- **Verify**: Test the skill invocation without a change ID
- **Done When**: `/ito-archive` without args prompts for selection from completed changes
- **Updated At**: 2026-02-02
- **Status**: [x] complete

______________________________________________________________________

## Wave 4: Testing & Validation

- **Depends On**: Wave 3

### Task 4.1: Add integration tests for completed status

- **Files**: `ito-rs/crates/ito-cli/tests/` or relevant test file
- **Dependencies**: None
- **Action**:
  Add tests that verify:
  1. Changes with all tasks completed show `completed: true` in JSON
  2. Changes with partial tasks show `completed: false`
  3. Changes with no tasks show `completed: false`
  4. `--completed` flag filters correctly
- **Verify**: `cargo test -p ito-cli`
- **Done When**: All new tests pass and cover the documented scenarios
- **Updated At**: 2026-02-02
- **Status**: [x] complete

### Task 4.2: Manual validation of full workflow

- **Files**: None (manual testing)
- **Dependencies**: Task 4.1
- **Action**:
  Manually test the complete workflow:
  1. Run `ito list` and verify "✓ Complete" display for completed changes
  2. Run `ito list --json` and verify `completed` boolean field
  3. Run `ito list --completed` and verify filtering
  4. Test `/ito-archive` skill without change ID
- **Verify**: Manual verification
- **Done When**: All scenarios work as expected
- **Updated At**: 2026-02-02
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
