# Change: Validate completion before exiting Ralph loop

## Why

The Ralph loop currently trusts the agent's completion promise without verification. This allows agents to claim "COMPLETE" while leaving build errors, test failures, or other validation issues unresolved. Users have observed loops exiting successfully despite 236+ compilation errors remaining, which violates the principle of verified completion.

## What Changes

- **BREAKING**: Ralph loop will no longer exit immediately upon detecting a completion promise
- Add a post-completion validation step that runs Ito validation commands (`make check`, `make test`, or configurable validation command)
- If validation fails, the loop continues with an injected context message about what failed
- Add `--validation-command` flag to customize the validation step (defaults to `make check`)
- Add `--skip-validation` flag to opt out of validation (preserve current behavior for edge cases)
- Update the preamble to inform agents that completion promises are subject to validation

## Capabilities

### New Capabilities

- `ralph-completion-validation`: Validation logic that runs after a completion promise is detected to verify the work is actually complete before exiting the loop

### Modified Capabilities

- `cli-ralph`: Add `--validation-command` and `--skip-validation` flags; change completion detection to include a validation gate

## Impact

- **Affected specs**: `cli-ralph`, `preamble-generation` (to update preamble text)
- **Affected code**: `ito-rs/crates/ito-core/src/ralph/runner.rs` (main loop logic)
- **User behavior change**: Loops will take longer but produce verified results; agents claiming false completion will be caught and the loop will continue
- **Backward compatibility**: Add `--skip-validation` for users who want the old trust-based behavior
