<!-- ITO:START -->
## ADDED Requirements

### Requirement: Cleanup agent instruction artifact

The system SHALL provide an `ito agent instruction cleanup` artifact that generates a comprehensive cleanup guide for agents.

- **Requirement ID**: cleanup-instruction:agent-instruction-artifact

#### Scenario: Agent runs cleanup instruction with no legacy files

- **WHEN** an agent runs `ito agent instruction cleanup`
- **AND** the repo has no orphaned or legacy Ito files
- **THEN** the instruction output SHALL list all currently installed Ito-managed files
- **AND** the output SHALL report "no legacy files detected"

#### Scenario: Agent runs cleanup instruction with legacy files present

- **WHEN** an agent runs `ito agent instruction cleanup`
- **AND** the repo contains orphaned files from previous Ito versions
- **THEN** the instruction output SHALL list all currently installed Ito-managed files
- **AND** the output SHALL list all detected legacy/orphaned files with their paths
- **AND** the output SHALL provide removal instructions for each orphaned file

### Requirement: Dynamic manifest generation

The instruction artifact SHALL dynamically generate the list of expected Ito-managed files based on the project's configured harness tools and embedded template assets.

- **Requirement ID**: cleanup-instruction:dynamic-manifest

#### Scenario: Manifest reflects configured tools

- **WHEN** a project is configured with `opencode` and `claude` harnesses
- **THEN** the manifest SHALL include files for `.opencode/` and `.claude/` directories
- **AND** the manifest SHALL NOT include files for unconfigured harnesses (e.g., `.codex/`, `.pi/`)

#### Scenario: Manifest stays in sync with template changes

- **WHEN** a new skill is added to `ito-templates` embedded assets
- **AND** `ito init --upgrade` is run
- **THEN** the cleanup instruction SHALL include the new skill in its expected manifest
- **AND** any previous version of that skill path SHALL appear in the legacy list if it was renamed

### Requirement: Legacy file registry

The `ito-templates` crate SHALL embed a structured registry of known legacy file paths — files that were renamed, removed, or relocated across Ito versions.

- **Requirement ID**: cleanup-instruction:legacy-registry

#### Scenario: Registry contains renamed skills

- **WHEN** the legacy registry is queried
- **THEN** it SHALL include entries for skills that were renamed (e.g., `ito-apply-change-proposal` -> `ito-apply`, `ito-write-change-proposal` -> `ito-proposal`)
- **AND** each entry SHALL specify the old path, the new path (if applicable), and the version in which the change occurred

#### Scenario: Registry contains removed skills

- **WHEN** the legacy registry is queried
- **THEN** it SHALL include entries for skills that were completely removed (e.g., `ito-dispatching-parallel-agents`, `ito-finishing-a-development-branch`, `ito-receiving-code-review`, `ito-requesting-code-review`, `ito-systematic-debugging`, `ito-test-driven-development`, `ito-writing-skills`)

#### Scenario: Registry contains removed planning directories

- **WHEN** the legacy registry is queried
- **THEN** it SHALL include entries for the legacy `.ito/planning/` directory and its files (`PROJECT.md`, `ROADMAP.md`, `STATE.md`)

#### Scenario: Registry contains directory renames

- **WHEN** the legacy registry is queried
- **THEN** it SHALL include entries for singular-to-plural directory migrations (e.g., `.opencode/command/` -> `.opencode/commands/`, `.opencode/agent/` -> `.opencode/agents/`)

### Requirement: Instruction output format

The cleanup instruction SHALL output structured markdown that an agent can follow step-by-step.

- **Requirement ID**: cleanup-instruction:output-format

#### Scenario: Instruction includes scanning commands

- **WHEN** the cleanup instruction is rendered
- **THEN** it SHALL include concrete shell commands (using `ls`, `find`, or similar) for the agent to detect orphaned files
- **AND** it SHALL include the expected file list for comparison

#### Scenario: Instruction includes confirmation gate

- **WHEN** the cleanup instruction lists files for removal
- **THEN** it SHALL instruct the agent to present the list to the user and wait for confirmation before deleting anything

#### Scenario: JSON output mode

- **WHEN** `ito agent instruction cleanup --json` is run
- **THEN** the output SHALL be valid JSON containing the manifest and legacy registry data
<!-- ITO:END -->
