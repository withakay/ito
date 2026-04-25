<!-- ITO:START -->
## Context

Ito already has the right change-management shape for LLM work: small proposals, capability-scoped deltas, optional design, traceable tasks, validation, and archive into long-term specs. The gap is not a missing primary BDD workflow; it is that the existing spec-driven workflow needs stronger artifact semantics and validators so generated proposals, specs, and tasks remain grounded.

## Goals / Non-Goals

**Goals:**

- Preserve `spec-driven` as Ito's default path.
- Add behavior, contract, state, and validation semantics without forcing every change to include every section.
- Prefer external contract references over inline copies of large contracts.
- Improve validator feedback for common LLM failure modes: weak scenarios, proposal/spec drift, vague tasks, and schema/template mismatch.
- Keep design docs decision-focused and avoid code-level overprescription.

**Non-Goals:**

- Do not make a BDD or Gherkin schema the primary workflow.
- Do not require executable `.feature` files for normal Ito changes.
- Do not inline large OpenAPI, JSON Schema, or AsyncAPI documents in specs.
- Do not introduce formal-methods tooling such as TLA+ or Alloy.

## Approach

Update the built-in spec-driven templates first, then add validators behind versioned validator identifiers. The proposal template gets Change Shape metadata. The spec template keeps delta requirements but adds optional metadata and guidance for Tags, Contract Refs, Rules / Invariants, and State Transitions. The design template expands around decisions, interfaces, data/state, verification, migration, and rollback with an explicit instruction to avoid implementation-level code examples unless they clarify an interface, migration shape, or non-obvious algorithm.

Validation should remain schema-driven. Existing `validate_as: ito.delta-specs.v1` continues to work. New validation facets should be opt-in through schema validation configuration, with diagnostics carrying their validator identifier so users can tell whether an issue came from delta parsing, scenario grammar, contract refs, proposal consistency, or task quality.

## Contracts / Interfaces

- `validation.yaml` needs a backward-compatible way to enable more than one validator for an artifact.
- Requirement metadata supports lightweight values such as `openapi:POST /path`, `jsonschema:Name`, `asyncapi:channel`, `cli:command`, and `config:key`.
- Contract lookup should use configured or conventional contract files, but missing contract configuration should produce actionable warnings before it becomes a hard blocker.

## Data / State

No persisted user data migration is expected. The important state is artifact state:

| Artifact | New State | Effect |
|---|---|---|
| proposal.md | Change Shape present | Guides optional validation facets and artifact guidance |
| specs/*/spec.md | Tags and Contract Refs present | Anchors behavior to external contracts without inline bloat |
| specs/*/spec.md | Rules / Invariants present | Guides tests for stateful behavior |
| tasks.md | Enhanced metadata complete | Enables stricter task quality validation |

## Decisions

### Decision: Strengthen spec-driven instead of making BDD primary

- **Chosen**: Keep `spec-driven` as the main workflow and make scenarios more disciplined.
- **Alternatives Considered**: Add a standalone BDD schema as the recommended default.
- **Rationale**: BDD is a scenario style, not a full change-management model. Ito already has proposals, deltas, tasks, validation, and archive semantics.
- **Consequences**: Teams get most BDD benefits without adding a second source of truth. A future BDD schema remains possible for executable `.feature` workflows.

### Decision: Use references for contracts

- **Chosen**: Store lightweight Contract Refs in requirements.
- **Alternatives Considered**: Embed full OpenAPI or JSON Schema snippets in every spec.
- **Rationale**: References reduce context use and drift while giving agents precise anchors.
- **Consequences**: Validators need contract lookup logic and clear diagnostics when references cannot be resolved.

### Decision: Prefer state tables over required Mermaid

- **Chosen**: Use textual invariants and transition tables as the primary model for stateful changes.
- **Alternatives Considered**: Require Mermaid diagrams for stateful or event-driven changes.
- **Rationale**: Tables are easier for LLMs to maintain and validate; Mermaid is useful but should be optional.
- **Consequences**: Mermaid can remain optional and derived from tables when useful for humans.

## Risks / Trade-offs

- More validators could make proposal creation feel heavier -> gate optional checks by Change Shape and schema facets.
- Contract lookup could become repo-specific too quickly -> start with reference parsing and clear configuration points before deep ecosystem integration.
- Task quality checks could overfit wording -> enforce only objective fields as errors and keep style concerns as warnings.
- Built-in template changes can affect existing habits -> keep compatibility for existing change artifacts and only make new templates stricter.

## Verification Strategy

- Add unit tests for parsing composed validator configuration while preserving single `validate_as` behavior.
- Add validation tests for scenario grammar, missing proposal capability deltas, unlisted delta capabilities, missing contract refs, and vague task verification.
- Add template export tests that verify `validation.yaml` is exported and minimalist/event-driven templates parse as delta specs.
- Run `make test` for implementation work and `make check` before completion.

## Migration / Rollback

Existing changes should continue to validate under their current schema behavior unless they opt into new facets or are regenerated from updated built-in templates. Rollback is template and validator removal or disabling the new facet identifiers in built-in `validation.yaml`.

## Open Questions

- Which contract file discovery conventions should be supported first for OpenAPI, JSON Schema, AsyncAPI, CLI, and config references?
- Should Change Shape drive validation automatically, or should it only guide agents while `validation.yaml` remains the single source of enabled validators?
<!-- ITO:END -->
