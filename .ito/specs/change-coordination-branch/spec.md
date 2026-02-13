## ADDED Requirements

### Requirement: Coordination branch can be provisioned before first change creation

The system SHALL provide a workflow to provision the coordination branch on `origin` before change-creation operations attempt reservation pushes.

#### Scenario: Init provisioning honors configured branch name

- **GIVEN** `changes.coordination_branch.name` is set to a custom branch
- **WHEN** the user runs `ito init --setup-coordination-branch`
- **THEN** provisioning targets the configured branch name
- **AND** no hardcoded fallback branch name is used for remote setup
