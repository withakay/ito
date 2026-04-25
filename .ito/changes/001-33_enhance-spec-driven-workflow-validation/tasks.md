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
- **Action**: Add the optional Change Shape block to the proposal template; add optional Tags / Contract Refs / Rules / Invariants / State Transitions sections to the requirement template; expand the design template toward decisions, interfaces, state, invariants, verification, migration, and rollback with an explicit anti-overprescription note.
- **Verify**: `cargo test -p ito-core --test templates_schema_resolution`
- **Done When**: Built-in spec-driven templates render with the new optional sections, no mandatory empty sections, and existing renders still parse as deltas.
- **Requirements**: ito-schemas:spec-driven-change-shape, ito-schemas:behavioral-requirement-metadata
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 1.2: Align minimalist and event-driven spec templates with their validators

- **Files**: `ito-rs/crates/ito-templates/assets/schemas/minimalist/templates/specs/spec.md`, `ito-rs/crates/ito-templates/assets/schemas/event-driven/templates/specs/spec.md`, `ito-rs/crates/ito-core/tests/templates_schemas_listing.rs`, `ito-rs/crates/ito-cli/tests/templates_schemas_export.rs`
- **Dependencies**: None
- **Action**: Replace `## Stories` / `### Story:` shapes with `## ADDED Requirements` / `### Requirement:` / `#### Scenario:` so the templates parse as `ito.delta-specs.v1`. Add export tests that include `validation.yaml`.
- **Verify**: `cargo test -p ito-core --test templates_schemas_listing && cargo test -p ito-cli --test templates_schemas_export`
- **Done When**: Built-in minimalist and event-driven spec templates parse as deltas; schema export includes `validation.yaml` for every built-in schema.
- **Requirements**: cli-templates-schemas:template-validator-alignment, cli-templates-schemas:export-validation-assets
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 1.3: Extend `validation.yaml` parsing with `rules:` and `proposal:`

- **Files**: `ito-rs/crates/ito-core/src/templates/types.rs`, `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/tests/validate.rs`
- **Dependencies**: None
- **Action**: Add an optional `rules: BTreeMap<String, ValidationLevelYaml>` field to `ValidationArtifactYaml` and `ValidationTrackingYaml`; introduce a `proposal:` artifact entry; add a `rule_id: Option<String>` field to validation diagnostics. Single `validate_as` schemas must remain valid. Unknown rule names produce a configuration warning but do not abort.
- **Verify**: `cargo test -p ito-core --test validate validation_yaml_rules_extension && cargo test -p ito-core --test validate validation_yaml_proposal_entry`
- **Done When**: `validation.yaml` accepts `rules:` per artifact and a `proposal:` entry; existing schemas continue to parse; diagnostics expose `rule_id` when produced by a rule.
- **Requirements**: ito-schemas:validation-rules-extension, ito-schemas:opt-in-rules-default
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 1.4: Parse enhanced-task quality fields

- **Files**: `ito-rs/crates/ito-domain/src/tasks/`, `ito-rs/crates/ito-core/src/task_repository.rs`
- **Dependencies**: None
- **Action**: Ensure the parsed enhanced task exposes `Files`, `Dependencies`, `Action`, `Verify`, `Done When`, `Requirements`, `Status`, `Updated At` as separate structured fields. Add unit tests covering present-vs-absent variants.
- **Verify**: `cargo test -p ito-domain tasks::enhanced::quality_fields`
- **Done When**: Enhanced-task parsing exposes the full quality-critical field set without altering existing behavior.
- **Requirements**: tasks-tracking:quality-critical-fields
- **Updated At**: 2026-04-25
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement scenario_grammar rule

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/tests/validate.rs`
- **Dependencies**: None
- **Action**: Add the `scenario_grammar` rule to `ito.delta-specs.v1`: WHEN/THEN required (severity from rule), GIVEN warning, excessive-step warning at threshold 8, conservative UI-mechanics warning gated on the `ui` tag and the canonical regex set in the spec.
- **Verify**: `cargo test -p ito-core --test validate scenario_grammar_rule`
- **Done When**: Diagnostics fire for each canonical case; markdown anchors and CSS-shaped tokens like `.unwrap` from code phrases do not trigger UI-mechanics warnings; rule is silent unless enabled in `validation.yaml`.
- **Requirements**: cli-validate:scenario-grammar-validation
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.2: Implement capabilities_consistency rule

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/src/change_repository.rs`, `ito-rs/crates/ito-core/tests/validate.rs`
- **Dependencies**: None
- **Action**: Parse `## Capabilities` per the grammar in the spec; compare against change-local `specs/<name>/` directories and baseline `.ito/specs/<name>/`. Emit errors for missing deltas, unlisted deltas, and new-vs-modified mismatches against baseline. Emit a parser-level warning when a bullet has no inline-code token.
- **Verify**: `cargo test -p ito-core --test validate capabilities_consistency_rule`
- **Done When**: All scenarios in `cli-validate:proposal-capabilities-consistency` are exercised; parsing routes through repository abstractions, not direct filesystem reads.
- **Requirements**: cli-validate:proposal-capabilities-consistency
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.3: Implement contract_refs rule (syntax-only v1)

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/tests/validate.rs`, `docs/schema-customization.md`
- **Dependencies**: None
- **Action**: Parse `Contract Refs` metadata; reject unknown schemes; emit one INFO advisory per change when refs exist without configured discovery; emit one warning per Public Contract facet declared in Change Shape that has no corresponding requirement reference. Do NOT attempt to resolve references against external contract files in v1.
- **Verify**: `cargo test -p ito-core --test validate contract_refs_rule`
- **Done When**: Syntax errors fire on unknown schemes; advisory and Public-Contract-anchor scenarios fire as specified; resolution is explicitly out of scope.
- **Requirements**: cli-validate:contract-reference-validation
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.4: Implement task_quality rule

- **Files**: `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/tests/validate.rs`
- **Dependencies**: None
- **Action**: Add the `task_quality` rule to `ito.tasks-tracking.v1` with the canonical severity table from the cli-validate spec, including the vague-verification denylist (case-insensitive exact match) and the implementation-task heuristic based on file extension.
- **Verify**: `cargo test -p ito-core --test validate task_quality_rule`
- **Done When**: Each row of the severity table is exercised; non-implementation tasks downgrade missing-Verify to warning.
- **Requirements**: cli-validate:task-quality-validation, tasks-tracking:concrete-verification
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

### Task 2.5: Update agent-facing docs and instructions

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/*.md.j2`, `docs/schema-customization.md`, `.ito/user-prompts/proposal.md`
- **Dependencies**: None
- **Action**: Document Change Shape, requirement metadata, the `rules:` extension, the opt-in default policy, and how to export and enable rules locally. Note that BDD `.feature` is intentionally not the primary path.
- **Verify**: `make docs`
- **Done When**: Agent instruction artifacts mention the new template sections only when relevant facets are declared, and the schema-customization docs cover the rules extension.
- **Requirements**: ito-schemas:spec-driven-change-shape, ito-schemas:behavioral-requirement-metadata, ito-schemas:validation-rules-extension, ito-schemas:opt-in-rules-default
- **Updated At**: 2026-04-25
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Final validation and quality gate

- **Files**: `ito-rs/crates/ito-templates/assets/schemas/`, `ito-rs/crates/ito-core/src/validate/mod.rs`, `ito-rs/crates/ito-core/src/templates/types.rs`, `docs/schema-customization.md`
- **Dependencies**: None
- **Action**: Run `ito validate 001-33_enhance-spec-driven-workflow-validation --strict`, `make check`, and `cargo test --workspace`. Capture any failure output and fix.
- **Verify**: `ito validate 001-33_enhance-spec-driven-workflow-validation --strict && make check`
- **Done When**: Strict validation passes for the change and the repository quality gate passes. This task does NOT carry traceability for new requirements; it is a gate.
- **Requirements**:
- **Updated At**: 2026-04-25
- **Status**: [ ] pending
<!-- ITO:END -->
