# Tasks for: 016-11_module-description-args

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 016-11_module-description-args
ito tasks next 016-11_module-description-args
ito tasks start 016-11_module-description-args 1.1
ito tasks complete 016-11_module-description-args 1.1
ito tasks show 016-11_module-description-args
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add failing integration coverage for module description argument

- **Files**: `ito-rs/crates/ito-cli/tests/create_more.rs`
- **Dependencies**: None
- **Action**:
  Add an integration test that invokes `ito create module <name> --description <text>` and asserts description metadata is persisted in the created module artifact.
- **Verify**: `cargo test -p ito-cli --test create_more`
- **Done When**: Test fails on current behavior and demonstrates missing description-argument support.
- **Updated At**: 2026-02-18
- **Status**: [ ] pending

### Task 1.2: Implement clap and forwarding support for module description

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/create.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Add a `--description` argument to `create module` clap definitions and ensure argument forwarding reaches create-module execution path consistently.
- **Verify**: `cargo test -p ito-cli --test create_more`
- **Done When**: New and existing create-module tests pass, including the description-argument case.
- **Updated At**: 2026-02-18
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Confirm compatibility and output stability

- **Files**: `ito-rs/crates/ito-cli/tests/cli_smoke.rs` (if update needed), `ito-rs/crates/ito-cli/tests/create_more.rs`
- **Dependencies**: None
- **Action**:
  Add or adjust assertions to confirm create-module success output and existing behavior remain stable when description is omitted or provided.
- **Verify**: `cargo test -p ito-cli --test create_more && cargo test -p ito-cli --test cli_smoke`
- **Done When**: Regression coverage exists for both legacy and new invocation forms.
- **Updated At**: 2026-02-18
- **Status**: [ ] pending

### Task 2.2: Run focused validation for strict completion

- **Files**: `.ito/changes/016-11_module-description-args/proposal.md`, `.ito/changes/016-11_module-description-args/specs/cli-module/spec.md`, `.ito/changes/016-11_module-description-args/specs/rust-artifact-workflow/spec.md`, `.ito/changes/016-11_module-description-args/design.md`, `.ito/changes/016-11_module-description-args/tasks.md`
- **Dependencies**: Task 2.1
- **Action**:
  Validate change artifacts and confirm spec/task consistency before implementation handoff.
- **Verify**: `ito validate 016-11_module-description-args --strict`
- **Done When**: Validation passes without strict-mode errors.
- **Updated At**: 2026-02-18
- **Status**: [ ] pending

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
