## ADDED Requirements

### Requirement: Slash Command Configuration

The init command SHALL generate slash command files for supported editors using shared templates.

#### Scenario: Generating slash commands for Claude Code

- **WHEN** the user selects Claude Code during initialization
- **THEN** create `.claude/commands/ito/proposal.md`, `.claude/commands/ito/apply.md`, and `.claude/commands/ito/archive.md`
- **AND** populate each file from shared templates so command text matches other tools
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for Cursor

- **WHEN** the user selects Cursor during initialization
- **THEN** create `.cursor/commands/ito-proposal.md`, `.cursor/commands/ito-apply.md`, and `.cursor/commands/ito-archive.md`
- **AND** populate each file from shared templates so command text matches other tools
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for OpenCode

- **WHEN** the user selects OpenCode during initialization
- **THEN** create `.opencode/commands/ito-proposal.md`, `.opencode/commands/ito-apply.md`, and `.opencode/commands/ito-archive.md`
- **AND** populate each file from shared templates so command text matches other tools
- **AND** each template includes instructions for the relevant Ito workflow stage
