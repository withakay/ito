## ADDED Requirements

### Requirement: Incomplete change guidance display

When validation fails due to incomplete change, the archive command MUST display clear, user-friendly guidance on appropriate next actions. The guidance SHALL provide specific recommendations based on what is incomplete.

#### Scenario: Guidance for missing proposal

- **WHEN** validation fails due to missing proposal.md
- **THEN** display message: "This change cannot be archived because the proposal is missing."
- **AND** display recommendation: "Create a proposal by running: ito instructions proposal --change <id>"
- **AND** do not archive the change

#### Scenario: Guidance for missing specs

- **WHEN** validation fails due to missing or incomplete specs
- **THEN** display message: "This change cannot be archived because specs are missing or incomplete."
- **AND** display recommendation: "Create specs by running: ito spec create <name> --change <id>"
- **AND** do not archive the change

#### Scenario: Guidance for incomplete implementation

- **WHEN** validation fails due to incomplete implementation
- **THEN** display message: "This change cannot be archived because implementation is incomplete."
- **AND** display recommendation: "View remaining tasks by running: ito status --change <id>"
- **AND** display recommendation: "Continue implementation by running: ito instructions apply --change <id>"
- **AND** do not archive the change

### Requirement: Multiple issues guidance

When a change has multiple completion issues, the guidance MUST enumerate all issues and provide a comprehensive set of recommendations.

#### Scenario: Multiple missing artifacts

- **WHEN** validation detects missing proposal.md and missing specs
- **THEN** display message: "This change cannot be archived because multiple artifacts are missing:"
- **AND** list all missing artifacts (proposal.md, specs)
- **AND** provide recommendation for each missing artifact
- **AND** display example command for addressing all issues

#### Scenario: Missing artifacts and incomplete implementation

- **WHEN** validation detects missing proposal.md, missing specs, and incomplete implementation
- **THEN** display message: "This change cannot be archived because it is incomplete:"
- **AND** list all issues (missing proposal, missing specs, incomplete implementation)
- **AND** prioritize recommendations (proposal first, then specs, then implementation)
- **AND** provide step-by-step guidance

### Requirement: Abandon change guidance

For users who want to abandon an incomplete change instead of completing it, the guidance MUST provide clear instructions on how to properly abandon or delete the change.

#### Scenario: Guidance for abandoning incomplete change

- **WHEN** user wants to abandon an incomplete change
- **THEN** display message: "If you want to abandon this change instead of completing it, you can:"
- **AND** display recommendation: "Delete the change by running: ito delete <id> --confirm"
- **AND** warn that deletion is irreversible

### Requirement: Draft state guidance

The guidance MUST support marking changes as draft when they are intentionally incomplete but should be preserved for future work.

#### Scenario: Guidance for marking as draft

- **WHEN** user wants to mark incomplete change as draft
- **THEN** display message: "If you want to save this incomplete change for future work, you can:"
- **AND** display recommendation: "Mark as draft by running: ito draft <id>"
- **AND** explain that draft changes are preserved but not considered for archive

### Requirement: Interactive guidance mode

When the archive command is run in interactive mode (--interactive flag), users SHALL be presented with a menu of options for handling incomplete changes.

#### Scenario: Interactive menu for incomplete change

- **WHEN** user runs 'ito archive <id> --interactive' on incomplete change
- **THEN** display validation failure message
- **AND** present menu options:
  - \[1\] Continue implementation (runs ito instructions apply)
  - \[2\] View status (runs ito status)
  - \[3\] Mark as draft (runs ito draft)
  - \[4\] Abandon/delete change (runs ito delete --confirm)
  - \[5\] Force archive (runs ito archive --force)
- **AND** wait for user selection
- **AND** execute selected action

### Requirement: Guidance formatting and readability

All guidance messages MUST be formatted for readability with clear headings, bullet points, and code examples. Terminal colors SHOULD be used to distinguish between error messages, recommendations, and command examples.

#### Scenario: Formatted guidance output

- **WHEN** guidance is displayed for incomplete change
- **THEN** error message is displayed in red color
- **AND** recommendations are displayed in yellow color
- **AND** command examples are displayed in code block format
- **AND** each section has a clear heading

### Requirement: Helpful error context

Error messages MUST include context about what is expected for completeness, helping users understand the requirements.

#### Scenario: Context for spec requirements

- **WHEN** validation detects missing scenarios in specs
- **THEN** error message explains what makes a spec complete
- **AND** states: "Each spec must contain at least one '#### Scenario:' block"
- **AND** provides example scenario format

#### Scenario: Context for implementation requirements

- **WHEN** validation detects incomplete implementation
- **THEN** error message explains what makes implementation complete
- **AND** states: "Implementation is complete when all tasks are marked as completed"
- **AND** indicates how many tasks remain

### Requirement: Guidance reference documentation

When available, guidance MUST reference relevant documentation for users who want to learn more about the change management workflow.

#### Scenario: Documentation reference

- **WHEN** guidance is displayed for incomplete change
- **AND** ito documentation exists
- **THEN** display message: "Learn more about the ito workflow at: <documentation-url>"
- **AND** link is clickable in supported terminals
