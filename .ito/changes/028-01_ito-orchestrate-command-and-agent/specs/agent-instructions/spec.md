<!-- ITO:START -->
## ADDED Requirements

### Requirement: Orchestrate artifact type supported by agent instruction command

The CLI SHALL support `ito agent instruction orchestrate` as a valid artifact type, rendering the orchestrator instruction document and printing it to stdout. When `orchestrate.md` is absent, the command SHALL emit setup guidance and exit with a non-zero status.

- **Requirement ID**: agent-instructions:orchestrate-artifact

#### Scenario: Orchestrate instruction rendered successfully

- **WHEN** an agent runs `ito agent instruction orchestrate` and `.ito/user-prompts/orchestrate.md` exists
- **THEN** the system renders `orchestrate.md.j2` and prints the full orchestrator instruction document to stdout
- **AND** exits with status code 0

#### Scenario: Missing orchestrate.md exits with setup guidance

- **WHEN** an agent runs `ito agent instruction orchestrate` and `.ito/user-prompts/orchestrate.md` does not exist
- **THEN** the system prints a message directing the agent to load the `ito-orchestrate-setup` skill
- **AND** exits with a non-zero status code without printing any instruction document
<!-- ITO:END -->
