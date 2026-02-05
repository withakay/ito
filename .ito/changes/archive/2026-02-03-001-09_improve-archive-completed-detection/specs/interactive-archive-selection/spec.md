## ADDED Requirements

### Requirement: Archive command prompts for selection when no change ID provided

The system SHALL prompt the user to select from completed changes when `ito archive` is invoked without a change ID argument.

#### Scenario: Interactive selection with completed changes available

- **WHEN** user runs `ito archive` without a change ID argument
- **AND** there are one or more completed changes
- **THEN** the system SHALL display a list of completed changes for selection
- **AND** the user can select which change(s) to archive

#### Scenario: No completed changes available for archiving

- **WHEN** user runs `ito archive` without a change ID argument
- **AND** there are no completed changes
- **THEN** the system SHALL display a message indicating no changes are ready to archive
- **AND** suggest running `ito list` to see change statuses

### Requirement: Archive skill supports interactive selection flow

The `/ito-archive` skill SHALL support an interactive flow when no change ID is specified, asking the user to select from completed changes.

#### Scenario: Skill prompts for completed change selection

- **WHEN** the `/ito-archive` skill is invoked without specifying a change ID
- **THEN** the skill SHALL query for completed changes
- **AND** present them to the user for selection before proceeding

#### Scenario: Skill proceeds with explicit change ID

- **WHEN** the `/ito-archive` skill is invoked with a specific change ID
- **THEN** the skill SHALL proceed directly with that change (existing behavior preserved)

### Requirement: Selection interface shows change context

When presenting changes for archive selection, the system SHALL show helpful context about each change.

#### Scenario: Selection list shows change details

- **WHEN** the interactive selection list is displayed
- **THEN** each option SHALL include the change name and completion date/last modified date
- **AND** optionally show the proposal summary if available

#### Scenario: Multiple selection support

- **WHEN** multiple changes are completed
- **THEN** the user SHALL be able to select multiple changes to archive in sequence
