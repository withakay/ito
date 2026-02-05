## ADDED Requirements

### Requirement: Bootstrap Instruction Artifact

The CLI SHALL support a `bootstrap` artifact in `ito agent instruction` that provides tool-specific preambles for agent adapters.

#### Scenario: Bootstrap artifact with tool flag
- **GIVEN** the user runs `ito agent instruction bootstrap --tool opencode`
- **WHEN** the command executes
- **THEN** it SHALL output OpenCode-specific tool mapping notes
- **AND** it SHALL output pointers to workflow instruction artifacts

#### Scenario: Supported tools
- **GIVEN** the user runs `ito agent instruction bootstrap --tool <tool>`
- **WHEN** `<tool>` is one of `opencode`, `claude`, `codex`
- **THEN** the command SHALL output tool-specific content

#### Scenario: Bootstrap content structure
- **GIVEN** the bootstrap artifact is generated
- **WHEN** rendered for any tool
- **THEN** it SHALL contain tool mapping notes (where tools differ)
- **AND** it SHALL contain "how to get workflow bodies" pointers
- **AND** it SHALL NOT contain full workflow text

### Requirement: Tool-Specific Content

The bootstrap artifact SHALL provide content tailored to each supported tool's capabilities.

#### Scenario: OpenCode-specific content
- **GIVEN** the tool is `opencode`
- **WHEN** bootstrap content is generated
- **THEN** it SHALL include MCP tool names and parallel invocation patterns

#### Scenario: Claude Code-specific content
- **GIVEN** the tool is `claude`
- **WHEN** bootstrap content is generated
- **THEN** it SHALL include Task tool delegation and Read/Write/Edit tool usage notes

#### Scenario: Codex-specific content
- **GIVEN** the tool is `codex`
- **WHEN** bootstrap content is generated
- **THEN** it SHALL include available commands and shell execution patterns
