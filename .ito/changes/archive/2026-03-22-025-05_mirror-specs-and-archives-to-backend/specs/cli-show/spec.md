## ADDED Requirements

### Requirement: `ito show specs` resolves truth specs through SpecRepository

When rendering promoted truth specs, `ito show specs` SHALL resolve spec content through the runtime-selected `SpecRepository` implementation.

#### Scenario: Remote mode shows specs without local spec markdown

- **GIVEN** remote persistence mode is active
- **AND** promoted spec content exists in the selected remote-backed implementation
- **WHEN** the user runs `ito show specs`
- **THEN** Ito renders the bundled truth specs from `SpecRepository`
- **AND** the command does not require local `.ito/specs/` markdown to exist
