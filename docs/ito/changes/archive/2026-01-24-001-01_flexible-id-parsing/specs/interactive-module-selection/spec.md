## ADDED Requirements

### Requirement: Prompt for module when not specified

When `/ito-proposal` is invoked without a module ID, the skill SHALL prompt the user with module selection options.

#### Scenario: No module specified triggers prompt

- **WHEN** user runs `/ito-proposal` without specifying a module
- **THEN** system presents interactive question with module options

#### Scenario: Module specified skips prompt

- **WHEN** user runs `/ito-proposal --module 001`
- **THEN** system uses specified module without prompting

### Requirement: Offer three module selection choices

The module selection prompt SHALL offer three choices: last worked-on module, create new module, or ungrouped.

#### Scenario: Option to use last worked-on module

- **WHEN** module selection prompt is displayed
- **THEN** first option is "Use last worked-on module: NNN_name" (if one exists)

#### Scenario: Option to create new module

- **WHEN** module selection prompt is displayed
- **THEN** second option is "Create a new module"

#### Scenario: Option for ungrouped change

- **WHEN** module selection prompt is displayed
- **THEN** third option is "Ungrouped (module 000)"

### Requirement: Track last worked-on module

The system SHALL track and retrieve the last module a user worked on.

#### Scenario: Last module stored after change creation

- **WHEN** user creates a change in module 001
- **THEN** system records 001 as last worked-on module

#### Scenario: Last module retrieved for prompt

- **WHEN** displaying module selection prompt
- **THEN** system retrieves and displays last worked-on module name

#### Scenario: No last module available

- **WHEN** no previous module work exists
- **THEN** "Use last worked-on module" option is not shown or marked as unavailable

### Requirement: Handle new module creation flow

When user selects "Create a new module", the system SHALL prompt for the module name and create it.

#### Scenario: New module name prompt

- **WHEN** user selects "Create a new module"
- **THEN** system prompts for module name

#### Scenario: Module created with provided name

- **WHEN** user provides module name "my-feature"
- **THEN** system runs `ito create module "my-feature"` and uses resulting ID

### Requirement: Update ito-proposal skill

The `ito-proposal` skill file SHALL be updated to include the interactive module selection flow.

#### Scenario: Skill includes prompt step

- **WHEN** reading `.opencode/skill/ito-proposal/SKILL.md`
- **THEN** step 3 includes logic for prompting when module not specified

#### Scenario: Skill documents all three options

- **WHEN** reading skill documentation
- **THEN** all three module selection options are documented
