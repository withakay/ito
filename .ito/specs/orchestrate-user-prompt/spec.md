<!-- ITO:START -->
## ADDED Requirements

### Requirement: Orchestrate user prompt schema

The system SHALL define a structured user prompt format for `orchestrate.md` consisting of a YAML front matter block followed by markdown sections (`## MUST`, `## PREFER`, `## Notes`) that the orchestrator reads to configure parallelism, gate order, failure policy, and gate overrides.

- **Requirement ID**: orchestrate-user-prompt:schema

#### Scenario: Valid front matter is parsed

- **WHEN** the orchestrator reads an `orchestrate.md` file with valid YAML front matter
- **THEN** the system extracts `max_parallel`, `failure_policy`, `preset`, and `gate_overrides` fields
- **AND** applies them to the run plan before execution begins

#### Scenario: Unknown front matter fields are ignored

- **WHEN** the `orchestrate.md` front matter contains unrecognised keys
- **THEN** the system ignores them and continues without error

#### Scenario: MUST section is treated as hard policy

- **WHEN** the orchestrator loads an `orchestrate.md` with a `## MUST` section
- **THEN** every instruction in that section is injected verbatim into the rendered orchestrator instruction document as non-negotiable policy
- **AND** the orchestrator agent MUST NOT deviate from those instructions

#### Scenario: PREFER section is treated as soft guidance

- **WHEN** the orchestrator loads an `orchestrate.md` with a `## PREFER` section
- **THEN** every instruction in that section is injected as advisory guidance
- **AND** the orchestrator MAY deviate when there is a well-reasoned conflict

### Requirement: Gate override via user prompt

The system SHALL allow individual gates to be disabled, added, or reordered through the `gate_overrides` key in `orchestrate.md` front matter without requiring changes to preset files or CLI flags.

- **Requirement ID**: orchestrate-user-prompt:gate-overrides

#### Scenario: Disable a gate

- **WHEN** `gate_overrides` specifies `security-review: skip`
- **THEN** the `security-review` gate is excluded from the run plan for all changes in that run

#### Scenario: Override gate order

- **WHEN** `gate_overrides` specifies a custom gate sequence
- **THEN** the resolved run plan uses that sequence, overriding the default gate order
<!-- ITO:END -->
