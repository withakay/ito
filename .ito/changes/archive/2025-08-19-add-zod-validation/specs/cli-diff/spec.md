## ADDED Requirements

### Requirement: Diff Command Enhancement

The diff command SHALL validate change structure before displaying differences.

#### Scenario: Validate before diff

- **WHEN** executing `ito diff change-name`
- **THEN** validate change structure
- **AND** show validation warnings if present
- **AND** continue with diff display
