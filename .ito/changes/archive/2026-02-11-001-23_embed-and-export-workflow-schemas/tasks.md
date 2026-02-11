# Tasks for: 001-23_embed-and-export-workflow-schemas

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use Ito tasks CLI for state transitions

```bash
ito tasks status 001-23_embed-and-export-workflow-schemas
ito tasks next 001-23_embed-and-export-workflow-schemas
ito tasks start 001-23_embed-and-export-workflow-schemas 1.1
ito tasks complete 001-23_embed-and-export-workflow-schemas 1.1
ito tasks show 001-23_embed-and-export-workflow-schemas
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Embed workflow schemas in ito-templates

- **Files**: `ito-rs/crates/ito-templates/assets/schemas/**`, `ito-rs/crates/ito-templates/src/lib.rs`
- **Dependencies**: None
- **Action**:
  Move/copy built-in schema directories into template assets and expose accessors for embedded schema content.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Embedded schema assets are available through typed template APIs and tests pass.
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 1.2: Update resolver precedence for schema loading

- **Files**: `ito-rs/crates/ito-core/src/workflow/mod.rs`, resolver-related tests
- **Dependencies**: Task 1.1
- **Action**:
  Implement schema source precedence: `.ito/templates/schemas` -> `${XDG_DATA_HOME}/ito/schemas` -> embedded built-ins (plus temporary legacy fallback if retained).
- **Verify**: `cargo test -p ito-core workflow`
- **Done When**: Resolver loads expected source for each precedence level and list output aggregates all sources.
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add CLI command for schema export

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/**`, `ito-rs/crates/ito-core/src/**` (if needed)
- **Dependencies**: None
- **Action**:
  Add `ito templates schemas export` command with `-f/--to` destination and force-overwrite behavior.
- **Verify**: `cargo test -p ito-cli templates`
- **Done When**: Command exports schema directories with deterministic files and documented overwrite semantics.
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 2.2: Add integration tests for export and overrides

- **Files**: `ito-rs/crates/ito-cli/tests/**`, `ito-rs/crates/ito-core/tests/**`
- **Dependencies**: Task 2.1
- **Action**:
  Add tests that prove project-local override precedence and validate export output/force behavior.
- **Verify**: `cargo test -p ito-cli && cargo test -p ito-core`
- **Done When**: Failing tests are added first (RED), then pass after implementation (GREEN), with cleanup/refactor completed.
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Migrate docs and finalize validation

- **Files**: `docs/schema-customization.md`, `README.md` (if needed), change artifacts
- **Dependencies**: None
- **Action**:
  Update docs to describe embedded defaults, local override path, and export workflow; run full checks.
- **Verify**: `make check && make test`
- **Done When**: Documentation aligns with command behavior and repository checks pass.
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
