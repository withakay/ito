# Interactive Module Selection Specification

## Purpose

Define the `interactive-module-selection` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Update ito-proposal skill

The `ito-proposal` skill file SHALL be updated to include the interactive module selection flow.

#### Scenario: Skill includes prompt step

- **WHEN** reading `.opencode/skills/ito-proposal/SKILL.md`
- **THEN** step 3 includes logic for prompting when module not specified

#### Scenario: Skill documents all three options

- **WHEN** reading skill documentation
- **THEN** all three module selection options are documented
