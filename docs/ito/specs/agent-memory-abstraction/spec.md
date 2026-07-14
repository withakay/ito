<!-- ITO:START -->
# Agent Memory Abstraction

## Purpose

This spec defines the current behavior and requirements for agent memory abstraction.

## Requirements

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
