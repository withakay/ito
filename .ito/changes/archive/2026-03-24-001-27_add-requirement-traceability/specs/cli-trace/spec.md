## ADDED Requirements

### Requirement: Trace command renders requirement coverage summary

The CLI SHALL expose `ito trace <change-id>` to render a change-package-local requirement traceability summary derived from delta requirement references and task requirement references for the resolved active or archived change.

#### Scenario: Trace command shows covered and uncovered requirements

- **GIVEN** a change is trace-ready
- **WHEN** a user runs `ito trace <change-id>`
- **THEN** the output includes the declared requirement ids for the change
- **AND** it identifies which requirements are covered and uncovered
- **AND** it surfaces unresolved task references, if any

#### Scenario: Trace command resolves archived change historically

- **GIVEN** an archived change exists for the requested canonical change id
- **AND** the archived change bundle contains traceability metadata
- **WHEN** a user runs `ito trace <change-id>`
- **THEN** the command loads the archived change bundle
- **AND** it renders a historical traceability summary from the archived change artifacts
- **AND** it labels the result as historical

### Requirement: Trace command explains unavailable traceability

If a change is not trace-ready, `ito trace <change-id>` SHALL succeed and explain why computed traceability is unavailable.

#### Scenario: Trace command reports unavailable status for checkbox change

- **GIVEN** a change declares requirement ids
- **AND** its active tracking file does not use enhanced task encoding
- **WHEN** a user runs `ito trace <change-id>`
- **THEN** the command succeeds
- **AND** the output explains that computed requirement coverage is unavailable because enhanced task trace references are not available

#### Scenario: Archived legacy change reports unavailable status

- **GIVEN** an archived change exists for the requested canonical change id
- **AND** the archived bundle predates requirement traceability metadata
- **WHEN** a user runs `ito trace <change-id>`
- **THEN** the command succeeds
- **AND** the output explains that historical computed traceability is unavailable for that archived change

### Requirement: Trace command emits machine-readable output

When invoked with `--json`, `ito trace <change-id>` SHALL emit machine-readable traceability output.

#### Scenario: Trace command outputs JSON summary

- **WHEN** a user runs `ito trace <change-id> --json`
- **THEN** the output is valid JSON
- **AND** it includes whether computed traceability is available
- **AND** it includes the lifecycle state of the resolved change
- **AND** it includes declared requirements, covered requirements, uncovered requirements, unresolved references, and unavailable reasons when applicable

### Requirement: Trace command surfaces invalid traced changes

If a change opts into requirement traceability but has invalid traceability metadata, `ito trace <change-id>` SHALL surface the invalid state explicitly rather than silently rendering a misleading coverage summary.

#### Scenario: Trace command reports invalid partial traceability

- **GIVEN** a change where some but not all delta requirements declare requirement ids
- **WHEN** a user runs `ito trace <change-id>`
- **THEN** the command reports that the change's traceability metadata is invalid
- **AND** it identifies the missing requirement-id coverage issue instead of reporting normal covered/uncovered results
