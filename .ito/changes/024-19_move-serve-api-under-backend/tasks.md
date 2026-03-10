# Tasks for: 024-19_move-serve-api-under-backend

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 024-19_move-serve-api-under-backend
ito tasks next 024-19_move-serve-api-under-backend
ito tasks start 024-19_move-serve-api-under-backend 1.1
ito tasks complete 024-19_move-serve-api-under-backend 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add `ito backend serve` command wiring

- **Files**: `ito-rs/crates/ito-cli/src/cli/`, `ito-rs/crates/ito-cli/src/commands/`, `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: None
- **Action**: Move backend server startup into the backend command group while reusing the existing serve implementation and preserving current flags/config behavior.
- **Verify**: `cargo test -p ito-cli backend`
- **Done When**: `ito backend serve` starts the backend server successfully and current serve-mode tests pass through the new command path.
- **Updated At**: 2026-03-10
- **Status**: [ ] pending

### Task 1.2: Remove top-level `serve-api` and add migration guidance

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/`, `ito-rs/crates/ito-cli/tests/`
- **Dependencies**: Task 1.1
- **Action**: Remove the top-level `serve-api` entrypoint and replace any remaining invocation path with actionable guidance to use `ito backend serve`.
- **Verify**: `cargo test -p ito-cli cli`
- **Done When**: `ito serve-api` is no longer a supported command and users are guided to `ito backend serve` instead.
- **Updated At**: 2026-03-10
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update docs, prompts, and QA workflows for the new command

- **Files**: `ito-rs/crates/ito-cli/tests/`, `qa/backend/`, `.ito/`, `ito-rs/crates/ito-templates/`
- **Dependencies**: None
- **Action**: Update backend instructions, QA walkthroughs, help snapshots, and related docs to reference `ito backend serve` as the canonical startup command.
- **Verify**: `cargo test -p ito-cli --test backend_qa_walkthrough && make check`
- **Done When**: No backend startup guidance points to `ito serve-api`, and QA/docs validation passes under the new command path.
- **Updated At**: 2026-03-10
- **Status**: [ ] pending
