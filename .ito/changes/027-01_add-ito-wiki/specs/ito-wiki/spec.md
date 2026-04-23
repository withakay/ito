## ADDED Requirements

### Requirement: Ito wiki root and reserved artifacts

The system SHALL provide a persistent `.ito/wiki/` root that acts as an LLM-maintained knowledge layer for Ito artifacts.

#### Scenario: Initialize wiki scaffold

- **WHEN** an Ito project is initialized or upgraded with wiki support
- **THEN** the project contains `.ito/wiki/index.md`, `.ito/wiki/log.md`, `.ito/wiki/overview.md`, `.ito/wiki/_meta/config.yaml`, `.ito/wiki/_meta/schema.md`, and `.ito/wiki/_meta/status.md`
- **AND** the scaffold is plain markdown plus simple config files suitable for Obsidian-style browsing
- **AND** existing wiki content is preserved instead of overwritten blindly

### Requirement: Ito-scoped source boundary

The Ito wiki SHALL treat Ito-owned artifacts as its default source boundary and MUST NOT become a general project wiki by default.

#### Scenario: Build wiki from Ito artifacts

- **WHEN** an agent refreshes or maintains the wiki
- **THEN** it uses `.ito/changes/`, `.ito/specs/`, `.ito/research/`, `.ito/modules/`, `.ito/project.md`, `.ito/architecture.md`, and related Ito guidance files as default sources
- **AND** files outside `.ito/` are referenced only when intentionally linked or cited as supporting context
- **AND** the wiki does not mirror arbitrary repo code or general docs by default

### Requirement: Durable navigation artifacts

The Ito wiki SHALL maintain durable navigation artifacts that let an LLM and a human browse the knowledge layer without re-deriving structure from raw sources on every query.

#### Scenario: Navigate the wiki through index and status

- **WHEN** an agent or user opens `.ito/wiki/index.md` and `.ito/wiki/_meta/status.md`
- **THEN** they can discover the major page groups, current freshness state, and notable coverage gaps
- **AND** `log.md` provides an append-only timeline of wiki operations
- **AND** the wiki schema describes expected page types and cross-linking conventions
