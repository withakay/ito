<!-- ITO:START -->
## Context

Ito already has the right change-management shape for LLM-assisted work: small proposals, capability-scoped delta specs, optional design, traceable tasks, validation, archive into long-term specs. The gap is not a missing primary BDD workflow; it is that the existing spec-driven workflow needs stronger artifact semantics and tighter validators so generated proposals, specs, and tasks remain grounded.

Two concrete defects motivate this change:

1. The built-in `minimalist` and `event-driven` spec templates use `## Stories` / `### Story:` markup but their `validation.yaml` declares `validate_as: ito.delta-specs.v1`. The delta validator silently ignores story-shaped content; that mismatch was discovered while reviewing the prior reviewer's feedback.
2. The default `#### Scenario:` validator only checks "non-empty" and never inspects the WHEN / THEN structure that the proposal template asks agents to follow.

## Goals / Non-Goals

**Goals:**

- Preserve `spec-driven` as the default workflow.
- Add behavior, contract, and validation semantics without forcing every change to use every section.
- Prefer external contract references over inline copies of large contracts.
- Improve validator feedback for common LLM failure modes: weak scenarios, proposal/spec drift, vague tasks, and template/validator mismatch.
- Keep design docs decision-focused and explicitly discourage code-level overprescription.
- Stay backward compatible with existing in-flight changes.

**Non-Goals:**

- Do not make a BDD or Gherkin schema the primary workflow.
- Do not require executable `.feature` files for normal Ito changes.
- Do not inline large OpenAPI / JSON Schema / AsyncAPI documents in specs.
- Do not introduce formal-methods tooling such as TLA+ or Alloy.
- Do not introduce a new validator id (`ito.delta-specs.v2`, `ito.scenario-grammar.v1`, etc.) in v1. New checks live as opt-in `rules:` inside existing validators.

## Approach

Three layers, each backward compatible:

1. **Templates first.** Update the spec-driven proposal/spec/design templates to expose Change Shape, requirement-level Tags / Contract Refs / Rules / State Transitions, and the decision-focused design sections. Align minimalist and event-driven spec templates with `ito.delta-specs.v1`.
2. **Validation extension second.** Extend `validation.yaml` parsing to accept an optional `rules:` map under each artifact entry and a new `proposal:` artifact entry. Each rule name resolves to a check inside an existing validator. Single `validate_as` schemas keep working.
3. **Built-in defaults stay quiet in v1.** The shipped `spec-driven/validation.yaml` does NOT enable any new rules by default. Teams opt in via `ito templates schemas export` and editing the project-local copy. This protects in-flight changes from sudden new diagnostics on upgrade.

## Contracts / Interfaces

`validation.yaml` schema extension (additive):

```yaml
version: 1
artifacts:
  specs:
    required: true
    validate_as: ito.delta-specs.v1
    rules:                       # NEW (optional)
      scenario_grammar: error
      ui_mechanics: warn
      contract_refs: warn
proposal:                        # NEW (optional)
  validate_as: ito.delta-specs.v1
  rules:
    capabilities_consistency: error
tracking:
  source: apply_tracks
  required: true
  validate_as: ito.tasks-tracking.v1
  rules:                         # NEW (optional)
    task_quality: error
```

Diagnostic envelope: every issue produced by a rule includes both `validator_id` (existing) and a new `rule_id` field, so a user can trace which rule fired.

Requirement-level metadata grammar:

- `- **Tags**: behavior, ui` — comma-separated.
- `- **Contract Refs**: openapi:POST /v1/x, jsonschema:Foo` — comma-separated `scheme:identifier`. Supported schemes: `{openapi, jsonschema, asyncapi, cli, config}`.
- `#### Rules / Invariants` and `#### State Transitions` — optional sub-sections.

Proposal capability parsing grammar (for `capabilities_consistency`):

- Look for `## Capabilities` then `### New Capabilities` and `### Modified Capabilities`.
- In each subsection, treat each markdown bullet as a candidate. The capability name is the first inline-code token (`` `<name>` ``).
- Skip placeholders (`<name>`, `<existing-name>`), HTML comments, and empty bullets.
- Match exactly against directory names under `specs/<name>/` (change-local) and `.ito/specs/<name>/` (baseline). Sub-module routing is out of scope for v1.

Contract reference resolution (v1):

- Parse syntax only. No file lookup.
- If at least one requirement carries a `Contract Refs` value and no contract-discovery configuration is present, emit one INFO diagnostic per change explaining how to enable resolution later.
- Future work: define `.ito/contracts.yaml` (or similar) discovery; add an `ito.contracts.v1` validator. Tracked as Open Question (resolved below).

## Data / State

Artifact-level state changes only:

| Artifact | New State | Effect |
|---|---|---|
| proposal.md | Optional Change Shape block | Advisory metadata; can be used to suggest enabling rules |
| specs/*/spec.md | Optional Tags, Contract Refs, Rules / Invariants, State Transitions | Anchors behavior to external contracts and stateful rules |
| validation.yaml | Optional `rules:` map per artifact, new `proposal:` entry | Opt-in checks |
| tasks.md | Same enhanced-task fields, parsed structurally | Enables `task_quality` rule |

## Decisions

### Decision: Strengthen spec-driven instead of making BDD primary

- **Chosen**: Keep `spec-driven` as the main workflow and tighten scenario discipline.
- **Alternatives Considered**: Add a standalone BDD schema as the default.
- **Rationale**: BDD is a scenario style, not a change-management model. Ito already has proposals, deltas, tasks, validation, archive.
- **Consequences**: Most BDD benefits without a second source of truth. A future BDD schema can land for teams that need executable `.feature`.

### Decision: Use `rules:` extension instead of composable facets

- **Chosen**: Extend `validation.yaml` with an optional `rules:` map per artifact entry. New checks run inside existing validators.
- **Alternatives Considered**: Allow a list-shaped `validate_as` (e.g. `validate_as: [a, b]`); introduce new validator IDs (`ito.scenario-grammar.v1`).
- **Rationale**: Avoids the "list of validator IDs" YAML-shape ambiguity flagged by both reviewers. Keeps validator IDs stable. Backward compatible.
- **Consequences**: Rule semantics live with the validator. Diagnostics carry both `validator_id` and `rule_id`. New validator IDs become a v2 conversation, not a v1 commitment.

### Decision: Lightweight Contract Refs

- **Chosen**: Store typed scheme:identifier pairs in requirements and validate syntax only in v1.
- **Alternatives Considered**: Embed full OpenAPI snippets; require a contract-discovery config up front.
- **Rationale**: References reduce context and drift. Discovery can land later without re-cutting the requirement format.
- **Consequences**: A future change adds discovery and resolution; v1 emits a single advisory diagnostic when refs exist but discovery is unconfigured.

### Decision: Prefer state tables over required Mermaid

- **Chosen**: Use textual rules and transition tables as the primary stateful model. Mermaid is allowed but never required.
- **Alternatives Considered**: Require Mermaid for stateful changes.
- **Rationale**: Tables are easier for LLMs to maintain and validate.
- **Consequences**: Mermaid is documentation, not a validation surface.

### Decision: Built-in defaults are quiet

- **Chosen**: Ship the rule machinery enabled only when project-local schemas opt in.
- **Alternatives Considered**: Enable scenario_grammar in built-in spec-driven by default.
- **Rationale**: Avoids breaking in-flight changes during upgrade. Teams adopt rules deliberately.
- **Consequences**: Adoption signaling is a docs problem; the agent-instruction artifact in Wave 2 covers it.

## Risks / Trade-offs

- **Validator/rule overlap.** A scenario could trigger both "must have at least one scenario" (existing delta validator) and a scenario_grammar rule. → Rules run after the existing validator's structural checks; rules skip scenarios that already failed structurally.
- **Heuristic false positives.** UI-mechanics and vague-verification heuristics risk noise. → Both ship at warning severity, both use conservative explicit pattern sets defined in the spec, both can be disabled by removing the rule from project-local `validation.yaml`.
- **Backward compat for in-flight changes.** → Built-in spec-driven `validation.yaml` does not enable new rules. Existing changes keep validating as before.
- **Capability-name parsing edge cases.** Legacy proposals may use bullets without backtick-wrapped names. → Parser ignores bullets without an inline-code token and emits a diagnostic only when at least one capability appears in the proposal.
- **Schema export drift.** If `validation.yaml` is missed on export, project-local overrides won't have the rule machinery. → Add an export test that fails when `validation.yaml` is absent from any built-in schema export.

## Verification Strategy

- Unit tests for `validation.yaml` parsing of the `rules:` map, including unknown rules, missing rules, and the new `proposal:` artifact entry.
- Validator tests for each rule: scenario_grammar (WHEN/THEN/GIVEN/excessive-step/UI-mechanics), capabilities_consistency (parse grammar + missing/unlisted/baseline mismatch), contract_refs (syntax + no-discovery advisory), task_quality (severity table).
- Template parity tests: render the built-in minimalist and event-driven spec templates into a synthetic change and assert `ito validate --strict` succeeds.
- Export tests: `ito templates schemas export` writes `validation.yaml` for every built-in schema; round-trip determinism is preserved.
- `make check` is the pre-merge gate. Coverage hard floor 80%; target 100%.

## Migration / Rollback

- Existing changes remain valid because no new rule is enabled in built-in `spec-driven/validation.yaml`.
- Rollback path: revert template changes, remove rule code paths, drop the `rules:` field. Backward-compatible omission keeps the configuration shape valid.

## Open Questions (resolved)

- **Contract discovery convention.** v1 does NOT define a discovery file. v1 parses ref syntax and emits a single INFO advisory per change when refs exist without configured discovery. A follow-up change introduces `.ito/contracts.yaml` (or similar) and an `ito.contracts.v1` validator.
- **Should Change Shape drive validation automatically?** No. Change Shape is advisory only in v1. `validation.yaml` remains the single source of enabled rules. A future change may use Change Shape as a hint to suggest rule activation, but never as an implicit gate.
<!-- ITO:END -->
