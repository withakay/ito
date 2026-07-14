## ADDED Requirements

### Requirement: Proposal and research workflows consult the wiki

Ito planning-oriented workflows SHALL treat the wiki as a preferred knowledge surface when it exists, while preserving raw Ito artifacts as the fallback source.

#### Scenario: Proposal workflow consults wiki first

- **WHEN** an agent starts proposal or planning work in a repo that has `.ito/wiki/index.md`
- **THEN** the workflow guidance tells the agent to consult the wiki before doing broader raw-source exploration
- **AND** the guidance tells the agent to warn if wiki coverage is absent, stale, or contradictory
- **AND** the guidance tells the agent to fall back to raw Ito artifacts and continue work
- **AND** the guidance suggests updating the wiki when the proposal work creates durable synthesis

#### Scenario: Research workflow files durable outputs back into wiki

- **WHEN** research produces durable findings, comparisons, or syntheses
- **THEN** workflow guidance suggests filing those results back into the wiki in addition to keeping the original research artifact
- **AND** the wiki update includes source links, freshness metadata, and topic-page cross-links
- **AND** short-lived findings remain in chat or research artifacts without forcing a wiki page

### Requirement: Archive workflows refresh wiki knowledge

Ito archive-oriented workflows SHALL treat wiki refresh as a normal post-archive maintenance step that updates topic pages and graph links without blocking archive completion by default.

#### Scenario: Archive completes after spec sync or change completion

- **WHEN** an agent finishes archiving a change or syncing its specs
- **THEN** the workflow guidance tells the agent to refresh relevant wiki topic pages from the archived change and any affected current specs
- **AND** the guidance frames the refresh as recommended follow-through rather than an implicit background action
- **AND** the guidance tells the agent to warn when wiki refresh is skipped or when stale coverage remains

#### Scenario: Archive update links graph-relevant sources

- **WHEN** archive-driven wiki refresh records durable knowledge
- **THEN** the updated wiki pages link to affected specs, modules, archived changes, research artifacts, architecture notes, and relevant documentation
- **AND** the update prefers topic-page synthesis over one archived-change page per change
