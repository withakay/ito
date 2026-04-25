<!-- ITO:START -->
# Tasks for: 001-33_enhance-spec-driven-workflow-validation

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-33_enhance-spec-driven-workflow-validation
ito tasks next 001-33_enhance-spec-driven-workflow-validation
ito tasks start 001-33_enhance-spec-driven-workflow-validation 1.1
ito tasks complete 001-33_enhance-spec-driven-workflow-validation 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Update spec-driven artifact templates

- **Files**: `ito-rs/crates/ito-templates/assets/schemas/spec-driven/templates/proposal.md`, `ito-rs/crates/ito-templates/assets/schemas/spec-driven/templates/spec.md`, `ito-rs/crates/ito-templates/assets/schemas/spec-driven/templates/design.md`
- **Dependencies**: None
- **Action**: Add Change Shape, behavior metadata, contract refs, state/invariant guidance, and decision-focused design sections while preserving the spec-driven lifecycle.
- **Verify**: `cargo test -p ito-core --test templates_schema_resolution`
- **Done When**: New spec-driven changes render templates with the added sections and no mandatory empty optional sections.
- **Requirements**: ito-schemas:spec-driven-change-shape, ito-schemas:behavioral-requirement-metadata
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 1.2: Add composable validation facet configuration

- **Files**: `ito-rs/crates/ito-core/src/templates/types.rs`, `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/tests/validate.rs`
- **Dependencies**: None
- **Action**: Support multiple versioned validators per artifact while keeping existing single `validate_as` schema files valid.
- **Verify**: `cargo test -p ito-core --test validate validation_yaml_parses_composed_validators`
- **Done When**: Schema validation can apply multiple validators to one artifact and diagnostics retain validator IDs.
- **Requirements**: ito-schemas:composable-validation-facets
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 1.3: Align minimalist and event-driven spec templates

- **Files**: `ito-rs/crates/ito-templates/assets/schemas/minimalist/templates/specs/spec.md`, `ito-rs/crates/ito-templates/assets/schemas/event-driven/templates/specs/spec.md`, `ito-rs/crates/ito-core/tests/templates_schemas_listing.rs`, `ito-rs/crates/ito-cli/tests/templates_schemas_export.rs`
- **Dependencies**: None
- **Action**: Replace story-shaped spec templates with delta requirement templates that match `ito.delta-specs.v1`, and ensure exported schemas include validation assets.
- **Verify**: `cargo test -p ito-core --test templates_schemas_listing && cargo test -p ito-cli --test templates_schemas_export`
- **Done When**: Built-in minimalist and event-driven spec templates parse as delta specs and schema export includes validation configuration.
- **Requirements**: cli-templates-schemas:template-validator-alignment
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement scenario grammar validation

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/tests/validate.rs`
- **Dependencies**: None
- **Action**: Add validator logic for `WHEN`, `THEN`, optional `GIVEN`, UI-mechanics warnings, and excessive scenario length warnings.
- **Verify**: `cargo test -p ito-core --test validate scenario_grammar_validation`
- **Done When**: Validation errors on missing `WHEN` or `THEN` and warns on missing `GIVEN` or untagged UI-mechanics language.
- **Requirements**: cli-validate:scenario-grammar-validation
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.2: Implement proposal capability consistency validation

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/tests/validate.rs`
- **Dependencies**: None
- **Action**: Parse proposal capability lists and compare them to delta spec directories and baseline specs.
- **Verify**: `cargo test -p ito-core --test validate proposal_capabilities_consistency`
- **Done When**: Validation reports listed capabilities without deltas, deltas not listed in the proposal, and new/modified mismatches against baseline specs.
- **Requirements**: cli-validate:proposal-capabilities-consistency
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.3: Implement contract reference validation

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/tests/validate.rs`, `docs/schema-customization.md`
- **Dependencies**: None
- **Action**: Parse requirement Contract Refs, validate configured or conventional contract references, and warn when public contract facets have no requirement anchors.
- **Verify**: `cargo test -p ito-core --test validate contract_reference_validation`
- **Done When**: Missing referenced operations or schemas produce actionable diagnostics and public contract facets are tied back to requirements.
- **Requirements**: cli-validate:contract-reference-validation
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.4: Implement enhanced task quality validation

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/tests/validate.rs`, `ito-rs/crates/ito-domain/src/tasks.rs`
- **Dependencies**: None
- **Action**: Preserve quality-critical enhanced task metadata and validate missing status, missing done-when, unresolved requirements, and vague verification.
- **Verify**: `cargo test -p ito-core --test validate task_quality_validation`
- **Done When**: Enhanced tasks expose the fields needed for validation and produce expected diagnostics for missing or vague metadata.
- **Requirements**: cli-validate:task-quality-validation, tasks-tracking:quality-critical-fields, tasks-tracking:concrete-verification
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Update agent instructions and docs

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/*.md.j2`, `docs/schema-customization.md`, `.ito/user-prompts/proposal.md`
- **Dependencies**: None
- **Action**: Document Change Shape, behavior scenarios, contract refs, state/invariant sections, task quality expectations, and the non-primary status of executable BDD schemas.
- **Verify**: `make docs`
- **Done When**: Agent-facing instructions explain the new facets without requiring irrelevant optional sections for every change.
- **Requirements**: ito-schemas:spec-driven-change-shape, ito-schemas:behavioral-requirement-metadata, cli-validate:scenario-grammar-validation, cli-validate:contract-reference-validation
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 3.2: Run full validation gates

- **Files**: `ito-rs/crates/ito-templates/assets/schemas/`, `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/src/templates/types.rs`, `docs/schema-customization.md`
- **Dependencies**: None
- **Action**: Run the project validation and quality gates for the completed implementation.
- **Verify**: `ito validate 001-33_enhance-spec-driven-workflow-validation --strict && make check`
- **Done When**: Ito strict validation passes for the change and the repository quality gate passes.
- **Requirements**: ito-schemas:composable-validation-facets, cli-validate:scenario-grammar-validation, cli-validate:proposal-capabilities-consistency, cli-validate:contract-reference-validation, cli-validate:task-quality-validation, tasks-tracking:quality-critical-fields, tasks-tracking:concrete-verification, cli-templates-schemas:template-validator-alignment
- **Updated At**: 2026-04-25
- **Status**: [ ] pending
<!-- ITO:END -->
