## ADDED Requirements

### Requirement: Integrate user-added context into preamble

The system SHALL integrate user-added context (from `--add-context` flag) into the preamble as a dedicated section that appears before the task description.

#### Scenario: Context section when context exists

- **WHEN** the ralph loop loads context from the context file
- **WHEN** context content is non-empty
- **THEN** the preamble SHALL include a "## Additional Context (added by user mid-loop)" section
- **THEN** the context section SHALL appear before the "## Your Task" section
- **THEN** the context section SHALL be followed by a separator line "---"

#### Scenario: No context section when context is empty

- **WHEN** the ralph loop loads context from the context file
- **WHEN** context content is empty or null
- **THEN** the preamble SHALL NOT include an "## Additional Context" section
- **THEN** the preamble SHALL proceed directly to the task section

#### Scenario: Context cleared between iterations

- **WHEN** user runs `ito ralph --clear-context` for a change
- **WHEN** the next iteration runs
- **THEN** the preamble SHALL NOT include the context section
- **THEN** the preamble SHALL reflect that context has been cleared

### Requirement: Load context from state directory

The system SHALL load user-added context from the ralph state directory for the current change ID.

#### Scenario: Load existing context file

- **WHEN** the ralph loop starts an iteration
- **WHEN** a context file exists at `.ito/.state/ralph/{changeId}/context.txt`
- **THEN** the system SHALL read and return the context content
- **THEN** the context content SHALL be passed to the preamble builder

#### Scenario: Handle missing context file

- **WHEN** the ralph loop starts an iteration
- **WHEN** no context file exists for the change ID
- **THEN** the system SHALL return null or empty string for context
- **THEN** the preamble builder SHALL omit the context section
