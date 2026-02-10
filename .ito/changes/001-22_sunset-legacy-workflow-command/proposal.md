## Why

`ito workflow` currently presents a YAML orchestration model that overlaps with the instruction- and skill-driven change workflow, but the two paths are not aligned in behavior or maturity. Consolidating around the working `ito agent instruction` flow reduces user confusion and lets us reuse the strongest workflow concepts (waves, checkpoints, structured execution) where users already work.

## What Changes

- Remove legacy `ito workflow` behavior and make the command group a no-op surface.
- Migrate useful concepts from legacy workflow YAML templates into existing instruction artifacts and skills:
  - Treat "execute workflow" structure as enhanced guidance for `ito agent instruction apply`.
  - Incorporate reusable research-stage guidance into proposal/research skill flows instead of separate workflow templates.
  - Position review as an explicit stage in the proposal lifecycle and instruction flow.
- Update CLI help and user-facing docs to point to `ito agent instruction <artifact>` + skills as the single workflow path.

## Capabilities

### New Capabilities

- `workflow-convergence`: Define the unified behavior when replacing legacy `ito workflow` with instruction- and skill-based workflows.

### Modified Capabilities

- `cli-workflow`: Replace legacy YAML workflow orchestration requirements with no-op semantics and removal of orchestration behavior.
- `agent-instructions`: Expand artifact guidance so proposal/apply/review stages absorb useful structure from legacy research/execute/review workflows.

## Impact

- Affected code: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/workflow.rs`, `ito-rs/crates/ito-core/src/workflow_templates.rs`, instruction generation in `ito-rs/crates/ito-core/src/workflow/` and related command wiring.
- Affected templates/docs: project bootstrap docs, README workflow sections, and skill/instruction references under template assets.
- User impact: one canonical workflow path (`ito agent instruction` + skills), fewer duplicate mental models, and no legacy orchestration behavior behind `ito workflow`.
