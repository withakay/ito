<!-- ITO:START -->
## REMOVED Requirements

### Requirement: Installed Ito memory skill
Ito SHALL install a shared `ito-memory` skill as a thin entrypoint to the configured memory instruction artifacts.

**Reason**: A standalone `ito-memory` skill expands the default surface even though authoritative memory behavior already comes from CLI instruction artifacts.
**Migration**: Memory query/search guidance moves into `ito-research`; capture/follow-through guidance moves into `ito-archive` and relevant review guidance. The existing `memory-capture`, `memory-search`, and `memory-query` instruction artifacts remain callable.

#### Scenario: Upgrade retires standalone memory skill
- **WHEN** Ito updates an installation containing an unmodified managed `ito-memory` skill
- **THEN** it prunes that obsolete skill
- **AND** retained lifecycle skills reference the existing memory instruction artifacts where configured

## ADDED Requirements

### Requirement: Memory provider instructions remain lifecycle-accessible
Ito SHALL retain provider-neutral memory instruction artifacts and SHALL make them discoverable through the lifecycle phases that consume or produce durable knowledge.

#### Scenario: Research queries configured memory
- **WHEN** research needs project memory
- **THEN** `ito-research` directs the agent to the configured search/query instruction artifact
- **AND** does not duplicate provider-specific policy

#### Scenario: Archive captures durable knowledge
- **WHEN** archive follow-through identifies durable project knowledge
- **THEN** `ito-archive` directs the agent to the configured capture instruction artifact
<!-- ITO:END -->
