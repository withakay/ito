# Spec: cli-plan (MODIFIED)

> Modifications to emit audit events on planning state mutations.

## MODIFIED

### Requirement: Planning State Management

The `ito plan` command SHALL provide subcommands for managing planning state, emitting audit events for state transitions.

#### Scenario: Decision emits audit event

WHEN `ito plan decision` records a planning decision
THEN an audit event SHALL be emitted with `entity: "planning"`, `entity_id: <decision_id>`, `op: "decision"`, `actor: "cli"`
AND the `meta` field SHALL contain the decision content

#### Scenario: Blocker emits audit event

WHEN `ito plan blocker` records a blocker
THEN an audit event SHALL be emitted with `entity: "planning"`, `entity_id: <blocker_id>`, `op: "blocker"`, `actor: "cli"`

#### Scenario: Question emits audit event

WHEN `ito plan question` records a question
THEN an audit event SHALL be emitted with `entity: "planning"`, `entity_id: <question_id>`, `op: "question"`, `actor: "cli"`

#### Scenario: Note emits audit event

WHEN `ito plan note` records a note
THEN an audit event SHALL be emitted with `entity: "planning"`, `entity_id: <note_id>`, `op: "note"`, `actor: "cli"`

#### Scenario: Focus change emits audit event

WHEN `ito plan focus` changes the current focus
THEN an audit event SHALL be emitted with `entity: "planning"`, `op: "focus_change"`, `from: <old_focus>`, `to: <new_focus>`, `actor: "cli"`

#### Scenario: Audit event failure does not block planning operation

WHEN the planning operation succeeds but the audit event write fails
THEN the planning state change SHALL be preserved
AND the audit failure SHALL be logged at `warn` level
AND the command SHALL exit successfully
