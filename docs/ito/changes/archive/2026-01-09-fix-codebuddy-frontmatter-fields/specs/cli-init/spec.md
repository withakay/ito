## MODIFIED Requirements

### Requirement: Slash Command Configuration

The init command SHALL generate slash command files for supported editors using shared templates.

#### Scenario: Generating slash commands for Antigravity

- **WHEN** the user selects Antigravity during initialization
- **THEN** create `.agent/workflows/ito-proposal.md`, `.agent/workflows/ito-apply.md`, and `.agent/workflows/ito-archive.md`
- **AND** ensure each file begins with YAML frontmatter that contains only a `description: <stage summary>` field followed by the shared Ito workflow instructions wrapped in managed markers
- **AND** populate the workflow body with the same proposal/apply/archive guidance used for other tools so Antigravity behaves like Windsurf while pointing to the `.agent/workflows/` directory

#### Scenario: Generating slash commands for Claude Code

- **WHEN** the user selects Claude Code during initialization
- **THEN** create `.claude/commands/ito/proposal.md`, `.claude/commands/ito/apply.md`, and `.claude/commands/ito/archive.md`
- **AND** populate each file from shared templates so command text matches other tools
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for CodeBuddy Code

- **WHEN** the user selects CodeBuddy Code during initialization
- **THEN** create `.codebuddy/commands/ito/proposal.md`, `.codebuddy/commands/ito/apply.md`, and `.codebuddy/commands/ito/archive.md`
- **AND** populate each file from shared templates that include CodeBuddy-compatible YAML frontmatter for the `description` and `argument-hint` fields
- **AND** use square bracket format for `argument-hint` parameters (e.g., `[change-id]`)
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for Cline

- **WHEN** the user selects Cline during initialization
- **THEN** create `.clinerules/workflows/ito-proposal.md`, `.clinerules/workflows/ito-apply.md`, and `.clinerules/workflows/ito-archive.md`
- **AND** populate each file from shared templates so command text matches other tools
- **AND** include Cline-specific Markdown heading frontmatter
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for Crush

- **WHEN** the user selects Crush during initialization
- **THEN** create `.crush/commands/ito/proposal.md`, `.crush/commands/ito/apply.md`, and `.crush/commands/ito/archive.md`
- **AND** populate each file from shared templates so command text matches other tools
- **AND** include Crush-specific frontmatter with Ito category and tags
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for Cursor

- **WHEN** the user selects Cursor during initialization
- **THEN** create `.cursor/commands/ito-proposal.md`, `.cursor/commands/ito-apply.md`, and `.cursor/commands/ito-archive.md`
- **AND** populate each file from shared templates so command text matches other tools
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for Factory Droid

- **WHEN** the user selects Factory Droid during initialization
- **THEN** create `.factory/commands/ito-proposal.md`, `.factory/commands/ito-apply.md`, and `.factory/commands/ito-archive.md`
- **AND** populate each file from shared templates that include Factory-compatible YAML frontmatter for the `description` and `argument-hint` fields
- **AND** include the `$ARGUMENTS` placeholder in the template body so droid receives any user-supplied input
- **AND** wrap the generated content in Ito managed markers so `ito update` can safely refresh the commands

#### Scenario: Generating slash commands for OpenCode

- **WHEN** the user selects OpenCode during initialization
- **THEN** create `.opencode/commands/ito-proposal.md`, `.opencode/commands/ito-apply.md`, and `.opencode/commands/ito-archive.md`
- **AND** populate each file from shared templates so command text matches other tools
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for Windsurf

- **WHEN** the user selects Windsurf during initialization
- **THEN** create `.windsurf/workflows/ito-proposal.md`, `.windsurf/workflows/ito-apply.md`, and `.windsurf/workflows/ito-archive.md`
- **AND** populate each file from shared templates (wrapped in Ito markers) so workflow text matches other tools
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for Kilo Code

- **WHEN** the user selects Kilo Code during initialization
- **THEN** create `.kilocode/workflows/ito-proposal.md`, `.kilocode/workflows/ito-apply.md`, and `.kilocode/workflows/ito-archive.md`
- **AND** populate each file from shared templates (wrapped in Ito markers) so workflow text matches other tools
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for Codex

- **WHEN** the user selects Codex during initialization
- **THEN** create global prompt files at `~/.codex/prompts/ito-proposal.md`, `~/.codex/prompts/ito-apply.md`, and `~/.codex/prompts/ito-archive.md` (or under `$CODEX_HOME/prompts` if set)
- **AND** populate each file from shared templates that map the first numbered placeholder (`$1`) to the primary user input (e.g., change identifier or question text)
- **AND** wrap the generated content in Ito markers so `ito update` can refresh the prompts without touching surrounding custom notes
