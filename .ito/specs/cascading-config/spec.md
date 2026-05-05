## ADDED Requirements

### Requirement: Cascading config is resolved once per CLI invocation

Within a single CLI invocation, the system SHALL resolve cascading project config at most once and reuse the merged result for all consumers.

#### Scenario: Multiple config consumers share the same resolved config

- **GIVEN** a single CLI invocation loads testing policy and worktree config
- **WHEN** both consumers request configuration
- **THEN** the system resolves cascading config once
- **AND** both consumers use the same merged config view
