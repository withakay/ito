## Context

Ito currently carries two workflow models:

1. `ito workflow` for YAML-defined orchestration (`init/list/show` implemented, with broader behavior specified but incomplete)
2. Instruction- and skill-driven change delivery via `ito agent instruction` plus `ito tasks`

This split creates duplicated concepts (research/execute/review), mixed messaging in docs/help, and implementation drift between specification and runtime behavior.

## Goals / Non-Goals

**Goals:**

- Remove the legacy `ito workflow` command family as a supported day-to-day path.
- Preserve useful workflow concepts by integrating them into proposal/apply/review instruction artifacts and skills.
- Provide a single canonical workflow model while leaving `ito workflow` as a no-op compatibility surface.

**Non-Goals:**

- Rebuilding a new generic YAML orchestration engine under a different command name.
- Changing the fundamental artifact schema contract (`proposal/specs/design/tasks/apply/review/archive`).
- Large redesign of task parsing or unrelated CLI command groups.

## Decisions

### Decision: Retire `ito workflow` command surface

- Remove command registration and command handler wiring for legacy workflow operations.
- Replace execution behavior with deterministic no-op behavior.

**Rationale:** The command family is partially implemented and overlaps with better-adopted flows, increasing maintenance and user confusion.

**Alternatives considered:**

- Keep command and finish full implementation: rejected due to duplicate orchestration model and higher maintenance cost.
- Keep command as hidden alias: rejected because it preserves conceptual drift and delayed cleanup.

### Decision: Fold legacy workflow value into instruction artifacts

- Use `proposal` instructions for structured research framing.
- Use `apply` instructions for structured execution with checkpoints and task-state guidance.
- Use `review` instructions as an explicit lifecycle stage tied to proposal/specs/tasks.

**Rationale:** Reuses working infrastructure and keeps one source of truth for agent workflows.

**Alternatives considered:**

- Keep standalone research/execute/review workflow YAML templates: rejected because they are parallel artifacts with overlapping purpose.

### Decision: Introduce explicit convergence specification

- Add a dedicated capability (`workflow-convergence`) to capture canonical workflow intent and guard against regression into dual systems.

**Rationale:** Makes the consolidation contract visible and testable beyond one-off code changes.

## Risks / Trade-offs

- Existing users/scripts may rely on `ito workflow` commands -> Mitigation: keep command namespace as no-op and remove side effects.
- Consolidation may reduce flexibility for teams using custom workflow YAML -> Mitigation: document extension via schemas, instruction templates, and skills.
- Multiple docs/templates need synchronized updates -> Mitigation: include docs/template updates in the same change and verify help/readme consistency.

## Rollout Plan

1. Update specs to retire `cli-workflow` requirements and define converged behavior.
2. Replace CLI behavior for `ito workflow` subcommands with deterministic no-op handlers.
3. Update instruction-generation content for proposal/apply/review to absorb useful legacy workflow structure.
4. Update README/help/template references to canonical instruction-and-skill workflow.
5. Validate with strict spec checks and targeted CLI tests for no-op semantics.

Rollback strategy:

- If no-op handling regresses command compatibility, restore a minimal command shim that still performs no orchestration side effects.

## Open Questions

- Which exact research prompts from legacy workflow templates are valuable enough to migrate into proposal/research instruction content?
