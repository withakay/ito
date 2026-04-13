<!-- ITO:START -->
## ADDED Requirements

### Requirement: Change-scoped Ralph runs include execution context

When Ralph targets a specific change, the system SHALL construct a change-scoped execution context instead of relying only on the base user prompt.

- **Requirement ID**: ralph-execution-context:change-scoped-context

#### Scenario: Change run includes proposal and task context

- **WHEN** `ito ralph --change <change-id>` starts an iteration
- **THEN** the prompt SHALL include labeled context for the targeted change proposal
- **AND** the prompt SHALL include the current task progress summary for the change
- **AND** the prompt SHALL include the next actionable task or tasks for the change when available

### Requirement: Change-scoped execution context includes Ito-native execution guidance

The change execution context SHALL include concise Ito-native execution guidance so Ralph can act like an autonomous change executor rather than a generic prompt loop.

- **Requirement ID**: ralph-execution-context:ito-execution-guidance

#### Scenario: Change run includes execution checklist

- **WHEN** Ralph builds a prompt for a targeted change
- **THEN** the prompt SHALL include a concise execution checklist derived from the change's implementation guidance
- **AND** the checklist SHALL reflect the change's tasks and validation expectations

### Requirement: Additional context and rejected validation remain visible

Ralph SHALL preserve and label user-added loop context and rejected validation context alongside the change execution context.

- **Requirement ID**: ralph-execution-context:preserve-loop-context

#### Scenario: Validation rejection is included with change context

- **GIVEN** the previous iteration rejected a completion promise
- **WHEN** the next iteration prompt is built
- **THEN** the prompt SHALL include the labeled validation failure section
- **AND** the prompt SHALL preserve any saved Ralph context for the targeted change

#### Scenario: Unscoped run degrades gracefully

- **WHEN** Ralph runs without `--change`
- **THEN** the system SHALL omit change-specific task and execution-guidance sections
- **AND** the loop SHALL still run using the provided prompt plus any module context that applies
<!-- ITO:END -->
