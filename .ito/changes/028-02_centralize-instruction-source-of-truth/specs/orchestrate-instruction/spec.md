<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Orchestrate instruction artifact type

The system SHALL support `orchestrate` as a first-class artifact type for `ito agent instruction`, rendering a complete authoritative orchestrator instruction document from baked-in orchestration policy, the project's `orchestrate.md` user prompt, per-change metadata, detected run context, and the canonical agent surface taxonomy. Skills and orchestrator agent prompts SHALL treat this rendered instruction as the source of truth for orchestration behavior.

- **Requirement ID**: orchestrate-instruction:artifact-type

#### Scenario: Render orchestrate instruction document

- **WHEN** an agent invokes `ito agent instruction orchestrate`
- **THEN** the system renders `orchestrate.md.j2` injecting the project's `orchestrate.md` user prompt content, resolved change list with `depends_on` and `preferred_gates` per change, detected harness context, and available preset if configured
- **AND** the rendered document is printed to stdout

#### Scenario: Missing orchestrate.md triggers setup guidance

- **WHEN** an agent invokes `ito agent instruction orchestrate` and no `orchestrate.md` exists in the project's user-prompts directory
- **THEN** the system emits a setup guidance message directing the agent to load the `ito-orchestrate-setup` skill before proceeding
- **AND** exits with a non-zero status code

#### Scenario: Harness context injection

- **WHEN** `ito agent instruction orchestrate` is rendered in an OpenCode session
- **THEN** the rendered document includes the detected harness name and available agent role suggestions derived from the active preset

#### Scenario: Complete orchestrator policy is rendered

- **WHEN** `ito agent instruction orchestrate` renders successfully
- **THEN** the output includes canonical guidance for orchestration source-of-truth precedence, coordinator responsibilities, planner/researcher/worker/reviewer/test-runner roles, dependency planning, gate order, run state files, event logging, failure policy, remediation packets, and resume behavior
- **AND** skills and agent prompts can remain thin because the rendered instruction contains the canonical workflow detail

#### Scenario: Direct orchestrator and delegated roles are rendered

- **WHEN** `ito agent instruction orchestrate` renders successfully
- **THEN** the output identifies `ito-orchestrator` as the direct coordinator entrypoint
- **AND** identifies planner, researcher, worker, reviewer, and test-runner agents as delegated roles dispatched by the orchestrator
- **AND** does not describe `ito-orchestrator` or `ito-general` as ordinary delegated worker sub-agents

#### Scenario: Project guidance is additive only

- **WHEN** `.ito/user-prompts/orchestrate.md` contains project-specific MUST, PREFER, or note sections
- **THEN** the rendered orchestrate instruction includes that project guidance as additive local policy
- **AND** local project guidance does not replace baked-in source-of-truth sections unless an explicit supported override field is documented
<!-- ITO:END -->
