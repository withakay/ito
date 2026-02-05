# Completed Status Display Specification

## Purpose

Define the `completed-status-display` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Changes with all tasks completed show "completed" status

The system SHALL display a "completed" status for changes where tasks.md exists and all tasks (checkbox items) are marked as done. This status SHALL be visually distinct from "no-tasks" and "in-progress" states.

#### Scenario: All tasks completed shows completed status

- **WHEN** a change has a tasks.md file with all checkbox items marked `[x]`
- **THEN** `ito list` SHALL display status as "completed" (not just task count like "5/5")

#### Scenario: Partial completion shows in-progress

- **WHEN** a change has a tasks.md file with some incomplete checkbox items
- **THEN** `ito list` SHALL display status as the task count ratio (e.g., "3/5")

#### Scenario: No tasks file shows no-tasks status

- **WHEN** a change has no tasks.md file or an empty tasks.md
- **THEN** `ito list` SHALL display status as "no-tasks"

### Requirement: JSON output includes completed boolean flag

The system SHALL include an explicit `completed` boolean field in `ito list --json` output for programmatic consumption.

#### Scenario: JSON output with completed change

- **WHEN** user runs `ito list --json` with a completed change
- **THEN** the output SHALL include `"completed": true` for that change alongside existing fields

#### Scenario: JSON output with incomplete change

- **WHEN** user runs `ito list --json` with an incomplete or no-tasks change
- **THEN** the output SHALL include `"completed": false` for that change

### Requirement: List command supports completed filter

The system SHALL support a `--completed` flag on `ito list` to show only changes that are completed.

#### Scenario: Filter to completed changes only

- **WHEN** user runs `ito list --completed`
- **THEN** only changes with "completed" status SHALL be displayed

#### Scenario: Filter returns empty when no completed changes

- **WHEN** user runs `ito list --completed` and no changes are completed
- **THEN** an empty list SHALL be displayed with an informational message
