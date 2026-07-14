<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Orchestrate instruction artifact type
The system SHALL support `orchestrate` as an instruction artifact that renders authoritative orchestration policy from baked-in templates, optional project guidance, change metadata, detected run context, and native agent capabilities. Retained `ito-loop` and `ito-apply` guidance MAY reference this artifact; no standalone orchestration skill is required.

#### Scenario: Render orchestrate instruction document
- **WHEN** an agent invokes `ito agent instruction orchestrate`
- **THEN** the system renders project guidance, dependency metadata, gates, run context, and available native roles
- **AND** prints the document to stdout

#### Scenario: Missing project guidance uses inline setup
- **WHEN** no project `orchestrate.md` exists
- **THEN** the system renders self-contained setup/remediation guidance
- **AND** does not name `ito-orchestrate-setup` or `ito-orchestrator-workflow`

#### Scenario: Harness context injection
- **WHEN** the instruction is rendered in a supported harness
- **THEN** it includes the harness name and native role suggestions that actually exist
- **AND** does not synthesize role skills as a fallback

#### Scenario: Complete policy is rendered
- **WHEN** orchestration instructions render successfully
- **THEN** the output includes source precedence, coordination responsibilities, dependency planning, gate order, run state, event logging, failure policy, remediation, and resume behavior

#### Scenario: Lifecycle entrypoint remains clear
- **WHEN** an agent uses orchestration for implementation work
- **THEN** `ito-loop` or `ito-apply` remains the installed lifecycle entrypoint
- **AND** native delegated roles report to that workflow rather than appearing as additional user-facing skills

#### Scenario: Project guidance is additive
- **WHEN** project-specific MUST, PREFER, or note sections exist
- **THEN** the rendered instruction includes them as additive local policy
- **AND** they do not silently replace baked-in safety gates
<!-- ITO:END -->
