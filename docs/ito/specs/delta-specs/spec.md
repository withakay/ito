## ADDED Requirements

### Requirement: Delta requirements can declare reference ids

The delta specs format SHALL allow a requirement block to include an explicit metadata line of the form `- **Requirement ID**: <id>`.

#### Scenario: Requirement id is preserved during parsing

- **GIVEN** a delta requirement includes `- **Requirement ID**: tasks-tracking:enhanced-requirements`
- **WHEN** the change delta is parsed
- **THEN** Ito preserves that requirement id as structured metadata on the requirement block
