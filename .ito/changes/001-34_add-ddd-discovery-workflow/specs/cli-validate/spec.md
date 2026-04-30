<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ubiquitous language consistency rule

When the `ubiquitous_language_consistency` rule is enabled, validation SHALL compare canonical domain terms from discovery outputs against proposal, spec, and task language and report drift.

- **Requirement ID**: `cli-validate:ubiquitous-language-consistency`

#### Scenario: Undefined alias is warned

- **GIVEN** discovery outputs define `workspace` as the canonical term
- **AND** a proposal or spec later uses `project space` as if it were a different concept
- **WHEN** validation runs with the rule enabled
- **THEN** validation reports a warning naming the canonical term and the drifting alias

#### Scenario: Consistent terminology passes

- **GIVEN** proposal, specs, and tasks all use the canonical discovery vocabulary
- **WHEN** validation runs with the rule enabled
- **THEN** validation emits no terminology-drift warning

### Requirement: Context boundary consistency rule

When the `context_boundary_consistency` rule is enabled, validation SHALL warn when a proposal spans multiple bounded contexts without naming the affected contexts or describing their relationship.

- **Requirement ID**: `cli-validate:context-boundary-consistency`

#### Scenario: Cross-context proposal without context framing warns

- **GIVEN** a proposal changes behavior in more than one bounded context
- **AND** the proposal or discovery handoff does not name those contexts or their relationship
- **WHEN** validation runs with the rule enabled
- **THEN** validation reports a warning that boundary framing is incomplete

#### Scenario: Cross-context proposal with explicit relationship passes

- **GIVEN** a proposal names the affected bounded contexts and their relationship
- **WHEN** validation runs with the rule enabled
- **THEN** validation does not emit a boundary-consistency warning for that relationship
<!-- ITO:END -->
