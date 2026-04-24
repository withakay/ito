<!-- ITO:START -->
## ADDED Requirements

### Requirement: Orchestrate instruction artifact type

The system SHALL support `orchestrate` as a first-class artifact type for `ito agent instruction`, rendering a complete orchestrator instruction document from the project's `orchestrate.md` user prompt, per-change metadata, and detected run context.

- **Requirement ID**: orchestrate-instruction:artifact-type

#### Scenario: Render orchestrate instruction document

- **WHEN** an agent invokes `ito agent instruction orchestrate`
- **THEN** the system renders `orchestrate.md.j2` injecting the project's `orchestrate.md` user prompt content, resolved change list with `depends_on` and `preferred_gates` per change, detected harness context, and available preset if configured
- **AND** the rendered document is printed to stdout

#### Scenario: Missing orchestrate.md triggers setup guidance

- **WHEN** an agent invokes `ito agent instruction orchestrate` and no `orchestrate.md` exists in the project's user-prompts directory
- **THEN** the system emits a setup guidance message directing the agent to load the `ito-orchestrate-setup` skill before proceeding
- **AND** exits with a non-zero status code

#### Scenario: Harness context injection

- **WHEN** `ito agent instruction orchestrate` is rendered in an OpenCode session
- **THEN** the rendered document includes the detected harness name and available agent role suggestions derived from the active preset
<!-- ITO:END -->
