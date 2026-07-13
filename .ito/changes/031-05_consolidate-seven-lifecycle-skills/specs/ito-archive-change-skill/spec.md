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
<!-- ITO:END -->
