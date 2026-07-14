# Tasks for: 001-22_sunset-legacy-workflow-command

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-22_sunset-legacy-workflow-command
ito tasks next 001-22_sunset-legacy-workflow-command
ito tasks start 001-22_sunset-legacy-workflow-command 1.1
ito tasks complete 001-22_sunset-legacy-workflow-command 1.1
ito tasks shelve 001-22_sunset-legacy-workflow-command 1.1
ito tasks unshelve 001-22_sunset-legacy-workflow-command 1.1
ito tasks show 001-22_sunset-legacy-workflow-command
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Remove legacy workflow command wiring

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/app/run.rs`, `ito-rs/crates/ito-cli/src/commands/mod.rs`, `ito-rs/crates/ito-cli/src/commands/workflow.rs`
- **Dependencies**: None
- **Action**:
  Remove legacy orchestration routing and wire `ito workflow` commands to deterministic no-op handlers.
- **Verify**: `cargo test -p ito-cli`
- **Done When**: CLI no longer executes legacy workflow template operations and tests pass.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 1.2: Remove legacy workflow template plumbing

- **Files**: `ito-rs/crates/ito-core/src/workflow_templates.rs`, `ito-rs/crates/ito-core/tests/workflow_templates.rs`, `ito-rs/crates/ito-core/src/lib.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Remove unused workflow-template initialization/list/load behavior, ensuring no active runtime path performs workflow orchestration.
- **Verify**: `cargo test -p ito-core`
- **Done When**: No active runtime path depends on legacy workflow template scaffolding.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Converge proposal/apply/review instruction content

- **Files**: `ito-rs/crates/ito-core/src/workflow/mod.rs`, `ito-rs/crates/ito-cli/src/commands/agent.rs`, `schemas/spec-driven/schema.yaml`, `schemas/spec-driven/templates/*.md`
- **Dependencies**: None
- **Action**:
  Update instruction artifacts so proposal includes stronger research framing, apply includes structured execution/checkpoints, and review is explicitly represented as a lifecycle stage.
- **Verify**: `cargo test -p ito-core && cargo test -p ito-cli`
- **Done When**: Instruction outputs reflect converged workflow concepts and existing behavior remains stable.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 2.2: Update skills and docs to canonical workflow path

- **Files**: `README.md`, `.opencode/skills/ito-workflow/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-workflow/SKILL.md`, `ito-rs/crates/ito-templates/assets/default/project/AGENTS.md`
- **Dependencies**: Task 2.1
- **Action**:
  Rewrite workflow guidance to center on `ito agent instruction` + skills and remove legacy `ito workflow` orchestration usage examples.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Documentation and skill text consistently describe one canonical workflow path.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Validate change integrity and no-op semantics

- **Files**: `.ito/changes/001-22_sunset-legacy-workflow-command/specs/**/*.md`, `.ito/specs/cli-workflow/spec.md`, `.ito/specs/agent-instructions/spec.md`
- **Dependencies**: None
- **Action**:
  Run strict validation and targeted command checks to confirm `ito workflow` commands are no-ops and spec deltas remain valid.
- **Verify**: `ito validate 001-22_sunset-legacy-workflow-command --strict`
- **Done When**: Validation succeeds and no-op behavior is documented/tested.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 4 (Checkpoint)

- **Depends On**: Wave 3

### Task 4.1: Approve no-op completion scope

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**: `.ito/changes/001-22_sunset-legacy-workflow-command/proposal.md`, `.ito/changes/001-22_sunset-legacy-workflow-command/design.md`
- **Dependencies**: None
- **Action**:
  Review that `ito workflow` is fully no-op and that instruction/skill workflow guidance is complete.
- **Done When**: Human reviewer confirms no-op rollout scope.
- **Updated At**: 2026-02-10
- **Status**: [x] complete
