## ADDED Requirements

### Requirement: Ito wiki root and reserved artifacts

The system SHALL provide a persistent `.ito/wiki/` root that acts as an LLM-maintained knowledge layer for Ito artifacts.

#### Scenario: Initialize wiki scaffold

- **WHEN** an Ito project is initialized or upgraded with wiki support
- **THEN** the project contains `.ito/wiki/index.md`, `.ito/wiki/log.md`, `.ito/wiki/overview.md`, `.ito/wiki/_meta/config.yaml`, `.ito/wiki/_meta/schema.md`, and `.ito/wiki/_meta/status.md`
- **AND** the scaffold is plain markdown plus simple config files suitable for Obsidian-style browsing
- **AND** existing wiki content is preserved instead of overwritten blindly

#### Scenario: Upgrade existing wiki content

- **WHEN** `ito init --upgrade`, `ito update`, or equivalent template refresh runs in a project with existing `.ito/wiki/` content
- **THEN** Ito installs missing scaffold files without deleting or replacing existing wiki pages
- **AND** marker-managed sections may be updated only when the file explicitly uses Ito managed markers
- **AND** LLM-authored wiki content remains user-owned unless the schema explicitly marks a section as managed

### Requirement: Ito-scoped source boundary

The Ito wiki SHALL treat Ito-owned artifacts as its default source boundary and MUST NOT become a general project wiki by default.

#### Scenario: Build wiki from Ito artifacts

- **WHEN** an agent refreshes or maintains the wiki
- **THEN** it uses `.ito/changes/`, `.ito/specs/`, `.ito/research/`, `.ito/modules/`, `.ito/project.md`, `.ito/architecture.md`, and related Ito guidance files as default sources
- **AND** files outside `.ito/` are referenced only when intentionally linked or cited as supporting context
- **AND** the wiki does not mirror arbitrary repo code or general docs by default

#### Scenario: Link outside Ito sources intentionally

- **WHEN** a wiki page references code, docs, issues, PRs, or external URLs outside `.ito/`
- **THEN** the page records those references as explicit supporting links
- **AND** those references do not expand the default wiki ingestion boundary

### Requirement: Page authority and freshness metadata

The Ito wiki SHALL make page authority, source coverage, and freshness explicit so agents can decide when to trust a page, warn, or fall back to raw sources.

#### Scenario: Read page metadata

- **WHEN** an agent opens a durable wiki page
- **THEN** the page identifies its page type, authority level, source references, freshness status, and known gaps
- **AND** authority distinguishes at least canonical summaries, advisory syntheses, decision records, and query artifacts
- **AND** freshness distinguishes at least fresh, stale, and unknown states

#### Scenario: Wiki conflicts with raw Ito artifacts

- **WHEN** a wiki page conflicts with a referenced spec, change, module, research artifact, or architecture document
- **THEN** the agent treats the raw Ito artifact as authoritative unless the page authority explicitly identifies a newer decision record
- **AND** the agent records or reports the conflict as a lint/freshness issue

### Requirement: Graph-friendly topic pages

The Ito wiki SHALL prefer durable topic pages with explicit cross-links over one-page-per-artifact mirroring.

#### Scenario: Summarize archived changes into topic pages

- **WHEN** archived changes add durable knowledge to the wiki
- **THEN** the default behavior is to update relevant topic pages with links to archived changes, current specs, modules, research, and relevant documentation
- **AND** individual archived-change pages are created only when the change is historically important or too large to summarize clearly in a topic page

### Requirement: Durable navigation artifacts

The Ito wiki SHALL maintain durable navigation artifacts that let an LLM and a human browse the knowledge layer without re-deriving structure from raw sources on every query.

#### Scenario: Navigate the wiki through index and status

- **WHEN** an agent or user opens `.ito/wiki/index.md` and `.ito/wiki/_meta/status.md`
- **THEN** they can discover the major page groups, current freshness state, notable coverage gaps, and high-value topic pages
- **AND** `log.md` provides an append-only timeline of wiki operations
- **AND** the wiki schema describes expected page types, authority metadata, freshness metadata, and cross-linking conventions
