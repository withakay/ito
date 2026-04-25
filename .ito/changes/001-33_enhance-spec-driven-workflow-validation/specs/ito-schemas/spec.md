<!-- ITO:START -->
## ADDED Requirements

These requirements extend workflow schema semantics and built-in spec-driven artifacts without replacing the existing proposal to specs to design to tasks lifecycle.

### Requirement: Spec-driven proposal change shape

The spec-driven proposal template SHALL include a compact Change Shape section that records Type, Risk, Stateful, Public Contract, Design Needed, and Design Reason metadata.

- **Requirement ID**: ito-schemas:spec-driven-change-shape

#### Scenario: Proposal declares workflow facets

- **GIVEN** an agent creates a spec-driven change proposal
- **WHEN** the proposal template is rendered
- **THEN** the proposal includes a Change Shape section with Type, Risk, Stateful, Public Contract, Design Needed, and Design Reason fields
- **AND** the fields are concise metadata rather than long-form implementation instructions

#### Scenario: Facets keep optional sections targeted

- **GIVEN** a proposal declares `Stateful: no` and `Public Contract: none`
- **WHEN** an agent writes specs and design artifacts
- **THEN** Ito guidance does not require state transition tables or contract sections solely because the template supports them

### Requirement: Spec-driven requirements support behavioral metadata

The spec-driven spec template SHALL support Tags, Contract Refs, Rules / Invariants, and State Transitions as optional requirement-level sections.

- **Requirement ID**: ito-schemas:behavioral-requirement-metadata

#### Scenario: Requirement references external contracts

- **GIVEN** a requirement affects an HTTP API and JSON payload
- **WHEN** the requirement is written
- **THEN** the requirement can reference `openapi:<operation>` and `jsonschema:<schema>` identifiers without copying full contract documents inline

#### Scenario: Stateful requirement records invariants

- **GIVEN** a requirement governs stateful behavior
- **WHEN** the requirement is written
- **THEN** the requirement can include Rules / Invariants and a State Transitions table that describe states, events, guards, next states, and effects

### Requirement: Validation configuration can compose facets

Workflow schema validation configuration MUST allow an artifact to enable multiple versioned validation facets without creating duplicate artifact declarations.

- **Requirement ID**: ito-schemas:composable-validation-facets

#### Scenario: Spec artifact enables multiple validators

- **GIVEN** a schema wants delta parsing, scenario grammar checks, and contract reference checks for `specs`
- **WHEN** Ito loads that schema's validation configuration
- **THEN** Ito can apply each configured versioned validator to the same artifact
- **AND** diagnostics identify the validator that produced each issue

#### Scenario: Existing single-validator schemas remain valid

- **GIVEN** a schema declares a single `validate_as` validator for an artifact
- **WHEN** Ito loads the validation configuration
- **THEN** Ito preserves the current behavior for that schema
<!-- ITO:END -->
