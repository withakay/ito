<!-- ITO:START -->
## REMOVED Requirements

### Requirement: Workflow skill loaded by convention
The orchestrator SHALL load an `ito-orchestrator-workflow` skill by convention when it exists.

**Reason**: A generated workflow skill is another discoverable activation surface and can drift from authoritative instruction policy.
**Migration**: `ito-loop` and native orchestrator agents load `ito agent instruction orchestrate`; project-specific policy remains in `.ito/user-prompts/orchestrate.md`.

#### Scenario: Orchestration loads authoritative instructions
- **WHEN** iterative or multi-change orchestration begins
- **THEN** the lifecycle workflow renders `ito agent instruction orchestrate`
- **AND** it does not search for `ito-orchestrator-workflow`

### Requirement: Workflow skill is living documentation
The generated `ito-orchestrator-workflow` skill SHALL serve as editable living documentation for repository orchestration conventions.

**Reason**: Project-specific living policy belongs in the user-prompt layer, while stable orchestration policy belongs in emitted instruction artifacts.
**Migration**: Preserve repository conventions in `.ito/user-prompts/orchestrate.md`; do not generate a project skill.

#### Scenario: Project policy remains editable
- **WHEN** a repository customizes orchestration conventions
- **THEN** it edits `.ito/user-prompts/orchestrate.md`
- **AND** instruction rendering composes that guidance without creating a skill directory
<!-- ITO:END -->
