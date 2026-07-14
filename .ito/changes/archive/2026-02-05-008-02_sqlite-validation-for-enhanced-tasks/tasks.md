# Tasks for: 008-02_sqlite-validation-for-enhanced-tasks

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Update enhanced tasks templates for wave deps + shelving + updated-at

- **Files**: ito-rs/crates/ito-workflow/src/tasks.rs, schemas/spec-driven/templates/tasks.md
- **Dependencies**: None
- **Action**:
  - Add an explicit wave `Depends On` line under each wave header
  - Update examples so task dependencies stay within a wave (no cross-wave task deps)
  - Extend the status legend to include `shelved` and a distinct marker example (e.g. `[-] shelved`)
  - Add `- **Updated At**: YYYY-MM-DD` to each task block
  - Update guidance text to explain: wave deps are cross-wave; task deps are within-wave only
- **Verify**: cargo test -p ito-workflow
- **Done When**: Rust template (and TS template, if still used) matches the new semantics
- **Updated At**: 2026-02-03
- **Status**: [x] complete

### Task 1.2: Extend Rust tasks.md parser/writer for wave deps + shelved + updated-at

- **Files**: ito-rs/crates/ito-workflow/src/tasks.rs
- **Dependencies**: Task 1.1
- **Action**:
  - Parse the wave-level `Depends On` line and represent it in the in-memory task model
  - Accept `shelved` as a valid status and preserve it during round-trip writes
  - Parse and write `**Updated At**: YYYY-MM-DD` and update it on status transitions
  - Enforce task deps are within-wave only and surface actionable diagnostics (path + line)
  - Update readiness evaluation to use wave deps + within-wave deps
- **Verify**: cargo test -p ito-workflow
- **Done When**: Rust parser round-trips updated format and readiness logic matches specs
- **Updated At**: 2026-02-03
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement repo integrity validator (Rust, in-memory SQLite)

- **Files**: ito-rs/crates/ito-workflow/src/lib.rs
- **Dependencies**: None
- **Action**:
  - Build a repository scanner that enumerates modules and changes from `.ito/modules` and `.ito/changes`
  - Load modules/changes into an in-memory relational model (SQLite) and enforce:
    - canonical change identity is numeric-only (`NNN-NN`)
    - duplicate numeric change IDs are an error (e.g. `008-01_foo` and `008-01_bar`)
    - canonical directory naming `NNN-NN_<slug>` is required
  - Produce errors that include both conflicting directory paths and actionable remediation
- **Verify**: cargo test -p ito-workflow
- **Done When**: `ito validate --changes` reports duplicate/invalid change directories correctly
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 2.2: Implement relational validation for waves/tasks/dependencies (Rust)

- **Files**: ito-rs/crates/ito-workflow/src/tasks.rs
- **Dependencies**: Task 2.1
- **Action**:
  - Create tables for waves, tasks, wave deps, and task deps
  - Enforce constraints and queries:
    - task deps do not cross waves
    - no deps on shelved tasks
    - cycle detection for wave deps and task deps
  - Return diagnostics with source locations from the markdown parser
- **Verify**: cargo test -p ito-workflow
- **Done When**: Invalid tasks.md structures are rejected with actionable, line-addressable errors
- **Updated At**: 2026-02-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add Rust CLI support for shelving/unshelving

- **Files**: ito-rs/crates/ito-cli/src/main.rs
- **Dependencies**: None
- **Action**:
  - Add `ito tasks shelve <change-id> <task-id>`
  - Add `ito tasks unshelve <change-id> <task-id>`
  - Enforce status transition rules from the spec
  - Ensure commands refuse to operate when validation errors exist
- **Verify**: cargo test -p ito-cli
- **Done When**: Commands update tasks.md deterministically and validation blocks unsafe operations
- **Updated At**: 2026-02-04
- **Status**: [x] complete

### Task 3.2: Ensure Rust `ito validate` surfaces repo integrity + tasks validation

- **Files**: ito-rs/crates/ito-cli/src/main.rs
- **Dependencies**: None
- **Action**:
  - Wire new repo integrity checks into validate output
  - Ensure errors include file paths and next-step remediation
  - Ensure JSON output includes the new issues with stable fields
- **Verify**: cargo test -p ito-cli
- **Done When**: `ito validate` reports new validations in both text and JSON modes
- **Updated At**: 2026-02-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Remove/avoid TypeScript-first assumptions

- **Files**: .ito/changes/008-02_sqlite-validation-for-enhanced-tasks/design.md
- **Dependencies**: None
- **Action**:
  - Ensure this change remains Rust-first in docs and task plan
  - Ensure any TypeScript parity work is explicitly deferred
- **Verify**: ito validate "008-02_sqlite-validation-for-enhanced-tasks" --strict
- **Done When**: Proposal artifacts reflect Rust-first implementation strategy
- **Updated At**: 2026-02-04
- **Status**: [x] complete

______________________________________________________________________

## Wave 5 (Checkpoint)

- **Depends On**: Wave 4

### Task 5.1: Review format + diagnostics quality

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**: schemas/spec-driven/templates/tasks.md
- **Dependencies**: None
- **Action**:
  - Validate that the updated tasks.md format stays grep/diff friendly
  - Validate that error messages are actionable and point to exact locations
  - Confirm the wave/task dependency scoping matches the intended mental model
- **Done When**: Human reviewer approves format and validator UX
- **Updated At**: 2026-02-04
- **Status**: [-] shelved
