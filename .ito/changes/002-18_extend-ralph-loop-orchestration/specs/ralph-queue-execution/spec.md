<!-- ITO:START -->
## ADDED Requirements

### Requirement: Continue-ready processes all eligible changes with aggregate results

When Ralph runs in continue-ready mode, the system SHALL attempt every eligible change in stable order and report aggregate results for the sweep.

- **Requirement ID**: ralph-queue-execution:continue-ready-sweep

#### Scenario: Continue-ready keeps going after one change fails

- **WHEN** `ito ralph --continue-ready` runs against multiple eligible changes
- **AND** one targeted change run fails
- **THEN** Ralph SHALL record the failure for that change
- **AND** Ralph SHALL continue to the next eligible change instead of aborting the full sweep immediately

#### Scenario: Continue-ready returns aggregate failure when any change fails

- **WHEN** `ito ralph --continue-ready` finishes processing all eligible changes
- **AND** one or more targeted change runs failed
- **THEN** the command SHALL exit non-zero
- **AND** the command SHALL print a summary of succeeded and failed changes

### Requirement: Continue-module processes ready changes without re-running completed selections

When Ralph runs in continue-module mode, the system SHALL process ready changes in the targeted module while avoiding duplicate work within the same run.

- **Requirement ID**: ralph-queue-execution:continue-module-sweep

#### Scenario: Continue-module advances through ready changes

- **WHEN** `ito ralph --module <module-id>` implies continue-module behavior
- **AND** the module has multiple ready changes
- **THEN** Ralph SHALL run each ready change in stable order
- **AND** Ralph SHALL avoid re-running a change already processed earlier in the same continue-module session

#### Scenario: Continue-module still reports failures after full sweep

- **WHEN** continue-module processing finishes
- **AND** one or more targeted change runs failed
- **THEN** the command SHALL report the failed changes in a final summary
- **AND** the command SHALL exit non-zero

### Requirement: Queue sweeps report per-change outcomes

Queue execution SHALL make per-change outcomes visible to operators and wrappers.

- **Requirement ID**: ralph-queue-execution:per-change-outcomes

#### Scenario: Queue summary distinguishes completion states

- **WHEN** a queue-style Ralph run completes
- **THEN** the output SHALL distinguish at least successful, failed, and skipped-or-unavailable targets
- **AND** the summary SHALL identify each affected change by change id
<!-- ITO:END -->
