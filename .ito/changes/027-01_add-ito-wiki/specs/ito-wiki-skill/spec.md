## ADDED Requirements

### Requirement: Wiki maintenance skill

The system SHALL provide an installable Ito wiki maintenance skill that teaches the harness how to set up, refresh, ingest into, and lint `.ito/wiki/`.

#### Scenario: Maintain wiki through installable skill

- **WHEN** a user asks the harness to create, refresh, ingest into, repair, or lint the Ito wiki
- **THEN** the harness can invoke an installable wiki maintenance skill
- **AND** the skill instructs the harness to respect the configured write boundary inside `.ito/wiki/`
- **AND** the skill explains how to update `index.md`, `log.md`, `_meta/status.md`, page metadata, source references, and cross-links
- **AND** the skill uses warn-and-update behavior rather than blocking when the wiki is stale or incomplete

#### Scenario: Maintain topic-oriented graph pages

- **WHEN** the maintenance skill incorporates new archived changes, specs, research, or decisions
- **THEN** it updates relevant topic pages with links to specs, modules, changes, research, and documentation
- **AND** it creates standalone artifact pages only when topic pages are insufficient

### Requirement: Wiki search skill

The system SHALL provide an installable Ito wiki search skill for answering from the wiki first with citations and controlled fallback.

#### Scenario: Search wiki before raw sources

- **WHEN** a user asks a planning, research, or recall question that may already be covered by the wiki
- **THEN** the harness can invoke a wiki search skill
- **AND** the skill begins from `.ito/wiki/index.md`
- **AND** the skill reads relevant wiki pages before broader raw-source exploration
- **AND** the skill distinguishes between answering in chat and filing a durable result back into the wiki

#### Scenario: Search quality and citation behavior

- **WHEN** the wiki search skill answers a question
- **THEN** it cites relevant wiki pages and source artifacts
- **AND** it calls out stale, missing, or contradictory wiki coverage
- **AND** it falls back to raw Ito artifacts when needed instead of hallucinating from incomplete wiki context

### Requirement: Wiki lint skill behavior

The system SHALL provide skill guidance for checking wiki health without requiring a first-iteration CLI lint command.

#### Scenario: Lint through skill workflow

- **WHEN** a user asks to lint or audit the Ito wiki
- **THEN** the maintenance skill checks page metadata, stale pages, missing source links, orphan pages, weak cross-links, contradictions, and coverage gaps
- **AND** returns actionable findings
- **AND** updates `_meta/status.md` and `log.md` when the user asks it to repair or record the lint pass
