## Purpose

Merge `writing-plans` skill into `ito-proposal` and remove the duplicate skill. Enhance `ito-proposal` with valuable task authoring patterns.

## ADDED Requirements

### Requirement: ito-proposal includes task granularity guidance

The `ito-proposal` skill SHALL guide users to create bite-sized tasks.

#### Scenario: Task size guidance

- **WHEN** `ito-proposal` generates tasks
- **THEN** it advises tasks should be 2-5 minutes of work each
- **AND** complex operations are broken into atomic steps

### Requirement: ito-proposal includes TDD flow per task

The `ito-proposal` skill SHALL document TDD flow for each implementation task.

#### Scenario: TDD task structure

- **WHEN** `ito-proposal` creates an implementation task
- **THEN** the task follows TDD steps: write failing test → run test → implement → run test → commit

### Requirement: ito-proposal includes task structure best practices

The `ito-proposal` skill SHALL guide users on task structure.

#### Scenario: Task completeness

- **WHEN** `ito-proposal` creates tasks
- **THEN** each task specifies: exact file paths, what code to write, exact commands to run
- **AND** tasks are self-contained and unambiguous

### Requirement: ito-proposal includes plan header guidance

The `ito-proposal` skill SHALL guide users on documenting context in proposals.

#### Scenario: Proposal context

- **WHEN** `ito-proposal` creates a proposal
- **THEN** it documents: goal, architecture decisions, tech stack considerations

## REMOVED Requirements

### Requirement: writing-plans skill removed

The `writing-plans` skill SHALL be removed from the ito-skills collection.

#### Scenario: Skill no longer exists

- **WHEN** a user or skill references `writing-plans` or `ito-writing-plans`
- **THEN** the skill is not found
- **AND** users should use `ito-proposal` instead

## MODIFIED Requirements

### Requirement: subagent-driven-development references ito-proposal

The `subagent-driven-development` skill SHALL reference `ito-proposal` for task creation instead of `writing-plans`.

#### Scenario: Planning reference

- **WHEN** `subagent-driven-development` needs a plan created
- **THEN** it directs users to `ito-proposal`
