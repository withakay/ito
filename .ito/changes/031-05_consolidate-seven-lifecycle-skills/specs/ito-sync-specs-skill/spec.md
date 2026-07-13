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
<!-- ITO:END -->
