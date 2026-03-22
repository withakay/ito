<!-- ITO:START -->
## ADDED Requirements

### Requirement: A sweep prompt exists for detecting hardcoded old-format IDs in repository artifacts

Ito SHALL provide an agent-facing sweep prompt that guides an agent to scan an Ito repository for hardcoded old-format IDs (`NNN-NN_name` or bare `NNN` module IDs) embedded in artifact files, and report findings with upgrade guidance.

The sweep is read-only by default — it reports findings; it does not automatically rewrite files.

#### Scenario: Sweep prompt is accessible via ito agent instruction

- **WHEN** user executes `ito agent instruction repo-sweep`
- **THEN** the CLI prints the repo-sweep prompt as a usable agent instruction artifact
- **AND** the prompt instructs the agent to scan `.ito/` artifact files (proposals, specs, tasks, user-prompts, module.md files)

#### Scenario: Sweep prompt instructs scanning for embedded change IDs

- **WHEN** an agent follows the sweep prompt
- **THEN** the agent scans files under `.ito/changes/`, `.ito/modules/`, and `.ito/user-prompts/` for patterns matching old-format IDs embedded in prose or YAML front matter
- **AND** reports: file path, line number, matched pattern, and suggested replacement if a sub-module mapping is known

#### Scenario: Sweep prompt provides upgrade guidance

- **WHEN** the sweep prompt is rendered
- **THEN** it includes instructions for how to update found references using `ito create sub-module` and `ito create change --sub-module`
- **AND** it notes that existing IDs do not need to be renamed — only newly created changes under a sub-module use the new format

### Requirement: Sweep prompt is installed as a template asset

The sweep prompt SHALL be an embedded template asset installed by `ito init` under `.ito/user-prompts/` or accessible via `ito agent instruction repo-sweep` without requiring a change context.

#### Scenario: Sweep prompt accessible without a --change flag

- **WHEN** user executes `ito agent instruction repo-sweep` with no `--change` argument
- **THEN** the command succeeds and prints the sweep prompt
- **AND** does not require an active change to be set
<!-- ITO:END -->
