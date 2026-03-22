<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Dashboard Display

The system SHALL provide a `dashboard` command that displays a dashboard overview of specs and changes.

The `ito view` command surface SHALL also expose a `proposal` subcommand (`ito view proposal <change-id>`) for viewing change artifacts; this does not affect the existing `ito dashboard` behavior.

#### Scenario: Basic dashboard display

- **WHEN** user runs `ito dashboard`
- **THEN** system displays a formatted dashboard with sections for summary, active changes, completed changes, and specifications

#### Scenario: No Ito directory

- **WHEN** user runs `ito dashboard` in a directory without Ito
- **THEN** system displays error message "✗ No ito directory found"

#### Scenario: Proposal subcommand is distinct from dashboard

- **WHEN** user runs `ito view proposal <change-id>`
- **THEN** the system routes to the proposal viewer, not the dashboard
- **AND** the dashboard display is not shown
<!-- ITO:END -->
