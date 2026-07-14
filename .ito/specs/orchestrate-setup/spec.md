<!-- ITO:START -->
# Orchestrate Setup

## Purpose

This spec defines the current behavior and requirements for orchestrate setup.

## Requirements

### Requirement: First-run setup detection
The system SHALL detect when no `orchestrate.md` exists in the project's user-prompts directory and emit self-contained setup guidance from the orchestrate instruction flow. It MUST NOT require an `ito-orchestrate-setup` skill.

#### Scenario: Missing orchestrate.md triggers inline guidance
- **WHEN** `ito agent instruction orchestrate` is invoked and `.ito/user-prompts/orchestrate.md` does not exist
- **THEN** the system prints instruction-backed setup guidance
- **AND** it does not direct the agent to load a retired helper skill

#### Scenario: Explicit setup mode uses the same instruction source
- **WHEN** orchestration setup is requested explicitly
- **THEN** the system renders setup guidance from the authoritative instruction/template source
- **AND** no harness-specific setup skill is required

### Requirement: Stack detection
The instruction-backed setup flow SHALL detect the project stack from `Cargo.toml`, `package.json`, `pyproject.toml`, and `go.mod`, using `generic` when no indicator exists.

#### Scenario: Single indicator file detected
- **WHEN** the worktree contains exactly one known stack indicator
- **THEN** setup selects the matching preset without requiring a helper skill

#### Scenario: Multiple or absent indicators
- **WHEN** multiple indicators or no indicators are found
- **THEN** setup asks for confirmation or explains the generic fallback in its emitted guidance

### Requirement: Skill and agent cross-reference
The instruction-backed setup flow MAY cross-reference project/user skills and native agent definitions as optional suggestions. It MUST NOT generate additional Ito-managed skills or auto-wire detected agents.

#### Scenario: External project skill is detected
- **WHEN** a preset recommends a non-Ito project skill that is installed
- **THEN** setup may identify it as available without adding it to Ito's lifecycle inventory

#### Scenario: Recommended skill is missing
- **WHEN** an optional external skill is unavailable
- **THEN** setup reports it as optional and does not block or install it

### Requirement: Setup outputs
The setup flow SHALL produce or update `.ito/user-prompts/orchestrate.md` after explicit confirmation. It MUST NOT create an `ito-orchestrator-workflow` skill or any other skill directory.

#### Scenario: Setup writes project prompt only
- **WHEN** setup completes successfully
- **THEN** `.ito/user-prompts/orchestrate.md` contains the approved project-specific policy
- **AND** no new skill directory is created

#### Scenario: Existing prompt requires confirmation
- **WHEN** the project prompt already exists
- **THEN** setup presents the proposed replacement or patch
- **AND** it requires confirmation before writing
<!-- ITO:END -->
