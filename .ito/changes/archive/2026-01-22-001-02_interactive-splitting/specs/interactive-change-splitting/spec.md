## ADDED Requirements

### Requirement: Interactive splitting prompt

The validator SHALL prompt the user for action when validation warnings occur in an interactive session.

#### Scenario: Prompt for large change warning

- **WHEN** validation detects >10 deltas in a change AND session is interactive
- **THEN** system displays warning "Change has X deltas (limit 10)"
- **AND** system prompts user with options: "Split change", "Suppress warning", "Ignore"

#### Scenario: No prompt in non-interactive mode

- **WHEN** validation detects >10 deltas AND session is NOT interactive (CI/script)
- **THEN** system outputs warning to stderr
- **AND** system exits with code 0 (warnings don't fail build)

### Requirement: Split change workflow

The system SHALL guide the user through creating a new change and moving deltas to it.

#### Scenario: User selects "Split change"

- **WHEN** user selects "Split change" from warning prompt
- **THEN** system asks "Select deltas to move to new change"
- **AND** system displays multi-select list of all deltas in the current change

#### Scenario: Create destination change

- **WHEN** user confirms delta selection
- **THEN** system asks "Create new change for selected deltas?"
- **AND** system prompts for new change name (defaulting to current name + "-part2")

#### Scenario: Execute split

- **WHEN** user provides new change name
- **THEN** system creates new change
- **AND** system moves selected deltas to new change specs
- **AND** system updates original change specs to remove moved deltas
- **AND** system reports success "Moved X deltas to new change Y"

### Requirement: Suppress warning workflow

The system SHALL allow users to explicitly suppress warnings for a specific change.

#### Scenario: User selects "Suppress warning"

- **WHEN** user selects "Suppress warning"
- **THEN** system adds `ignore_warnings: ["max_deltas"]` to the change's `.ito.yaml` config
- **AND** system reports "Warning suppressed for this change"

#### Scenario: Suppressed warning check

- **WHEN** validation runs on a change with `ignore_warnings` config
- **THEN** system skips the check for that specific warning
