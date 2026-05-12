<!-- ITO:START -->
## Why

Ito's default spec-driven workflow already gives LLMs useful anchors, but its templates and validation leave too much room for underspecified scenarios, proposal/spec drift, vague task verification, and contract details copied inline instead of referenced. Tightening the existing workflow preserves Ito's lightweight shape while making behavior, contracts, and validation more explicit, without forcing every change to adopt every facet.

## What Changes

- Keep `spec-driven` as the primary workflow and make it more behavior-aware, contract-aware, and validation-aware.
- Add an opt-in **Change Shape** metadata block to spec-driven proposals (Type, Risk, Stateful, Public Contract, Design Needed, Design Reason) with a defined allowed-value vocabulary.
- Add **scenario grammar validation** for delta requirements: WHEN/THEN required (error), GIVEN recommended (warning), an explicit excessive-step threshold of 8 steps (warning), and a conservative UI-mechanics warning gated on multi-token patterns and an explicit `ui` tag.
- Add **lightweight Contract Refs** (`openapi:<operation>`, `jsonschema:<name>`, `asyncapi:<channel>`, `cli:<command>`, `config:<key>`) on requirements instead of inline OpenAPI / JSON Schema / AsyncAPI documents. v1 validates syntax only; resolution against contract files is deferred behind a documented configuration point.
- Add **proposal-capabilities ↔ delta-spec consistency** validation with a defined parsing grammar for the proposal `## Capabilities` section.
- Add **enhanced task quality validation**: missing Status / Done When / Verify are errors; vague Verify and missing Files/Action are warnings; unresolved Requirement IDs are errors.
- Replace the proposed "composable validation facets" with a backward-compatible **`rules:` extension** to existing validators (no new validator IDs in v1). Rules are opt-in per artifact in `validation.yaml`.
- Built-in `spec-driven` `validation.yaml` does NOT enable the new rules by default in v1; they are opt-in via project-local schema overrides only. Change Shape remains purely advisory and never enables rules implicitly. This protects in-flight changes from sudden new diagnostics.
- Expand the spec-driven design template toward decisions, interfaces, state, invariants, verification, and migration with an explicit anti-overprescription rule (no full code examples).
- Add optional state-transition tables and rules/invariants sections for stateful changes; prefer tables over decorative Mermaid.
- Align built-in **minimalist** and **event-driven** spec templates with their already-configured `ito.delta-specs.v1` validator (current templates use `## Stories` / `### Story:` and silently fail to parse as deltas).
- Defer an executable BDD `.feature` schema; not part of the primary path.

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `ito-schemas`: support a richer spec-driven proposal/spec/design template set, define Change Shape allowed values, and add an opt-in `rules:` extension to per-artifact `validation.yaml` entries.
- `cli-validate`: add scenario grammar, proposal-capability consistency, contract-reference syntax, and enhanced task-quality checks behind opt-in rules.
- `tasks-tracking`: define which enhanced-task fields are required, optional, or warning-only when missing; define vague-verification denylist semantics.
- `cli-templates-schemas`: keep built-in minimalist and event-driven spec templates compatible with their declared validators, and ensure `validation.yaml` is included in exported schemas.

## Impact

- Schema assets under `ito-rs/crates/ito-templates/assets/schemas/` for `spec-driven`, `minimalist`, and `event-driven`.
- Validation parsing and diagnostics in `ito-rs/crates/ito-core/src/validate/` and the schema validation types in `ito-rs/crates/ito-core/src/templates/types.rs`.
- Enhanced-task field parsing in `ito-rs/crates/ito-domain/src/tasks/` (directory; not a single `tasks.rs` file).
- Agent-facing instruction output from `ito agent instruction proposal|specs|design|tasks`.
- Tests covering rule extension parsing, scenario grammar, proposal-capability consistency, contract-ref syntax, task quality, and built-in schema export parity.
- No changes required for in-flight changes that do not opt into the new rules.
<!-- ITO:END -->
