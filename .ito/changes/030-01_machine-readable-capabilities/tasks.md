# Tasks for: 030-01_machine-readable-capabilities

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 030-01_machine-readable-capabilities
ito tasks next 030-01_machine-readable-capabilities
ito tasks start 030-01_machine-readable-capabilities 1.1
ito tasks complete 030-01_machine-readable-capabilities 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define capabilities JSON contract

- **Files**: `ito-rs/crates/ito-cli/src/**`, `ito-rs/crates/ito-core/src/**`, `schemas/**`
- **Dependencies**: None
- **Action**: Add tests for `ito capabilities --json` producing a versioned manifest with command entries, artifact entries, aliases, and JSON support metadata.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests fail before implementation and assert the required manifest fields.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 1.2: Cover focused capabilities queries

- **Files**: `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Add tests for `ito capabilities command tasks --json`, `ito capabilities artifacts --json`, and `ito capabilities aliases --json`.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Tests assert focused responses include only the requested capability subset.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement capabilities command and data model

- **Files**: `ito-rs/crates/ito-cli/src/**`, `ito-rs/crates/ito-core/src/**`
- **Dependencies**: None
- **Action**: Add the `capabilities` command and response structs for commands, flags, aliases, examples, artifacts, JSON support, deprecations, and replacements.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: `ito capabilities --json` emits valid deterministic JSON.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

### Task 2.2: Derive command tree metadata

- **Files**: `ito-rs/crates/ito-cli/src/**`
- **Dependencies**: None
- **Action**: Populate command paths, flags, positional args, aliases, summaries, and subcommands from the Clap command tree where possible.
- **Verify**: `bash ito-rs/tools/test-affected.sh`
- **Done When**: Capability output tracks the real command tree without duplicating basic flag metadata manually.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add guidance and verification

- **Files**: `ito-rs/crates/ito-templates/assets/**`, `docs/**`, `ito-rs/crates/ito-cli/tests/**`
- **Dependencies**: None
- **Action**: Update agent guidance to prefer `ito capabilities --json` before uncommon commands and add regression coverage for common command entries.
- **Verify**: `make check`
- **Done When**: Guidance is updated and project validation passes.
- **Updated At**: 2026-05-31
- **Status**: [ ] pending
