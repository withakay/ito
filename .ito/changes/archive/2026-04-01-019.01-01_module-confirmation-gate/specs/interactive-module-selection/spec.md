## MODIFIED Requirements

### Requirement: Update ito-proposal skill

The `ito-proposal` skill file SHALL be updated to include the interactive module selection flow as an explicit confirmation gate that requires user response before creating change scaffolding.

#### Scenario: Skill includes mandatory confirmation step

- **WHEN** reading `.opencode/skills/ito-proposal/SKILL.md`
- **THEN** the module selection step requires explicit user confirmation before proceeding to `ito create change`
- **AND** the agent SHALL NOT proceed to create change scaffolding until the user has confirmed or provided a module choice

#### Scenario: Skill documents all module selection options

- **WHEN** reading skill documentation
- **THEN** all module selection options are documented:
  1. Use an existing module
  2. Create a new top-level module
  3. Create a new sub-module under an existing module

#### Scenario: Skill documents sub-module ID format

- **WHEN** reading skill documentation
- **THEN** the sub-module ID format (`NNN.SS`) is explained
- **AND** the commands for creating and using sub-modules are documented (`ito create sub-module`, `--sub-module` flag)

## ADDED Requirements

### Requirement: New-proposal instruction template enforces module confirmation gate

The `new-proposal.md.j2` instruction template SHALL present module selection as a mandatory confirmation gate that blocks scaffolding creation until the user explicitly confirms their choice.

#### Scenario: Agent presents module options and waits for confirmation

- **WHEN** the agent follows the new-proposal instructions
- **THEN** it SHALL run `ito list --modules` to display available modules (including sub-modules)
- **AND** it SHALL present the user with three clear options:
  1. Use an existing module (specify which one)
  2. Create a new module (enter a name)
  3. Create a new sub-module under an existing module (specify parent and name)
- **AND** it SHALL wait for the user to confirm before running `ito create change`

#### Scenario: Sub-module creation is offered as an option

- **WHEN** modules already exist in the project
- **THEN** the instruction template explains that the user can create a sub-module under any existing module
- **AND** it provides the `ito create sub-module <name> --module <parent-id>` command
- **AND** it explains the `--sub-module <NNN.SS>` flag for `ito create change`

#### Scenario: Agent does not default silently to module 000

- **WHEN** the user has not explicitly chosen a module
- **THEN** the agent SHALL NOT silently default to module `000`
- **AND** it SHALL ask the user to confirm or choose a module first
