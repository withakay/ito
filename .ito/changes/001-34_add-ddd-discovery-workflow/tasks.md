<!-- ITO:START -->
# Tasks for: 001-34_add-ddd-discovery-workflow

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 001-34_add-ddd-discovery-workflow
ito tasks next 001-34_add-ddd-discovery-workflow
ito tasks start 001-34_add-ddd-discovery-workflow 1.1
ito tasks complete 001-34_add-ddd-discovery-workflow 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define the DDD discovery bundle and handoff format

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-plan/SKILL.md`, `ito-rs/crates/ito-templates/assets/skills/ito-proposal-intake/SKILL.md`, `ito-rs/crates/ito-templates/assets/instructions/agent/new-proposal.md.j2`
- **Dependencies**: None
- **Action**: Add a consistent discovery grammar for ubiquitous language, bounded contexts, technique-fit triage, optional event storming, domain-grill questioning, evidence checks, commands, policies, aggregates, invariants, and proposal handoff summaries.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Planning and proposal-entry guidance can ask for and carry forward DDD discovery outputs, challenge fuzzy language, and consult repository evidence without forcing immediate proposal scaffolding.
- **Requirements**: `domain-discovery-workflow:ddd-discovery-bundle`, `domain-discovery-workflow:canonical-discovery-handoff`, `domain-discovery-workflow:domain-grill-interview-mode`, `domain-discovery-workflow:glossary-conflict-challenge`, `domain-discovery-workflow:scenario-boundary-probing`, `domain-discovery-workflow:code-documentation-cross-check`, `domain-discovery-workflow:ubiquitous-language-glossary`, `domain-discovery-workflow:bounded-context-map`, `domain-discovery-workflow:technique-fit-triage`, `domain-discovery-workflow:event-storming-technique-fit`, `domain-discovery-workflow:proposal-handoff-summary`, `workflow-convergence:domain-discovery-entrypoint`
- **Updated At**: 2026-04-30
- **Status**: [ ] pending

### Task 1.2: Add schema assets or template hooks for discovery artifacts

- **Files**: `ito-rs/crates/ito-templates/assets/schemas/spec-driven/`, `ito-rs/crates/ito-templates/assets/schemas/event-driven/`, `ito-rs/crates/ito-core/src/templates/`
- **Dependencies**: Task 1.1
- **Action**: Introduce reusable template support so selected discovery outputs and lazily discovered domain-doc locations can be referenced by both `spec-driven` and `event-driven` workflows without making every section mandatory.
- **Verify**: `cargo test -p ito-core templates && cargo test -p ito-cli instructions`
- **Done When**: Built-in schema guidance can surface discovery artifacts or discovery sections without duplicating conflicting grammars across schemas.
- **Requirements**: `ito-schemas:domain-discovery-artifacts`, `ito-schemas:canonical-discovery-summary-contract`, `ito-schemas:domain-documentation-location-discovery`, `ito-schemas:cross-schema-discovery-vocabulary`, `ito-schemas:discovery-artifact-optionality`
- **Updated At**: 2026-04-30
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add domain-language and documentation consistency validation

- **Files**: `ito-rs/crates/ito-core/src/validate/`, `ito-rs/crates/ito-core/tests/validate.rs`
- **Dependencies**: None
- **Action**: Add opt-in rules that compare canonical domain terms and proposed context/ADR updates against the canonical discovery handoff.
- **Verify**: `cargo test -p ito-core --test validate ubiquitous_language_consistency_rule domain_documentation_consistency_rule`
- **Done When**: Validation can warn on term drift, undefined aliases, glossary mismatches, or conflicting documentation updates without blocking simple changes by default.
- **Requirements**: `cli-validate:ubiquitous-language-consistency`, `cli-validate:domain-documentation-consistency`
- **Updated At**: 2026-04-30
- **Status**: [ ] pending

### Task 2.2: Add bounded-context consistency validation

- **Files**: `ito-rs/crates/ito-core/src/validate/`, `ito-rs/crates/ito-core/tests/validate.rs`, `ito-rs/crates/ito-core/src/change_repository.rs`
- **Dependencies**: Task 2.1
- **Action**: Add an opt-in rule that flags cross-context proposals that do not name affected contexts, relationships, or justification.
- **Verify**: `cargo test -p ito-core --test validate context_boundary_consistency_rule`
- **Done When**: Cross-context changes can be reviewed for missing context ownership or relationship framing.
- **Requirements**: `cli-validate:context-boundary-consistency`, `domain-discovery-workflow:context-map-distinguishes-module-and-capability`
- **Updated At**: 2026-04-30
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Update review and workflow documentation

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/review.md.j2`, `docs/agent-workflow.md`, `docs/presentations/march-2026/ito-workflow-diagram.mmd`
- **Dependencies**: None
- **Action**: Teach review instructions and human-facing workflow docs to look for discovery outputs, language drift, evidence checks, proposed context/ADR updates, technique-fit decisions, and cross-context ambiguity before implementation begins.
- **Verify**: `make docs`
- **Done When**: Review guidance and workflow docs explain when to invoke DDD discovery and how its outputs feed proposal quality.
- **Requirements**: `domain-discovery-workflow:canonical-discovery-handoff`, `domain-discovery-workflow:lazy-domain-documentation-capture`, `workflow-convergence:domain-discovery-entrypoint`, `workflow-convergence:domain-discovery-review-gate`, `workflow-convergence:domain-docs-change-scope`
- **Updated At**: 2026-04-30
- **Status**: [ ] pending

### Task 3.2: Final validation and change-package gate

- **Files**: `.ito/changes/001-34_add-ddd-discovery-workflow/`, affected schema, validation, and documentation files
- **Dependencies**: Task 3.1
- **Action**: Run change validation and targeted tests for instruction rendering, schema loading, and validation rules.
- **Verify**: `ito validate 001-34_add-ddd-discovery-workflow --strict && cargo test -p ito-core --test validate && cargo test -p ito-cli instructions`
- **Done When**: The change validates strictly and the targeted workflow tests pass.
- **Requirements**: `domain-discovery-workflow:ddd-discovery-bundle`, `domain-discovery-workflow:canonical-discovery-handoff`, `domain-discovery-workflow:domain-grill-interview-mode`, `domain-discovery-workflow:glossary-conflict-challenge`, `domain-discovery-workflow:scenario-boundary-probing`, `domain-discovery-workflow:code-documentation-cross-check`, `domain-discovery-workflow:ubiquitous-language-glossary`, `domain-discovery-workflow:bounded-context-map`, `domain-discovery-workflow:technique-fit-triage`, `domain-discovery-workflow:event-storming-technique-fit`, `domain-discovery-workflow:proposal-handoff-summary`, `domain-discovery-workflow:context-map-distinguishes-module-and-capability`, `domain-discovery-workflow:lazy-domain-documentation-capture`, `workflow-convergence:domain-discovery-entrypoint`, `workflow-convergence:domain-discovery-review-gate`, `workflow-convergence:domain-docs-change-scope`, `ito-schemas:domain-discovery-artifacts`, `ito-schemas:canonical-discovery-summary-contract`, `ito-schemas:domain-documentation-location-discovery`, `ito-schemas:cross-schema-discovery-vocabulary`, `ito-schemas:discovery-artifact-optionality`, `cli-validate:ubiquitous-language-consistency`, `cli-validate:context-boundary-consistency`, `cli-validate:domain-documentation-consistency`
- **Updated At**: 2026-04-30
- **Status**: [ ] pending
<!-- ITO:END -->
