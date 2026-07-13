<!-- ITO:START -->
## ADDED Requirements

### Requirement: Migrate-to-main instruction is always available
The standard Ito binary SHALL embed and render `ito agent instruction migrate-to-main` even when backend and coordination-branch runtime features are not compiled.
- **Requirement ID**: agent-instructions:migrate-to-main-availability
- **Tags**: behavior

#### Scenario: Standard build renders migration instruction
- **GIVEN** Ito was built without backend and coordination-branch features
- **WHEN** a user runs `ito agent instruction migrate-to-main`
- **THEN** Ito renders the complete migration prompt successfully

### Requirement: Legacy diagnostics name one remediation
Every warning or blocking diagnostic produced by legacy coordination detection SHALL name the exact command `ito agent instruction migrate-to-main` and SHALL explain whether the attempted operation was allowed as a read or rejected as a write.
- **Requirement ID**: agent-instructions:legacy-coordination-remediation
- **Tags**: behavior

#### Scenario: Read warning identifies remediation
- **GIVEN** a read-only command is allowed against legacy state
- **WHEN** Ito prints the legacy-state warning
- **THEN** the warning includes `ito agent instruction migrate-to-main`
- **AND** states that the current operation remained read-only

#### Scenario: Write error identifies remediation
- **GIVEN** a mutating command is rejected against legacy state
- **WHEN** Ito prints the blocking error
- **THEN** the error includes `ito agent instruction migrate-to-main`
- **AND** states that no mutation occurred
<!-- ITO:END -->
