## ADDED Requirements

### Requirement: Codex Bootstrap Snippet

The system SHALL provide a bootstrap snippet for Codex that delegates workflow content to the Ito CLI.

#### Scenario: Bootstrap snippet points to CLI
- **GIVEN** the bootstrap snippet is installed to `~/.codex/instructions/ito-skills-bootstrap.md`
- **WHEN** a Codex agent session starts
- **THEN** the snippet SHALL point to `ito agent instruction <artifact>` for all workflow content

#### Scenario: Bootstrap snippet lists available artifacts
- **GIVEN** the bootstrap snippet is rendered
- **WHEN** an agent reads it
- **THEN** it SHALL provide a quick reference of available instruction artifacts (proposal, specs, design, tasks, apply, review, archive)

#### Scenario: Bootstrap snippet is concise
- **GIVEN** the bootstrap snippet content
- **WHEN** measured
- **THEN** it SHALL NOT exceed 20 lines of text
- **AND** it SHALL NOT embed full workflow instructions

### Requirement: Deprecate Node CLI Skill Runner

The system SHALL deprecate the Node CLI skill runner (`ito-skills/.codex/ito-skills-codex`) in favor of the bootstrap snippet approach.

#### Scenario: Node CLI marked deprecated
- **GIVEN** the Node CLI skill runner exists
- **WHEN** documentation is updated
- **THEN** it SHALL be marked as deprecated
- **AND** users SHALL be directed to the bootstrap snippet approach
