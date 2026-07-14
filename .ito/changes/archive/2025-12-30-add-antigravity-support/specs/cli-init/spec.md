# Delta for CLI Init

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
- **AND** populate each file from shared templates so command text matches other tools
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for Cline

- **WHEN** the user selects Cline during initialization
- **THEN** create `.clinerules/ito-proposal.md`, `.clinerules/ito-apply.md`, and `.clinerules/ito-archive.md`
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

#### Scenario: Generating slash commands for GitHub Copilot

- **WHEN** the user selects GitHub Copilot during initialization
- **THEN** create `.github/prompts/ito-proposal.prompt.md`, `.github/prompts/ito-apply.prompt.md`, and `.github/prompts/ito-archive.prompt.md`
- **AND** populate each file with YAML frontmatter containing a `description` field that summarizes the workflow stage
- **AND** include `$ARGUMENTS` placeholder to capture user input
- **AND** wrap the shared template body with Ito markers so `ito update` can refresh the content
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for Gemini CLI

- **WHEN** the user selects Gemini CLI during initialization
- **THEN** create `.gemini/commands/ito/proposal.toml`, `.gemini/commands/ito/apply.toml`, and `.gemini/commands/ito/archive.toml`
- **AND** populate each file as TOML that sets a stage-specific `description = "<summary>"` and a multi-line `prompt = """` block with the shared Ito template
- **AND** wrap the Ito managed markers (`<!-- ITO:START -->` / `<!-- ITO:END -->`) inside the `prompt` value so `ito update` can safely refresh the body between markers without touching the TOML framing
- **AND** ensure the slash-command copy matches the existing proposal/apply/archive templates used by other tools

#### Scenario: Generating slash commands for iFlow CLI

- **WHEN** the user selects iFlow CLI during initialization
- **THEN** create `.iflow/commands/ito-proposal.md`, `.iflow/commands/ito-apply.md`, and `.iflow/commands/ito-archive.md`
- **AND** populate each file from shared templates so command text matches other tools
- **AND** include YAML frontmatter with `name`, `id`, `category`, and `description` fields for each command
- **AND** wrap the generated content in Ito managed markers so `ito update` can safely refresh the commands
- **AND** each template includes instructions for the relevant Ito workflow stage

#### Scenario: Generating slash commands for RooCode

- **WHEN** the user selects RooCode during initialization
- **THEN** create `.roo/commands/ito-proposal.md`, `.roo/commands/ito-apply.md`, and `.roo/commands/ito-archive.md`
- **AND** populate each file from shared templates so command text matches other tools
- **AND** include simple Markdown headings (e.g., `# Ito: Proposal`) without YAML frontmatter
- **AND** wrap the generated content in Ito managed markers where applicable so `ito update` can safely refresh the commands
- **AND** each template includes instructions for the relevant Ito workflow stage
