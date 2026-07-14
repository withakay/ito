<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Ito agents are classified by activation mode
Ito-managed generated agent templates SHALL declare whether each native agent is a direct entrypoint or a delegated role sub-agent. Agent activation metadata MUST remain independent of the canonical skill inventory, and no agent definition may be installed under a skill discovery directory.

#### Scenario: Harness supports native agent definitions
- **WHEN** a supported harness has a distinct native location or format for agents
- **THEN** Ito installs each retained agent only in that native agent surface
- **AND** the prompt and metadata preserve its direct or delegated activation mode

#### Scenario: Harness lacks an independent agent surface
- **WHEN** a harness can represent an agent role only by creating a discoverable skill
- **THEN** Ito does not synthesize the role as an additional managed skill
- **AND** retained lifecycle skills use instruction-backed or ordinary harness delegation instead

### Requirement: General and orchestrator agents are direct entrypoints
When a harness supports native agents, Ito SHALL keep `ito-general` and `ito-orchestrator` as native direct entrypoints. Neither agent SHALL create a skill directory or count toward the seven Ito-managed lifecycle skills.

#### Scenario: Native direct agents remain activatable
- **WHEN** Ito installs or updates agents for a harness with a native direct-agent surface
- **THEN** `ito-general` and `ito-orchestrator` remain directly activatable there
- **AND** their prompts describe their direct responsibilities

#### Scenario: Direct agents do not expand skills
- **WHEN** generated agent and skill surfaces are audited
- **THEN** direct agent definitions are reported separately
- **AND** the Ito-managed skill set remains exactly the canonical seven

### Requirement: Delegated role agents remain narrowly scoped
Ito MAY keep planner, researcher, worker, reviewer, and test-runner roles as narrowly scoped delegated agents only where the harness provides a native sub-agent surface. Such roles MUST report to the owning lifecycle workflow and MUST NOT be installed as discoverable skills.

#### Scenario: Native delegated role is retained
- **WHEN** implementation, research, review, planning, or test execution is delegated in a harness with native sub-agents
- **THEN** the narrow role may be installed in the native sub-agent location
- **AND** it reports to the retained lifecycle entrypoint that dispatched it

#### Scenario: Delegated roles are not skill fallbacks
- **WHEN** a harness lacks a native delegated-agent mechanism
- **THEN** Ito does not create planner, researcher, worker, reviewer, or test-runner skill directories
<!-- ITO:END -->
