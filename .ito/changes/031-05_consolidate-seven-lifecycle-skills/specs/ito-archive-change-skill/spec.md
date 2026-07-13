<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Ito Archive Change Skill
The system SHALL provide the retained `ito-archive` lifecycle skill for promoting accepted delta specs, archiving completed changes, and reporting archive follow-through. It MUST NOT install the obsolete `/ito-archive-change` wrapper as a separate skill.

#### Scenario: Archive a complete change
- **WHEN** `ito-archive` receives a change whose required artifacts and tasks are complete
- **THEN** it follows the authoritative archive instruction
- **AND** promotes accepted specs before moving the change into the dated archive location

#### Scenario: Archive request has no change ID
- **WHEN** `ito-archive` is invoked without a change
- **THEN** it uses the supported change-selection flow without invoking a retired helper

### Requirement: Spec Sync Prompt
The retained archive workflow SHALL make spec promotion an explicit archive decision when delta specs exist. It MUST use the archive instruction or direct CLI behavior and MUST NOT invoke an `ito-sync-specs` skill.

#### Scenario: Delta specs exist
- **WHEN** archive preflight finds delta specs in the completed change
- **THEN** it presents the promotion action and its effects
- **AND** applies the accepted deltas through the archive workflow before archiving

#### Scenario: No delta specs exist
- **WHEN** archive preflight finds no delta specs
- **THEN** it proceeds without offering a retired sync-skill action

### Requirement: Skill Output
The retained `ito-archive` skill SHALL report archive location, schema, spec-promotion results, wiki/memory follow-through, and any cleanup guidance without embedding output from a retired sync skill.

#### Scenario: Archive completes with spec promotion
- **WHEN** archive completes after promoting delta specs
- **THEN** the output summarizes promoted capabilities and the archived location
- **AND** names any remaining lifecycle follow-through

#### Scenario: Archive completes without spec promotion
- **WHEN** archive completes with no delta specs to promote
- **THEN** the output identifies the archived location and schema

## ADDED Requirements

### Requirement: Archive reconciles accepted delta operations into current specs
The archive implementation SHALL reconcile accepted delta specs by exact requirement heading before moving the change. It SHALL normalize current specs to a single `## Requirements` section and preserve unrelated requirements and purpose text.

#### Scenario: Added requirement
- **WHEN** a delta contains an ADDED requirement absent from the current spec
- **THEN** archive appends that requirement exactly once

#### Scenario: Modified requirement
- **WHEN** a delta contains a MODIFIED requirement with an exact current heading
- **THEN** archive replaces only that requirement and preserves unrelated requirements

#### Scenario: Removed requirement
- **WHEN** a delta contains a REMOVED requirement with an exact current heading
- **THEN** archive removes that requirement
- **AND** removes the capability spec when no current requirements remain

#### Scenario: Renamed requirement
- **WHEN** a delta contains a RENAMED `FROM:` and `TO:` pair
- **THEN** archive renames the exact current requirement without changing its body

#### Scenario: Invalid delta identity
- **WHEN** a delta attempts to add a duplicate or modify, remove, or rename a missing requirement
- **THEN** archive fails before overwriting the current spec
<!-- ITO:END -->
