## MODIFIED Requirements

### Requirement: ito-domain crate dependencies

`ito-domain` SHALL depend on `ito-common` ONLY (no other `ito-*` dependencies).

`ito-domain` SHALL NOT depend on `ito-core`, `ito-config`, `ito-cli`, `ito-web`, or any adapter crate.

`ito-domain` SHALL contain a `schemas` module providing workflow-related serde types previously supplied by `ito-schemas`:

- `schemas::workflow` (e.g., `WorkflowDefinition`, `WaveDefinition`, `TaskDefinition`, `AgentType`, `TaskType`)
- `schemas::workflow_plan` (e.g., `ExecutionPlan`)
- `schemas::workflow_state` (e.g., `WorkflowExecution`)

Schema types MUST be pure data and validation: no filesystem access and no process execution.

#### Scenario: Domain depends only on common

- **WHEN** running `cargo tree -p ito-domain`
- **THEN** it includes `ito-common`
- **AND** it does not include `ito-core`
- **AND** it does not include `ito-config`
- **AND** it does not include `ito-cli`
- **AND** it does not include `ito-web`

#### Scenario: Schema types are pure data

- **WHEN** reviewing the `ito-domain::schemas` module
- **THEN** it contains only serde data types and validation logic
- **AND** it performs no filesystem or process I/O

### Requirement: Discovery module in ito-domain

`ito-domain` SHALL provide a `discovery` module for listing changes, modules, and specs.

Discovery functions SHALL accept generic `F: FileSystem` for testability. Production code in `ito-domain` SHALL NOT use `std::fs` directly.

#### Scenario: Discovery functions are filesystem-abstracted

- **WHEN** calling discovery functions
- **THEN** they accept generic `F: FileSystem`
- **AND** `ito-domain` production code does not call `std::fs` directly
