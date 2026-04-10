## ADDED Requirements

### Requirement: Wiki maintenance skill

The system SHALL provide an installable Ito wiki maintenance skill that teaches the harness how to set up and maintain `.ito/wiki/`.

#### Scenario: Maintain wiki through installable skill

- **WHEN** a user asks the harness to create, refresh, ingest into, or lint the Ito wiki
- **THEN** the harness can invoke an installable wiki maintenance skill
- **AND** the skill instructs the harness to respect the configured write boundary inside `.ito/wiki/`
- **AND** the skill explains how to update `index.md`, `log.md`, and `_meta/status.md`

### Requirement: Wiki search skill

The system SHALL provide an installable Ito wiki search skill for answering from the wiki first.

#### Scenario: Search wiki before raw sources

- **WHEN** a user asks a planning, research, or recall question that may already be covered by the wiki
- **THEN** the harness can invoke a wiki search skill
- **AND** the skill begins from `.ito/wiki/index.md`
- **AND** the skill distinguishes between answering in chat and filing a durable result back into the wiki
