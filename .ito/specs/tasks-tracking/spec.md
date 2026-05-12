<!-- ITO:START -->
## ADDED Requirements

These requirements tighten enhanced-task field semantics so the `task_quality` rule (`cli-validate:task-quality-validation`) has unambiguous structured input. Field severities for the rule live in the cli-validate spec; this spec governs parsing.

### Requirement: Enhanced tasks expose quality-critical fields

Enhanced task blocks SHALL preserve the following metadata as structured fields on the parsed task: `Files`, `Dependencies`, `Action`, `Verify`, `Done When`, `Requirements`, `Status`, `Updated At`.

- **Requirement ID**: tasks-tracking:quality-critical-fields

#### Scenario: All fields are parsed when present

- **GIVEN** an enhanced task block contains lines `- **Files**:`, `- **Dependencies**:`, `- **Action**:`, `- **Verify**:`, `- **Done When**:`, `- **Requirements**:`, `- **Status**:`, and `- **Updated At**:`
- **WHEN** Ito parses the tracking file
- **THEN** the parsed task exposes each value as a separate structured field

#### Scenario: Missing optional fields produce no parser error

- **GIVEN** an enhanced task omits `Files`, `Dependencies`, `Action`, or `Updated At`
- **WHEN** Ito parses the tracking file
- **THEN** parsing succeeds and the missing fields are absent from the parsed task
- **AND** any severity decision is left to the `task_quality` rule

### Requirement: Vague verification denylist semantics

The vague-verification check SHALL use an exact, case-insensitive denylist evaluated after trimming whitespace.

- **Requirement ID**: tasks-tracking:concrete-verification

#### Scenario: Denylist match warns

- **GIVEN** a task has `Verify: Run Tests` (any case)
- **WHEN** the `task_quality` rule runs
- **THEN** validation emits a warning identifying the task and the denylist match

#### Scenario: Tool-led verify is accepted

- **GIVEN** a task has `Verify: make test`, `Verify: cargo test ...`, `Verify: ito validate ...`, or any value not in the denylist
- **WHEN** the `task_quality` rule runs
- **THEN** validation does not emit a vague-verification warning
<!-- ITO:END -->
