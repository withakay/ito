## ADDED Requirements

### Requirement: Module targeting selects the first ready change

When the user targets Ralph with `--module <module-id>`, the system SHALL resolve to the lowest-ID ready change in that module.

#### Scenario: Module target auto-selects first ready change

- **GIVEN** module `<module-id>` contains multiple changes
- **AND** more than one change is in `Ready` work status
- **WHEN** executing `ito ralph --module <module-id> ...`
- **THEN** the system SHALL list ready changes for that module
- **AND** the system SHALL select the lowest-ID ready change as the execution target

#### Scenario: Module target fails when no ready changes exist but work remains

- **GIVEN** module `<module-id>` has no changes in `Ready` work status
- **AND** at least one module change is not `Complete`
- **WHEN** executing `ito ralph --module <module-id> ...`
- **THEN** the command SHALL fail
- **AND** the error SHALL identify remaining non-complete changes

### Requirement: Continuous module mode with drift-aware revalidation

The system SHALL support `--continue-module` to process ready changes across a module until module work is complete, while revalidating module readiness before and after each change execution.

#### Scenario: Continue-module processes all ready changes to completion

- **GIVEN** module `<module-id>` contains multiple ready changes
- **WHEN** executing `ito ralph --module <module-id> --continue-module ...`
- **THEN** the system SHALL execute Ralph for the lowest-ID ready change first
- **AND** after each completed change run, the system SHALL refresh module readiness and continue with the next lowest-ID ready change
- **AND** once all module changes are complete, the command SHALL exit successfully

#### Scenario: Continue-module reorients when module state shifts

- **GIVEN** `--continue-module` is running for module `<module-id>`
- **AND** another process changes module task state between selection and run start
- **WHEN** Ralph performs preflight module revalidation
- **THEN** the system SHALL re-select the current lowest-ID ready change
- **AND** the system SHALL continue execution against the reoriented target
