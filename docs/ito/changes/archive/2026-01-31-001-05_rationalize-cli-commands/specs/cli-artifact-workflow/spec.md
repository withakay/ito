## MODIFIED Requirements

### Requirement: Status Command

The system SHALL display artifact completion status for a change, including scaffolded (empty) changes.

> **Fixes bug**: Previously required `proposal.md` to exist via `getActiveChangeIds()`.

#### Scenario: Show status with all states

- **WHEN** user runs `ito status --change <id>`
- **THEN** the system displays each artifact with status indicator:
  - `[x]` for completed artifacts
  - `[ ]` for ready artifacts
  - `[-]` for blocked artifacts (with missing dependencies listed)

#### Scenario: Status shows completion summary

- **WHEN** user runs `ito status --change <id>`
- **THEN** output includes completion percentage and count (e.g., "2/4 artifacts complete")

#### Scenario: Status JSON output

- **WHEN** user runs `ito status --change <id> --json`
- **THEN** the system outputs JSON with changeName, schemaName, isComplete, and artifacts array

#### Scenario: Status JSON includes apply requirements

- **WHEN** user runs `ito status --change <id> --json`
- **THEN** the system outputs JSON with:
  - `changeName`, `schemaName`, `isComplete`, `artifacts` array
  - `applyRequires`: array of artifact IDs needed for apply phase

#### Scenario: Status on scaffolded change

- **WHEN** user runs `ito status --change <id>` on a change with no artifacts
- **THEN** system displays all artifacts with their status
- **AND** root artifacts (no dependencies) show as ready `[ ]`
- **AND** dependent artifacts show as blocked `[-]`

#### Scenario: Missing change parameter

- **WHEN** user runs `ito status` without `--change`
- **THEN** the system displays an error with list of available changes
- **AND** includes scaffolded changes (directories without proposal.md)

#### Scenario: Unknown change

- **WHEN** user runs `ito status --change unknown-id`
- **AND** directory `ito/changes/unknown-id/` does not exist
- **THEN** the system displays an error listing all available change directories

### Requirement: Instructions Command

The system SHALL output enriched instructions for creating an artifact, including for scaffolded changes.

#### Scenario: Show enriched instructions

- **WHEN** user runs `ito x-instructions <artifact> --change <id>`
- **THEN** the system outputs:
  - Artifact metadata (ID, output path, description)
  - Template content
  - Dependency status (done/missing)
  - Unlocked artifacts (what becomes available after completion)

#### Scenario: Instructions JSON output

- **WHEN** user runs `ito x-instructions <artifact> --change <id> --json`
- **THEN** the system outputs JSON matching ArtifactInstructions interface

#### Scenario: Unknown artifact

- **WHEN** user runs `ito x-instructions unknown-artifact --change <id>`
- **THEN** the system displays an error listing valid artifact IDs for the schema

#### Scenario: Artifact with unmet dependencies

- **WHEN** user requests instructions for a blocked artifact
- **THEN** the system displays instructions with a warning about missing dependencies

#### Scenario: Instructions on scaffolded change

- **WHEN** user runs `ito x-instructions proposal --change <id>` on a scaffolded change
- **THEN** system outputs template and metadata for creating the proposal
- **AND** does not require any artifacts to already exist

### Requirement: Templates Command

The system SHALL show resolved template paths for all artifacts in a schema.

#### Scenario: List template paths with default schema

- **WHEN** user runs `ito x-templates`
- **THEN** the system displays each artifact with its resolved template path using the default schema

#### Scenario: List template paths with custom schema

- **WHEN** user runs `ito x-templates --schema tdd`
- **THEN** the system displays template paths for the specified schema

#### Scenario: Templates JSON output

- **WHEN** user runs `ito x-templates --json`
- **THEN** the system outputs JSON mapping artifact IDs to template paths

#### Scenario: Template resolution source

- **WHEN** displaying template paths
- **THEN** the system indicates whether each template is from user override or package built-in

### Requirement: New Change Command

The system SHALL create new change directories with validation.

#### Scenario: Create valid change

- **WHEN** user runs `ito create change add-feature`
- **THEN** the system creates `.ito/changes/add-feature/` directory

#### Scenario: Invalid change name

- **WHEN** user runs `ito create change "Add Feature"` with invalid name
- **THEN** the system displays validation error with guidance

#### Scenario: Duplicate change name

- **WHEN** user runs `ito create change existing-change` for an existing change
- **THEN** the system displays an error indicating the change already exists

#### Scenario: Create with description

- **WHEN** user runs `ito create change add-feature --description "Add new feature"`
- **THEN** the system creates the change directory with description in README.md

### Requirement: Schema Selection

The system SHALL support custom schema selection for workflow commands.

#### Scenario: Default schema

- **WHEN** user runs workflow commands without `--schema`
- **THEN** the system uses the "spec-driven" schema

#### Scenario: Custom schema

- **WHEN** user runs `ito status --change <id> --schema tdd`
- **THEN** the system uses the specified schema for artifact graph

#### Scenario: Unknown schema

- **WHEN** user specifies an unknown schema
- **THEN** the system displays an error listing available schemas

### Requirement: Experimental Isolation

The system SHALL implement artifact workflow commands in isolation for easy removal.

#### Scenario: Single file implementation

- **WHEN** artifact workflow feature is implemented
- **THEN** all commands are in `src/commands/artifact-workflow.ts`

#### Scenario: Help text marking

-- **WHEN** user runs `--help` on any `x-*` artifact workflow command
-- **THEN** help text indicates the command is experimental

#### Scenario: Command naming indicates experimental

-- **WHEN** experimental artifact workflow commands are exposed
-- **THEN** they are named using the `x-` prefix (e.g., `ito x-instructions`)

### Requirement: Apply Instructions Command

The system SHALL generate schema-aware apply instructions via `ito x-instructions apply`.

#### Scenario: Generate apply instructions

-- **WHEN** user runs `ito x-instructions apply --change <id>`

- **AND** all required artifacts (per schema's `apply.requires`) exist
- **THEN** the system outputs:
  - Context files from all existing artifacts
  - Schema-specific instruction text
  - Progress tracking file path (if `apply.tracks` is set)

#### Scenario: Apply blocked by missing artifacts

-- **WHEN** user runs `ito x-instructions apply --change <id>`

- **AND** required artifacts are missing
- **THEN** the system indicates apply is blocked
- **AND** lists which artifacts must be created first

#### Scenario: Apply instructions JSON output

-- **WHEN** user runs `ito x-instructions apply --change <id> --json`

- **THEN** the system outputs JSON with:
  - `contextFiles`: array of paths to existing artifacts
  - `instruction`: the apply instruction text
  - `tracks`: path to progress file or null
  - `applyRequires`: list of required artifact IDs
