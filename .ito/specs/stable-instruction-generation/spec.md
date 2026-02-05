# Stable Instruction Generation Specification

## Purpose

Define the `stable-instruction-generation` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Instruction generation command under agent namespace

The CLI SHALL provide `ito agent instruction [artifact]` command that generates enriched, context-aware instructions for artifact creation.

#### Scenario: Generate instructions for proposal artifact

- **WHEN** agent runs `ito agent instruction proposal --change "001-01_my-change"`
- **THEN** system outputs XML-formatted instructions containing:
  - Task description for the artifact
  - Output path for the artifact file
  - Template content
  - Dependencies (empty for proposal)
  - What artifacts this unlocks

#### Scenario: Generate instructions with dependency context

- **WHEN** agent runs `ito agent instruction specs --change "001-01_my-change"`
- **AND** proposal.md exists in the change directory
- **THEN** system outputs instructions including:
  - Dependency listing with proposal.md path and status "done"
  - Context section telling agent to read dependency files

#### Scenario: Generate instructions with missing dependency

- **WHEN** agent runs `ito agent instruction design --change "001-01_my-change"`
- **AND** proposal.md does NOT exist
- **THEN** system outputs instructions with dependency status "missing"
- **AND** includes warning that dependency is not complete

### Requirement: JSON output option

The command SHALL support `--json` flag for structured output.

#### Scenario: JSON output format

- **WHEN** agent runs `ito agent instruction specs --change "001-01_my-change" --json`
- **THEN** system outputs valid JSON containing all instruction fields
- **AND** output can be parsed by standard JSON parsers

### Requirement: Schema option for non-default schemas

The command SHALL support `--schema` option to specify alternate workflow schemas.

#### Scenario: Custom schema override

- **WHEN** agent runs `ito agent instruction proposal --change "001-01_my-change" --schema minimal`
- **THEN** system loads template from the `minimal` schema
- **AND** generates instructions according to that schema's artifact graph

### Requirement: Error handling for invalid artifacts

The command SHALL provide clear error messages when requesting invalid artifact types.

#### Scenario: Invalid artifact name

- **WHEN** agent runs `ito agent instruction invalid-artifact --change "001-01_my-change"`
- **THEN** system displays error message listing valid artifact names for the schema
- **AND** exits with non-zero status code
