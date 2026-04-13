<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ralph state captures actionable run outcomes

Ralph SHALL persist enough state to explain what happened in the last run and support restart-context generation.

- **Requirement ID**: ralph-run-reporting:actionable-run-state

#### Scenario: Iteration history records run outcome details

- **WHEN** a Ralph iteration finishes
- **THEN** the persisted history SHALL record the iteration outcome
- **AND** the history SHALL include whether the completion promise was accepted or rejected
- **AND** the history SHALL include the harness exit result and effective working directory for the iteration

### Requirement: Ralph status shows restartable operator context

`ito ralph --status` SHALL show enough information for an operator or wrapper to resume the loop intelligently.

- **Requirement ID**: ralph-run-reporting:status-supports-resume

#### Scenario: Status reports latest iteration and failure context

- **WHEN** `ito ralph --status --change <change-id>` is executed
- **THEN** the output SHALL report the latest iteration number
- **AND** the output SHALL summarize the latest run outcome
- **AND** the output SHALL include the latest known validation rejection or failure reason when one exists

#### Scenario: Status reports task progress for targeted change

- **WHEN** `ito ralph --status --change <change-id>` is executed
- **THEN** the output SHALL include the current task progress summary for the targeted change

### Requirement: Restart summaries derive from persisted Ralph state

When a wrapper or operator restarts Ralph after an interrupted or failed run, the restart note SHALL be derived from persisted Ralph state and current task status.

- **Requirement ID**: ralph-run-reporting:restart-summary-from-state

#### Scenario: Restart summary uses prior run state

- **WHEN** a supervising wrapper prepares to rerun Ralph for a targeted change
- **THEN** it SHALL be able to derive a concise restart summary from the saved Ralph state and current task status
- **AND** the restart summary SHALL identify the last known progress and the reason the previous run ended or failed
<!-- ITO:END -->
