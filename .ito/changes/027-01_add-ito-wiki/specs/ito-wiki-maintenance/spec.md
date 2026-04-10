## ADDED Requirements

### Requirement: Incremental wiki refresh

The system SHALL support incremental refresh of the Ito wiki from changed or newly relevant Ito artifacts.

#### Scenario: Refresh after new research or changes

- **WHEN** an agent refreshes the wiki after new proposals, spec updates, research output, or archived changes
- **THEN** the agent updates the most relevant existing pages first
- **AND** creates new pages only for durable concepts, topics, or syntheses
- **AND** updates `index.md`, `log.md`, and `_meta/status.md` to reflect the refresh

### Requirement: Query and file-back workflow

The system SHALL support answering questions from the wiki first and filing durable query outputs back into the wiki when appropriate.

#### Scenario: Answer from index-first lookup

- **WHEN** an agent receives a question that the wiki can answer
- **THEN** it reads `.ito/wiki/index.md` first to locate relevant pages
- **AND** reads only the minimum additional pages needed to answer well
- **AND** cites the relevant wiki pages in its response

#### Scenario: File durable query result back into wiki

- **WHEN** a query produces a durable comparison, synthesis, or decision aid
- **THEN** the agent may save that result under the wiki as a new page
- **AND** updates `index.md` and `log.md` accordingly

### Requirement: Wiki lint workflow

The system SHALL support linting the Ito wiki for health and coverage problems.

#### Scenario: Detect stale or weakly connected content

- **WHEN** an agent performs a wiki lint pass
- **THEN** it looks for stale pages, contradictions, orphan pages, missing cross-links, and obvious source drift from newer Ito artifacts
- **AND** it records concrete follow-up guidance in the wiki or in its response
- **AND** it updates `_meta/status.md` to reflect the lint result
