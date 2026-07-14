## ADDED Requirements

### Requirement: Experimental Workflow Slash Commands

The system SHALL expose the experimental workflow via hyphenated `/ito-*` slash commands and SHALL NOT use `/opsx:*`.

#### Scenario: Listing experimental workflow commands

- **WHEN** `ito artifact-experimental-setup` completes successfully
- **THEN** the output lists the experimental commands:
  - `/ito-explore`
  - `/ito-new-change`
  - `/ito-continue-change`
  - `/ito-apply-change`
  - `/ito-ff-change`
  - `/ito-sync-specs`
  - `/ito-archive-change`

### Requirement: Claude Command File Generation

The system SHALL generate Claude command wrapper files as flat files under `.claude/commands/` using the `ito-*.md` naming convention.

#### Scenario: Generating experimental workflow commands for Claude Code

- **WHEN** `ito artifact-experimental-setup` runs
- **THEN** it creates `.claude/commands/ito-explore.md`, `.claude/commands/ito-new-change.md`, `.claude/commands/ito-continue-change.md`, `.claude/commands/ito-apply-change.md`, `.claude/commands/ito-ff-change.md`, `.claude/commands/ito-sync-specs.md`, and `.claude/commands/ito-archive-change.md`
