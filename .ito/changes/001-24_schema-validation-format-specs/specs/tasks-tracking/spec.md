## ADDED Requirements

### Requirement: Tasks tracking format has a stable validator id and normative spec

The tasks tracking markdown format for `tasks.md` SHALL be documented as a first-class, versioned specification.

The v1 validator id for this format SHALL be `ito.tasks-tracking.v1`.

#### Scenario: Author discovers the correct spec from a validator id

- **GIVEN** a validation issue references `ito.tasks-tracking.v1`
- **WHEN** an author searches the repository for the spec
- **THEN** the normative spec document for v1 is discoverable at `.ito/specs/tasks-tracking/spec.md`

### Requirement: Tasks tracking supports checkbox encoding

The tasks tracking format MUST support a checkbox-list encoding.

In checkbox encoding, a task SHALL be represented by a markdown list item beginning with one of:

- `- [ ]` (pending)
- `- [x]` (complete)
- `- [~]` (in-progress)
- `- [>]` (in-progress alias)

#### Scenario: Checkbox tasks are recognized

- **WHEN** a `tasks.md` contains checkbox-list items using the supported markers
- **THEN** the system recognizes those items as tasks
- **AND** it assigns each one a status consistent with the marker

### Requirement: Tasks tracking supports enhanced wave-based encoding

The tasks tracking format MUST support an enhanced wave-based encoding suitable for the `ito tasks` CLI.

In enhanced encoding:

- Waves SHOULD be declared using headings of the form `## Wave <N>`.
- Tasks SHOULD be declared using headings of the form `### Task <id>: <name>`.
- Tasks MAY declare dependencies and status using bold-key metadata lines (e.g., `- **Dependencies**: ...`, `- **Status**: ...`).

#### Scenario: Enhanced tasks file is considered tracking

- **GIVEN** a `tasks.md` file authored in enhanced format
- **WHEN** it contains at least one recognizable task block
- **THEN** the file is considered a valid tasks tracking file

### Requirement: Enhanced wave-based encoding defines wave and dependency semantics

In enhanced wave-based encoding:

- A wave heading of the form `## Wave <N>` defines a wave number `<N>`.
- Each wave section MUST include a wave dependency line of the form `- **Depends On**: ...`.
- Wave `<N>` MUST be treated as dependent on completion of all prior waves unless explicitly documented otherwise.
- Task dependencies declared via `- **Dependencies**: ...` MUST reference tasks within the same wave.

#### Scenario: Cross-wave task dependency is rejected

- **GIVEN** an enhanced tasks file declares Wave 2 depends on Wave 1
- **WHEN** a Wave 2 task declares `- **Dependencies**: 1.1`
- **THEN** validation fails with an actionable message

### Requirement: Enhanced task blocks include status and updated-at metadata

Enhanced task blocks MUST include `- **Status**: ...` and `- **Updated At**: YYYY-MM-DD` lines.

#### Scenario: Missing updated-at metadata is rejected

- **GIVEN** an enhanced tasks file contains a task block without an `- **Updated At**:` line
- **WHEN** the file is validated
- **THEN** validation fails with an actionable message

### Requirement: Declared tracking files contain at least one task

If a file is used as a tasks tracking file, it MUST contain at least one recognizable task.

#### Scenario: Empty tracking file is invalid

- **GIVEN** a `tasks.md` file with no checkbox tasks and no enhanced task blocks
- **WHEN** the file is validated as a tasks tracking file
- **THEN** validation fails with an actionable message

### Requirement: Tasks tracking validation issues cite the validator id

Validation issues for tasks tracking markdown SHALL cite the format validator id.

#### Scenario: Validation issue cites validator id

- **GIVEN** a tasks tracking file fails validation
- **WHEN** a validation issue is produced
- **THEN** the issue text (or structured metadata) includes `ito.tasks-tracking.v1`
