<!-- ITO:START -->
## REMOVED Requirements

### Requirement: Specs Sync Skill
The system SHALL provide an `/ito-sync-specs` skill that reconciles change delta specs into current specs.

**Reason**: Spec promotion is part of the retained `ito-archive` lifecycle and does not need another installed activation name.
**Migration**: Use `ito-archive` or the direct archive CLI/instruction flow; reconciliation behavior remains internal to accepted spec promotion.

#### Scenario: Archive owns accepted spec promotion
- **WHEN** a completed change with delta specs is archived
- **THEN** `ito-archive` performs or directs the supported reconciliation
- **AND** no `ito-sync-specs` skill or command wrapper is installed

### Requirement: Delta Reconciliation Logic
The retired sync skill SHALL no longer own delta reconciliation.

**Reason**: Reconciliation is a core archive operation, not a separate agent activation surface.
**Migration**: Use `ito archive` or `ito-archive`; archive applies ADDED, MODIFIED, REMOVED, and RENAMED operations to current specs.

#### Scenario: Reconciliation moves into archive
- **WHEN** an accepted change contains delta specs
- **THEN** the archive workflow reconciles them without invoking `ito-sync-specs`

### Requirement: Skill Output
The retired sync skill SHALL no longer own promotion output.

**Reason**: Promotion feedback belongs to the archive result that performed the operation.
**Migration**: Read the `ito archive` result for affected capabilities and archive follow-through.

#### Scenario: Archive reports promotion
- **WHEN** archive reconciliation completes
- **THEN** the archive result reports the affected capabilities without a sync-skill handoff
<!-- ITO:END -->
