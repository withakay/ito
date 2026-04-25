<!-- ITO:START -->
## Why

Ito's default spec-driven workflow already gives LLMs useful anchors, but its templates and validation leave too much room for underspecified scenarios, proposal/spec drift, vague task verification, and contract details copied inline instead of referenced. Tightening the existing workflow preserves Ito's lightweight shape while making behavior, contracts, state, and validation more explicit.

## What Changes

- Keep `spec-driven` as the primary workflow and make it more behavior-aware, contract-aware, and validation-aware.
- Add Change Shape metadata to spec-driven proposals so Ito and agents can select only the relevant rigor for a change.
- Add scenario grammar validation for delta specs, requiring `WHEN` and `THEN`, warning on missing `GIVEN`, excessive step counts, and UI-mechanics language unless tagged `ui`.
- Add lightweight requirement metadata for tags and contract references instead of embedding large OpenAPI, JSON Schema, AsyncAPI, CLI, or config contracts inline.
- Add proposal capabilities to spec delta consistency validation so proposal scope and delta files cannot silently diverge.
- Expand spec-driven design guidance toward decisions, interfaces, state, invariants, verification, and migration while explicitly discouraging code-level overprescription.
- Add optional state/invariant sections to spec guidance for stateful changes, preferring tables and text over decorative Mermaid diagrams.
- Add enhanced task quality validation for status, requirement references, concrete verification commands, and done-when criteria.
- Align built-in minimalist and event-driven spec templates with their configured delta-spec validators instead of using story-shaped templates that do not parse as deltas.
- Leave an executable BDD schema out of the primary path; it can be added later only for teams that specifically need `.feature` files.

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `ito-schemas`: support richer built-in spec-driven templates, Change Shape metadata, contract references, state/invariant guidance, and composable validation facets.
- `cli-validate`: validate scenario grammar, proposal/spec capability consistency, contract references, and task quality.
- `tasks-tracking`: require higher-quality enhanced task metadata when traceability and validation are active.
- `cli-templates-schemas`: keep exported built-in schema templates consistent with their validation configuration.

## Impact

- Affected schema assets under `ito-rs/crates/ito-templates/assets/schemas/` for spec-driven, minimalist, and event-driven workflows.
- Validation parsing and diagnostics in `ito-rs/crates/ito-core/src/validate/` and related schema validation types.
- Proposal, spec, design, and task instruction output from `ito agent instruction ...`.
- Tests covering validator identifiers, delta parsing, proposal/spec consistency, task quality, and built-in schema exports.
<!-- ITO:END -->
