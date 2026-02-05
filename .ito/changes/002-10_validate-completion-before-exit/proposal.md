# Change: Validate completion before exiting Ralph loop

## Why

The Ralph loop currently trusts the agent's completion promise without verification. This allows agents to claim "COMPLETE" while leaving build errors, test failures, or incomplete tasks. Users have observed loops exiting successfully despite 236+ compilation errors remaining, which violates the principle of verified completion.

The loop already has access to the change ID and can use Ito's own tooling to verify task completion status. It should also run the project's standard validation (tests, checks, lints) before accepting completion.

## What Changes

- **BREAKING**: Ralph loop will no longer exit immediately upon detecting a completion promise
- Add Ito-native validation: check task completion status via `ito tasks status <change-id>`
  - All tasks must be `complete` or `shelved` (with reason)
  - If tasks remain pending/in-progress, reject completion and continue
- Add project validation: run the project's standard checks (`make check`, `make test`, or configured command)
  - Ralph should *always* run project validation when a completion promise is detected
  - This catches build errors, test failures, lint issues, etc.
- Add `--validation-command` flag for additional explicit validation beyond project defaults
- Add `--skip-validation` flag to opt out of all validation (escape hatch for edge cases)
- If any validation fails, inject failure output as context and continue to next iteration
- Update the preamble to inform agents that completion promises are validated

## Validation Order

When a completion promise is detected:

1. **Ito task status** (if change-id provided): Verify all tasks complete/shelved
2. **Project validation**: Run validation commands from project configuration (ito.json, .ito/config.json, AGENTS.md, CLAUDE.md)
3. **Extra validation** (if `--validation-command` specified): Run additional explicit check

All must pass for completion to be accepted.

## Capabilities

### New Capabilities

- `ralph-completion-validation`: Validation logic that verifies task completion status and runs project validation before accepting a completion promise

### Modified Capabilities

- `cli-ralph`: Add `--validation-command` and `--skip-validation` flags; change completion detection to include validation gate

## Impact

- **Affected specs**: `cli-ralph`
- **Affected code**: 
  - `ito-rs/crates/ito-core/src/ralph/runner.rs` (main loop logic)
  - `ito-rs/crates/ito-core/src/ralph/prompt.rs` (context injection)
- **User behavior change**: Loops will take longer but produce verified results; agents claiming false completion will be caught and the loop will continue with feedback
- **Backward compatibility**: Add `--skip-validation` for users who want the old trust-based behavior
