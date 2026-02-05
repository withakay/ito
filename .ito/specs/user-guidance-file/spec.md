# User Guidance File Specification

## Purpose

Define the `user-guidance-file` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Project-local guidance file

Ito SHALL support a project-local Markdown file that users can edit to provide additional guidance for LLM-driven workflows.

#### Scenario: File created during init

- **WHEN** a user runs `ito init` in a project
- **THEN** Ito creates `.ito/user-guidance.md` if it does not exist
- **AND** the file explains how to add guidance

#### Scenario: User edits are preserved

- **GIVEN** `.ito/user-guidance.md` already exists and contains user-authored content
- **WHEN** a user runs `ito update`
- **THEN** Ito MUST NOT overwrite user-authored content

### Requirement: Managed header block

The guidance file SHALL contain a managed header block that Ito may update over time without impacting user-authored guidance.

#### Scenario: Managed block can be updated

- **GIVEN** `.ito/user-guidance.md` contains a `<!-- ITO:START -->` managed block
- **WHEN** Ito updates templates
- **THEN** only the managed block content is updated
- **AND** user-authored content outside the managed block is preserved
