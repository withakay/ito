<!-- ITO:START -->
## Context

Ito already has two partial answers to proposal discovery: `ito-proposal` asks lightweight clarifying questions before scaffolding, and `ito-brainstorming` explores broader design options. Neither one cleanly handles the common cases where a user knows they want either a fix or a feature, but does not know how much workflow they need.

The result is predictable drift toward `spec-driven`, even though Ito already ships `minimalist` and `tdd`. This is especially visible for small fixes, regression work, and supporting platform or infrastructure changes that still deserve rigor but do not need a full proposal/design stack.

## Goals / Non-Goals

**Goals:**

- Add a Stage 0 intake flow that improves understanding before change scaffolding.
- Introduce intent-biased entrypoints for fix and feature workflows.
- Make schema recommendation explicit and teach when `minimalist` and `tdd` are a better fit than `spec-driven`.
- Cover non-product work such as platform, tooling, release, and infrastructure changes in the same decision model.

**Non-Goals:**

- Adding a brand-new schema in this change.
- Replacing `ito-proposal` as the canonical neutral workflow entrypoint.
- Adding native Rust CLI subcommands if command-wrapper assets are sufficient to prove the workflow first.

## Decisions

### 1. Add a dedicated intake capability before proposal scaffolding

**Decision**: Introduce a narrow intake flow whose job is to clarify the requested change, determine whether a proposal is needed, and hand off a concise summary into proposal creation.

**Rationale**: The current gap is not absence of questions; it is absence of a first-class intake stage with explicit outcomes.

### 2. Use intent-biased entrypoints instead of a new schema

**Decision**: Add `ito-fix` and `ito-feature` entrypoints that route into the existing proposal workflow with different defaults, while keeping `ito-proposal` as the neutral lane.

**Rationale**: This makes the workflow easier to choose without proliferating schemas. The bias should live in intake questions and schema recommendations, not in more schema definitions.

### 3. Treat schema selection as recommendation logic, not as a manual taxonomy test

**Decision**: Keep the existing schema set, but codify guidance such as:
- `spec-driven` for new capabilities, ambiguous feature work, and cross-cutting behavior changes
- `minimalist` for localized fixes and bounded platform/tooling/infrastructure changes
- `tdd` when the safest fix path is test-first regression work

**Rationale**: The main problem is not availability of schemas; it is underpowered guidance at the moment of proposal creation.

### 4. Start with harness command and skill assets

**Decision**: Model `ito-fix` and `ito-feature` first as workflow command/skill assets that feed into Ito proposal creation, rather than immediately expanding the Rust CLI surface.

**Rationale**: The behavior change is mostly in the workflow layer. Starting there keeps implementation smaller while preserving room for later CLI promotion if the UX proves valuable.

## Risks / Trade-offs

- **Risk: Entry-point confusion** -> Mitigation: define crisp roles for `ito-fix`, `ito-feature`, `ito-proposal`, and `ito-brainstorming` in the guidance and tests.
- **Risk: Duplicate questioning across skills** -> Mitigation: require intake handoff so downstream proposal creation consumes prior context instead of restarting discovery.
- **Trade-off: More front-door assets** -> Accepted because intent-biased entrypoints should reduce user uncertainty more than they increase surface area.
- **Risk: `minimalist` gets overused for high-impact fixes** -> Mitigation: schema guidance must account for blast radius and behavior change, not just whether the user called it a fix.

## Migration Plan

1. Define the new workflow capabilities in spec deltas.
2. Add the intake and intent-biased command/skill assets in embedded templates.
3. Update neutral proposal guidance to cooperate with the new entrypoints rather than duplicate them.
4. Add tests for schema recommendation copy, installed assets, and routing expectations.

## Open Questions

<<<<<<< HEAD
- None for the first implementation. The intake handoff is an in-session summary, and `tdd` may be recommended from any lane when regression-oriented work makes test-first execution the safest path.
=======
- Should the intake handoff be a dedicated persisted artifact, or an internal summary passed directly into proposal generation until a change exists?
- Should the first implementation recommend `tdd` only from `ito-fix`, or also from neutral intake when a regression-oriented request is detected?
>>>>>>> 2df3dfff (archive changes)
<!-- ITO:END -->
