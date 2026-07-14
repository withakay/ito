<!-- ITO:START -->
## ADDED Requirements

### Requirement: Run state directory layout

The system SHALL maintain all run state under `.ito/.state/orchestrate/runs/<run-id>/` using a fixed file layout: `run.json` for run metadata, `plan.json` for the resolved execution plan, `events.jsonl` as an append-only event log, and `changes/<change-id>.json` for per-change gate results.

- **Requirement ID**: orchestrate-run-state:layout

#### Scenario: State directory is created on first run

- **WHEN** the orchestrator begins a new run
- **THEN** the system creates `.ito/.state/orchestrate/runs/<run-id>/` and initialises `run.json` and `plan.json` before dispatching any worker
- **AND** the run directory is NOT committed to version control

#### Scenario: Per-change gate result file is written after each gate

- **WHEN** a gate completes for a given change (pass, fail, or skip)
- **THEN** the system writes or updates `changes/<change-id>.json` with the gate name, status, timestamp, and any error payload

### Requirement: Append-only event log

The system SHALL append a structured JSON event record to `events.jsonl` for every significant lifecycle transition: run start, run complete, gate start, gate pass, gate fail, gate skip, worker dispatch, worker complete, remediation dispatch.

- **Requirement ID**: orchestrate-run-state:event-log

#### Scenario: Event log is never truncated during a run

- **WHEN** the orchestrator appends an event to `events.jsonl`
- **THEN** all previous events remain intact
- **AND** the new event is a valid JSON object on its own line

#### Scenario: Interrupted run can be inspected via event log

- **WHEN** an orchestrator run is interrupted mid-execution
- **THEN** the `events.jsonl` file contains all events up to the point of interruption
- **AND** an agent resuming the run can reconstruct the last known state from `events.jsonl` and the per-change gate files

### Requirement: Run resumability

The system SHALL support resuming an interrupted run by reading the existing state directory, skipping gates already recorded as passed, and continuing from the last incomplete gate.

- **Requirement ID**: orchestrate-run-state:resumability

#### Scenario: Resume after interruption

- **WHEN** the orchestrator is invoked with `--resume <run-id>`
- **THEN** it reads the existing `plan.json` and per-change gate files
- **AND** only reruns gates that are not recorded as passed
<!-- ITO:END -->
