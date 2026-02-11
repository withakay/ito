# Tasks for: 011-03_generate-config-json-schema

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use Ito tasks CLI for all status changes

```bash
ito tasks status 011-03_generate-config-json-schema
ito tasks next 011-03_generate-config-json-schema
ito tasks start 011-03_generate-config-json-schema 1.1
ito tasks complete 011-03_generate-config-json-schema 1.1
ito tasks show 011-03_generate-config-json-schema
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add canonical schema artifact generation path

- **Files**: `ito-rs/crates/ito-cli/src/app/config.rs`, `schemas/ito-config.schema.json`
- **Dependencies**: None
- **Action**:
  Ensure schema generation produces deterministic output suitable for `schemas/ito-config.schema.json`, and add/refresh the committed artifact.
- **Verify**: `ito config schema --output schemas/ito-config.schema.json`
- **Done When**: Command writes schema to target path and file content is stable across repeated runs.
- **Updated At**: 2026-02-10
- **Status**: [ ] pending

### Task 1.2: Add build/check schema drift verification

- **Files**: `Makefile`, `scripts/` (if needed), CI check entrypoints if applicable
- **Dependencies**: Task 1.1
- **Action**:
  Add a build/check step that regenerates or verifies the schema artifact and fails when committed output is stale.
- **Verify**: `make check`
- **Done When**: Checks fail on schema drift and pass when schema is current.
- **Updated At**: 2026-02-10
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Wire `$schema` references in config templates/files

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/.ito/config.json`, related config templates, loader tests
- **Dependencies**: None
- **Action**:
  Update generated config templates (and any canonical config examples) to reference the committed schema path, while keeping runtime config loading behavior unchanged.
- **Verify**: `cargo test -p ito-cli config_schema` (or nearest relevant test target)
- **Done When**: Generated config files include valid local `$schema` references and config loading still ignores `$schema`.
- **Updated At**: 2026-02-10
- **Status**: [ ] pending

### Task 2.2: Add/extend tests for schema artifact and completion behavior

- **Files**: `ito-rs/crates/ito-cli/tests/*`, `ito-rs/crates/ito-config/tests/*`, `ito-rs/crates/ito-templates/tests/*`
- **Dependencies**: Task 2.1
- **Action**:
  Add tests covering deterministic schema generation, stale-schema failure behavior, and `$schema` compatibility in config parsing.
- **Verify**: `cargo test -p ito-cli && cargo test -p ito-config && cargo test -p ito-templates`
- **Done When**: New tests fail before implementation and pass after implementation.
- **Updated At**: 2026-02-10
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Final validation and docs touch-ups

- **Files**: `README.md`, `docs/` references to schema usage, updated schema artifact
- **Dependencies**: None
- **Action**:
  Document how to reference the local schema and how to regenerate it; run full quality checks.
- **Verify**: `make check && make test`
- **Done When**: Documentation matches behavior, checks pass, and committed schema is current.
- **Updated At**: 2026-02-10
- **Status**: [ ] pending

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
