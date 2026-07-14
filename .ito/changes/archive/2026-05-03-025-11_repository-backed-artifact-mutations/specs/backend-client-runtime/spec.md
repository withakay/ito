## MODIFIED Requirements

### Requirement: Backend runtime provides repository-ready remote clients

When remote persistence mode is active, Ito SHALL resolve backend runtime state that is sufficient to construct remote-backed repository implementations for change, task, module, and spec access.

The same resolved runtime SHALL also be sufficient to construct remote-backed artifact mutation clients for active-work change/spec artifact updates.

#### Scenario: Remote runtime is reused across repository implementations

- **GIVEN** remote persistence mode is active and runtime resolution succeeds
- **WHEN** Ito constructs remote-backed repositories
- **THEN** those repositories share the resolved runtime context instead of performing command-local backend setup independently

#### Scenario: Remote runtime is reused for artifact mutation clients

- **GIVEN** remote persistence mode is active and runtime resolution succeeds
- **WHEN** Ito constructs remote-backed artifact mutation clients
- **THEN** those clients share the same resolved runtime context as the repository readers
- **AND** command handlers do not perform independent backend configuration for artifact mutation operations
