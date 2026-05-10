## ADDED Requirements

### Requirement: Backend runtime provides repository-ready remote clients

When remote persistence mode is active, Ito SHALL resolve backend runtime state that is sufficient to construct remote-backed repository implementations for change, task, module, and spec access.

#### Scenario: Remote runtime is reused across repository implementations

- **GIVEN** remote persistence mode is active and runtime resolution succeeds
- **WHEN** Ito constructs remote-backed repositories
- **THEN** those repositories share the resolved runtime context instead of performing command-local backend setup independently
