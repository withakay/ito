# CLI Plan Specification

## Purpose

The `ito plan` command group provides project-level planning functionality, enabling teams to manage PROJECT.md, ROADMAP.md, and STATE.md files that persist across sessions and provide context for long-running AI-assisted development.

## Requirements

### Requirement: Project planning initialization

The CLI SHALL initialize the `.ito/planning/` directory structure with template files for project vision, roadmapping, and state tracking.

#### Scenario: Initialize planning directory

- **WHEN** executing `ito plan init`
- **THEN** create the `.ito/planning/` directory if it does not exist
- **AND** create `PROJECT.md` template with sections for vision, value proposition, constraints, stakeholders, and out-of-scope items
- **AND** create `ROADMAP.md` template with milestone and phase structure
- **AND** create `STATE.md` template with sections for current focus, decisions, blockers, and session notes
- **AND** display a success message indicating the planning structure has been initialized
- **AND** skip creating any files that already exist to preserve existing content

### Requirement: Planning status display

The CLI SHALL display the current state of project planning artifacts, including which documents exist and their last modification times.

#### Scenario: Show planning status

- **WHEN** executing `ito plan status`
- **THEN** check for existence of `.ito/planning/PROJECT.md`, `.ito/planning/ROADMAP.md`, and `.ito/planning/STATE.md`
- **AND** display a table showing each file's status (exists/missing) and last modified timestamp
- **AND** indicate the current milestone from ROADMAP.md if it exists
- **AND** show the current focus from STATE.md if it exists
- **AND** print a hint to run `ito plan init` if any files are missing

### Requirement: Project state management

The CLI SHALL provide commands to record and display project state information, including decisions, blockers, and session notes.

#### Scenario: Show current state

- **WHEN** executing `ito state` or `ito state show`
- **THEN** read `.ito/planning/STATE.md` if it exists
- **AND** display the contents in a human-readable format
- **AND** print an error message and suggest running `ito plan init` if STATE.md does not exist

#### Scenario: Record a decision

- **WHEN** executing `ito state decision "[decision text]"`
- **THEN** append a decision entry to the "Recent Decisions" section in STATE.md with the current timestamp
- **AND** create STATE.md if it does not exist using the template
- **AND** display a confirmation that the decision has been recorded

#### Scenario: Record a blocker

- **WHEN** executing `ito state blocker "[blocker text]"`
- **THEN** append a blocker entry to the "Blockers" section in STATE.md with the current timestamp
- **AND** create STATE.md if it does not exist using the template
- **AND** display a confirmation that the blocker has been recorded

#### Scenario: Add a session note

- **WHEN** executing `ito state note "[note text]"`
- **THEN** append a note entry to the "Session Notes" section in STATE.md with the current date as a header
- **AND** create STATE.md if it does not exist using the template
- **AND** display a confirmation that the note has been added

### Requirement: Roadmap milestone management

The CLI SHALL provide commands to create milestones and phases in the project roadmap.

#### Scenario: Create a milestone

- **WHEN** executing `ito plan milestone [milestone-name]`
- **THEN** append a new milestone section to `.ito/planning/ROADMAP.md` with the provided name
- **AND** include a placeholder target date and empty phase table
- **AND** create ROADMAP.md if it does not exist using the template
- **AND** display a confirmation that the milestone has been created
- **AND** update the "Current Milestone" indicator at the top of ROADMAP.md to the new milestone

#### Scenario: Add a phase to a milestone

- **WHEN** executing `ito plan phase [milestone-name] [phase-name]`
- **THEN** add a new row to the phase table for the specified milestone in ROADMAP.md
- **AND** set the status to "Pending" and changes to "-"
- **AND** display a confirmation that the phase has been added
- **AND** print an error if the specified milestone does not exist

### Requirement: Error handling

The CLI SHALL provide clear error messages and recovery suggestions when planning commands encounter issues.

#### Scenario: Planning directory cannot be created

- **WHEN** the `.ito/planning/` directory cannot be created due to permissions or filesystem errors
- **THEN** display an error message explaining the failure
- **AND** suggest checking directory permissions and disk space
- **AND** exit with code 1

#### Scenario: State file is malformed

- **WHEN** STATE.md exists but has unexpected or malformed content
- **THEN** display a warning that the file format may be incorrect
- **AND** attempt to parse and display what content can be read
- **AND** suggest running `ito plan init` to recreate the template

### Requirement: Template quality

The CLI SHALL generate high-quality templates that follow Ito conventions and provide clear guidance for users.

#### Scenario: PROJECT.md template includes required sections

- **WHEN** generating PROJECT.md
- **THEN** include sections for: Vision, Core Value Proposition, Constraints, Stakeholders, Out of Scope, and AI Assistant Notes
- **AND** provide clear guidance and placeholder text for each section
- **AND** follow the format documented in project-planning-research-proposal.md

#### Scenario: ROADMAP.md template includes required structure

- **WHEN** generating ROADMAP.md
- **THEN** include a "Current Milestone" indicator at the top
- **AND** include a "Milestones" section with placeholder milestones
- **AND** structure milestones with name, target, and a phase table (Phase, Name, Status, Changes)
- **AND** follow the format documented in project-planning-research-proposal.md

#### Scenario: STATE.md template includes required sections

- **WHEN** generating STATE.md
- **THEN** include sections for: Current Focus, Recent Decisions, Open Questions, Blockers, Session Notes, and "For AI Assistants"
- **AND** provide clear guidance for AI assistants on how to resume work
- **AND** follow the format documented in project-planning-research-proposal.md

## Why

Project-level planning is essential for long-running AI-assisted development. These commands provide:

1. **Session continuity**: STATE.md persists context across coding sessions
1. **Stakeholder alignment**: PROJECT.md captures vision, constraints, and success criteria
1. **Milestone tracking**: ROADMAP.md provides phased execution plan
1. **Decision history**: Record decisions to avoid revisiting settled issues
1. **Blocker visibility**: Track and communicate obstacles to progress

Without these tools, teams must manually maintain project planning artifacts, leading to lost context, unclear priorities, and difficulty onboarding new AI assistants or team members.
