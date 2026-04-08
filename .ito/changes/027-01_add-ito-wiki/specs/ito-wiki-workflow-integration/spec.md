## ADDED Requirements

### Requirement: Proposal and research workflows consult the wiki

Ito planning-oriented workflows SHALL treat the wiki as a preferred knowledge surface when it exists.

#### Scenario: Proposal workflow consults wiki first

- **WHEN** an agent starts proposal or planning work in a repo that has `.ito/wiki/index.md`
- **THEN** the workflow guidance tells the agent to consult the wiki before doing broader raw-source exploration
- **AND** the guidance does not block work if the wiki is absent or stale

#### Scenario: Research workflow files durable outputs back into wiki

- **WHEN** research produces durable findings or syntheses
- **THEN** workflow guidance suggests filing those results back into the wiki in addition to keeping the original research artifact

### Requirement: Archive workflows refresh wiki knowledge

Ito archive-oriented workflows SHALL treat wiki refresh as a normal post-archive maintenance step.

#### Scenario: Archive completes after spec sync or change completion

- **WHEN** an agent finishes archiving a change or syncing its specs
- **THEN** the workflow guidance tells the agent to refresh the wiki from the archived change and any affected current specs
- **AND** the guidance frames the refresh as recommended follow-through rather than an implicit background action
