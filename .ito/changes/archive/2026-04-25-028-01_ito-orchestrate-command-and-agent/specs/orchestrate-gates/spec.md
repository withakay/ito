<!-- ITO:START -->
## ADDED Requirements

### Requirement: Default gate pipeline

The system SHALL define a default gate pipeline executed in the following order for each change: `apply-complete`, `format`, `lint`, `tests`, `style`, `code-review`, `security-review`. Cheaper objective gates SHALL run before expensive reviewer passes.

- **Requirement ID**: orchestrate-gates:pipeline

#### Scenario: Gates execute in default order

- **WHEN** no gate overrides are present in `orchestrate.md` or `.ito.yaml`
- **THEN** the orchestrator executes gates in the order: `apply-complete → format → lint → tests → style → code-review → security-review`

#### Scenario: Gate is skipped when not applicable

- **WHEN** a preset or user prompt marks a gate as `skip`
- **THEN** the gate is recorded as skipped in the run state and the next gate proceeds immediately

### Requirement: Gate pass, fail, and skip semantics

Each gate SHALL have exactly one of three terminal outcomes: `pass` (worker reported success), `fail` (worker reported failure or timed out), `skip` (gate excluded by policy). A `fail` outcome SHALL halt further gate execution for that change and trigger remediation.

- **Requirement ID**: orchestrate-gates:semantics

#### Scenario: Passing gate allows downstream gate to proceed

- **WHEN** a gate reports `pass` for a change
- **THEN** the orchestrator dispatches the next gate in the pipeline for that change

#### Scenario: Failing gate halts the change pipeline

- **WHEN** a gate reports `fail` for a change
- **THEN** the orchestrator stops dispatching further gates for that change
- **AND** records the failure in `changes/<change-id>.json`
- **AND** initiates remediation unless `failure_policy` is `stop`

### Requirement: Remediation on gate failure

The system SHALL, on gate failure, construct a remediation packet containing the failed gate name, the gate's error output, and the change context, then dispatch it to a fresh apply worker. Only the failed gate and its downstream gates SHALL be rerun after remediation; previously passing gates SHALL NOT be rerun.

- **Requirement ID**: orchestrate-gates:remediation

#### Scenario: Remediation packet dispatched to fresh worker

- **WHEN** a gate fails for a change
- **THEN** the orchestrator creates a remediation packet and dispatches it to a new apply worker agent
- **AND** the original worker is not reused

#### Scenario: Only failed and downstream gates rerun after remediation

- **WHEN** the apply worker reports remediation complete
- **THEN** the orchestrator reruns only the failed gate and all gates that follow it in the pipeline
- **AND** gates that passed before the failure are not rerun
<!-- ITO:END -->
