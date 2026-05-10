## ADDED Requirements

### Requirement: Incremental wiki refresh

The system SHALL support incremental refresh of the Ito wiki from changed or newly relevant Ito artifacts.

#### Scenario: Refresh after new research or changes

- **WHEN** an agent refreshes the wiki after new proposals, spec updates, research output, or archived changes
- **THEN** the agent updates the most relevant existing topic pages first
- **AND** creates new pages only for durable concepts, topics, decision records, research syntheses, or query artifacts
- **AND** updates `index.md`, `log.md`, and `_meta/status.md` to reflect the refresh
- **AND** records source references and freshness metadata for changed pages

#### Scenario: Wiki is stale or missing during workflow

- **WHEN** an agent starts proposal, research, or archive work and the wiki is absent, stale, or missing coverage
- **THEN** the agent warns that wiki coverage is unavailable or stale
- **AND** falls back to raw Ito artifacts to continue the work
- **AND** updates the wiki when the resulting synthesis has durable value

### Requirement: Query and file-back workflow

The system SHALL support answering questions from the wiki first and filing durable query outputs back into the wiki when appropriate.

#### Scenario: Answer from index-first lookup

- **WHEN** an agent receives a planning, research, or recall question that the wiki may answer
- **THEN** it reads `.ito/wiki/index.md` first to locate relevant pages
- **AND** reads only the minimum additional wiki pages needed to answer well
- **AND** cites the relevant wiki pages and source artifacts in its response
- **AND** falls back to raw Ito artifacts when wiki coverage is missing, stale, or contradictory

#### Scenario: File durable query result back into wiki

- **WHEN** a query produces a durable comparison, synthesis, or decision aid
- **THEN** the agent may save that result under the wiki as a query artifact or topic-page update
- **AND** updates `index.md`, `log.md`, and page metadata accordingly
- **AND** avoids filing short-lived chat answers that do not have durable value

### Requirement: Wiki lint workflow

The system SHALL support linting the Ito wiki for health, freshness, graph, and coverage problems.

#### Scenario: Detect stale or weakly connected content

- **WHEN** an agent performs a wiki lint pass
- **THEN** it looks for stale pages, contradictions, orphan pages, missing cross-links, missing source references, missing authority metadata, and obvious source drift from newer Ito artifacts
- **AND** it records concrete follow-up guidance in `_meta/status.md`, the affected page, or its response
- **AND** it appends the lint pass to `log.md`

#### Scenario: Lint reports production-readiness gaps

- **WHEN** lint finds issues that reduce wiki trustworthiness
- **THEN** the lint result distinguishes warnings from suggested updates
- **AND** does not block workflows by default in this first iteration
- **AND** provides enough detail for an agent to repair the wiki incrementally
