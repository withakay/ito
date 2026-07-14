<!-- ITO:START -->
# Cli Validate

## Purpose

This spec defines the current behavior and requirements for cli validate.

## Requirements

### Requirement: Validation defines trace-ready changes

When a change opts into requirement traceability, `ito validate <change-id>` MUST require every delta requirement in that change to declare a requirement id before computed coverage is considered available.

#### Scenario: Missing requirement id fails traced validation

- **GIVEN** a change where at least one delta requirement declares a requirement id
- **AND** another delta requirement in the same change declares no requirement id
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation fails with an actionable error identifying the requirement that is missing a requirement id

### Requirement: Validation fails on invalid requirement references

When a change provides traceability metadata, `ito validate <change-id>` MUST fail if the change contains duplicate requirement ids or task references that do not resolve within that change.

#### Scenario: Unknown task requirement reference fails validation

- **GIVEN** a change task declares a requirement reference that no delta requirement declares
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation fails with an actionable error identifying the unresolved reference and task

### Requirement: Validation reports uncovered requirements

When a change is trace-ready, `ito validate <change-id>` MUST report declared requirement ids that are not covered by any non-shelved enhanced task.

#### Scenario: Non-strict validation warns on uncovered requirement

- **GIVEN** a change declares a requirement id that no non-shelved enhanced task references
- **WHEN** executing `ito validate <change-id>` without `--strict`
- **THEN** validation reports the uncovered requirement as a warning

#### Scenario: Strict validation errors on uncovered requirement

- **GIVEN** a change declares a requirement id that no non-shelved enhanced task references
- **WHEN** executing `ito validate <change-id> --strict`
- **THEN** validation reports the uncovered requirement as an error

### Requirement: Validation reports unavailable computed traceability

When a change declares requirement traceability metadata but its active tracking file does not support enhanced task trace references, `ito validate <change-id>` MUST report that computed requirement coverage is unavailable instead of reporting every declared requirement as uncovered.

#### Scenario: Checkbox tracking reports unavailable coverage

- **GIVEN** a change declares requirement ids
- **AND** its active tracking file uses checkbox task encoding rather than enhanced task blocks
- **WHEN** executing `ito validate <change-id>`
- **THEN** validation reports that computed requirement coverage is unavailable for that change
- **AND** it does not treat every declared requirement as uncovered solely because enhanced task trace references are unavailable

### Requirement: Ubiquitous language consistency rule

When the `ubiquitous_language_consistency` rule is enabled, validation SHALL compare canonical domain terms from the canonical discovery handoff against proposal, spec, and task language and report drift.

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

When the `context_boundary_consistency` rule is enabled, validation SHALL warn when a proposal spans multiple bounded contexts without naming the affected contexts, describing ownership, or describing their relationship and translation boundaries.

- **Requirement ID**: `cli-validate:context-boundary-consistency`

#### Scenario: Cross-context proposal without context framing warns

- **GIVEN** a proposal changes behavior in more than one bounded context
- **AND** the proposal or discovery handoff does not name those contexts, ownership, relationship, or translation boundary
- **WHEN** validation runs with the rule enabled
- **THEN** validation reports a warning that boundary framing is incomplete

#### Scenario: Cross-context proposal with explicit relationship passes

- **GIVEN** a proposal names the affected bounded contexts, ownership, relationship, and translation boundary
- **WHEN** validation runs with the rule enabled
- **THEN** validation does not emit a boundary-consistency warning for that relationship

### Requirement: Domain documentation consistency rule

When the `domain_documentation_consistency` rule is enabled, validation SHALL warn when proposed `CONTEXT.md`, `CONTEXT-MAP.md`, or ADR updates conflict with the canonical discovery handoff or existing domain documentation.

- **Requirement ID**: `cli-validate:domain-documentation-consistency`

#### Scenario: Proposed context doc conflicts with discovery handoff

- **GIVEN** the discovery handoff defines a canonical term
- **AND** a proposed `CONTEXT.md` update defines the same term differently
- **WHEN** validation runs with the rule enabled
- **THEN** validation reports a warning naming the conflicting term and source locations

#### Scenario: Documentation updates match discovery handoff

- **GIVEN** proposed context or ADR updates use the same terms, context ownership, and decision rationale as the discovery handoff
- **WHEN** validation runs with the rule enabled
- **THEN** validation emits no domain-documentation consistency warning
<!-- ITO:END -->
