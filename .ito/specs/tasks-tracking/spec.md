# Tasks Tracking Specification

## Purpose

Define the v1 tasks tracking markdown format used by `tasks.md` files under `.ito/changes/<change-id>/tasks.md`.

This specification exists so validators and schema validation can reference the format via a stable validator id.

## Requirements

### Requirement: Tasks tracking has a stable validator id

The v1 validator id for the tasks tracking format SHALL be `ito.tasks-tracking.v1`.

#### Scenario: Author can locate the normative spec

- **GIVEN** a validation issue cites `ito.tasks-tracking.v1`
- **WHEN** an author searches this repository
- **THEN** they find this normative spec at `.ito/specs/tasks-tracking/spec.md`

### Requirement: Tasks tracking supports checkbox encoding

The tasks tracking format MUST support a checkbox-list encoding.

In checkbox encoding, a task SHALL be a markdown list item starting with one of:

- `- [ ]` (pending)
- `- [x]` (complete)
- `- [~]` (in-progress)
- `- [>]` (in-progress alias)

#### Scenario: Checkbox tasks are recognized

- **WHEN** a `tasks.md` contains checkbox tasks with supported markers
- **THEN** those tasks are recognized and assigned the corresponding status

### Requirement: Tasks tracking supports enhanced wave-based encoding

The tasks tracking format MUST support an enhanced wave-based encoding suitable for `ito tasks`.

In enhanced encoding:

- Waves SHOULD be declared as headings of the form `## Wave <N>`.
- Each wave section MUST include a dependency line of the form `- **Depends On**: ...`.
- Tasks SHOULD be declared as headings of the form `### Task <id>: <name>`.
- Each enhanced task block MUST include `- **Status**: ...` and `- **Updated At**: YYYY-MM-DD`.
- Task dependencies declared via `- **Dependencies**: ...` MUST reference tasks within the same wave.

#### Scenario: Cross-wave dependency fails validation

- **GIVEN** an enhanced tasks file with Wave 2 depending on Wave 1
- **WHEN** a Wave 2 task declares a dependency on a Wave 1 task id
- **THEN** validation fails with an actionable error

#### Scenario: Missing updated-at fails validation

- **GIVEN** an enhanced task block without an `- **Updated At**:` line
- **WHEN** the tasks file is validated
- **THEN** validation fails

### Requirement: Tracking files contain at least one recognizable task

If a file is used as a tasks tracking file, it MUST contain at least one recognizable task (checkbox task or enhanced task block).

#### Scenario: Empty tasks file fails validation

- **GIVEN** a `tasks.md` file with no checkbox tasks and no enhanced task blocks
- **WHEN** it is validated as a tracking file
- **THEN** validation fails

### Requirement: Validation issues cite the tasks tracking validator id

Validation issues attributable to tasks tracking markdown SHALL cite `ito.tasks-tracking.v1`.

#### Scenario: Error message includes validator id

- **GIVEN** a tasks tracking file fails validation
- **WHEN** a validation issue is reported
- **THEN** the issue message includes `ito.tasks-tracking.v1`
