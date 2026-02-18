<!-- ITO:START -->
## Why

`ito create module` currently accepts module name, scope, and dependencies, but not a module description argument. Users who script module scaffolding cannot provide descriptive metadata at creation time and must edit `module.md` manually after the command completes.

## What Changes

- Add description argument support to `ito create module` so users can provide module description text at creation time.
- Ensure the verb-first command path (`ito create module`) and the underlying create-module behavior remain aligned and deterministic for non-interactive use.
- Add/extend CLI tests to cover argument parsing and created module metadata when description is provided.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `cli-module`: `ito create module` behavior is expanded to support a description argument.
- `rust-artifact-workflow`: Rust parity behavior for `create module` is expanded to include description-argument handling consistent with TypeScript.

## Impact

- Affected code: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/create.rs`, and create-command integration tests.
- User-facing impact: improved CLI ergonomics for module scaffolding in scripted and one-shot workflows.
- Spec impact: updates required in `cli-module` and `rust-artifact-workflow` capability requirements.
<!-- ITO:END -->
