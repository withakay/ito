<!-- ITO:START -->
## ADDED Requirements

### Requirement: A sweep prompt exists for detecting old-only ID assumptions in prompt and instruction files

Ito SHALL provide an agent-facing sweep prompt that guides an agent to scan an Ito repository for prompts, templates, regexes, and examples that assume only module-level IDs (`NNN-NN_name`) exist, and report findings with upgrade guidance.

The sweep is read-only by default — it reports findings; it does not automatically rewrite files.

#### Scenario: Sweep prompt is accessible via ito agent instruction

- **WHEN** user executes `ito agent instruction repo-sweep`
- **THEN** the CLI prints the repo-sweep prompt as a usable agent instruction artifact
- **AND** the prompt instructs the agent to scan prompt and instruction surfaces such as `.ito/user-prompts/`, `AGENTS.md`, `.opencode/`, `.github/`, `.codex/`, and template-authoring sources like `ito-rs/crates/ito-templates/assets/`

#### Scenario: Sweep prompt instructs scanning for old-only assumptions

- **WHEN** an agent follows the sweep prompt
- **THEN** the agent scans target files for hardcoded regexes, examples, or prose that only mention `NNN-NN_name`
- **AND** reports: file path, line number, assumption type, and a suggested generalization that accepts both `NNN-NN_name` and `NNN.SS-NN_name`

#### Scenario: Sweep prompt provides upgrade guidance

- **WHEN** the sweep prompt is rendered
- **THEN** it includes instructions for how to update found references so prompts/examples remain format-flexible for both module-level and sub-module changes
- **AND** it notes that existing IDs do not need to be renamed

### Requirement: Sweep prompt is installed as a template asset

The sweep prompt SHALL be an embedded template asset installed by `ito init` under `.ito/user-prompts/` or accessible via `ito agent instruction repo-sweep` without requiring a change context.

#### Scenario: Sweep prompt accessible without a --change flag

- **WHEN** user executes `ito agent instruction repo-sweep` with no `--change` argument
- **THEN** the command succeeds and prints the sweep prompt
- **AND** does not require an active change to be set
<!-- ITO:END -->
