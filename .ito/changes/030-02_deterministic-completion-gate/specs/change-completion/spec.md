## ADDED Requirements

### Requirement: Deterministic Completion Verdict

Ito SHALL provide a command that evaluates whether a change is complete using deterministic evidence.

#### Scenario: Complete change verdict

- **WHEN** all required artifacts exist, all tasks are complete or shelved with reasons, required validation passes, and no completion blockers exist
- **THEN** `ito change verify-complete <change> --json` SHALL return a `complete` verdict.

#### Scenario: Checkbox-only false positive

- **WHEN** task checkboxes are complete but required implementation evidence or validation is missing
- **THEN** the verifier SHALL return an `incomplete` verdict
- **AND** the response SHALL identify the missing evidence.

### Requirement: Machine-Readable Blocking Reasons

The verifier SHALL emit structured blocking reasons suitable for agents and automation.

#### Scenario: Blocking reasons include evidence

- **WHEN** the verifier detects an incomplete change
- **THEN** the JSON response SHALL include reason codes, human-readable messages, and evidence paths or commands where available.

### Requirement: Ralph Completion Integration

Ralph SHALL use the deterministic verifier before accepting completion promises.

#### Scenario: Completion promise rejected

- **WHEN** an agent emits a completion promise for a change that fails verification
- **THEN** Ralph SHALL reject the promise and continue or fail according to loop configuration
- **AND** Ralph SHALL include verifier reasons in the next prompt or final report.

### Requirement: Archive Completion Integration

Archive flows SHALL use the verifier before moving a change to the archive.

#### Scenario: Archive refuses incomplete change

- **WHEN** a user runs `ito archive <change>` for a change that fails completion verification
- **THEN** Ito SHALL refuse to archive the change by default
- **AND** Ito SHALL print or emit the verifier's blocking reasons.
