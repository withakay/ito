<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ito agents are classified by activation mode

Ito-managed generated agent templates SHALL declare and preserve whether each agent is a direct entrypoint or a delegated role sub-agent.

- **Requirement ID**: agent-surface-taxonomy:activation-mode

#### Scenario: Direct entrypoint is installed for direct activation

- **WHEN** a supported harness distinguishes direct agents from delegated sub-agents
- **THEN** Ito installs direct entrypoint agents in the harness location or format used for direct user activation
- **AND** the prompt describes the agent as user-activatable rather than as a worker spawned by another agent

#### Scenario: Delegated role is installed as sub-agent

- **WHEN** a supported harness distinguishes direct agents from delegated sub-agents
- **THEN** Ito installs delegated role agents in the harness location or format used for sub-agents
- **AND** the prompt names the direct workflow or coordinator expected to dispatch that role

#### Scenario: Harness lacks direct versus delegated separation

- **WHEN** a supported harness does not provide separate direct-agent and sub-agent installation mechanisms
- **THEN** Ito preserves the activation-mode distinction in generated prompt text, metadata, or naming
- **AND** direct entrypoints remain discoverable as primary user-facing Ito agents

### Requirement: General and orchestrator agents are direct entrypoints

Ito SHALL treat `ito-general` and `ito-orchestrator` as direct entrypoint agents rather than delegated sub-agents.

- **Requirement ID**: agent-surface-taxonomy:direct-general-orchestrator

#### Scenario: General agent is directly activatable

- **WHEN** Ito installs or updates generated agents for a supported harness
- **THEN** `ito-general` is available as a direct user-activatable agent
- **AND** its prompt describes it as the balanced Ito development agent for direct use

#### Scenario: Orchestrator agent is directly activatable

- **WHEN** Ito installs or updates generated agents for a supported harness
- **THEN** `ito-orchestrator` is available as a direct user-activatable coordinator agent
- **AND** its prompt describes it as responsible for coordinating delegated planner, researcher, worker, reviewer, and test-runner roles

#### Scenario: Direct agents are not generated only as delegated roles

- **WHEN** generated harness assets are inspected after `ito init`, `ito init --upgrade`, or `ito update`
- **THEN** `ito-general` and `ito-orchestrator` do not appear solely in delegated sub-agent locations or metadata
- **AND** any delegated copies are either removed or clearly marked as compatibility shims with a migration path

### Requirement: Delegated role agents remain narrowly scoped

Ito SHALL keep planner, researcher, worker, reviewer, and test-runner style agents as delegated roles with narrow responsibilities.

- **Requirement ID**: agent-surface-taxonomy:delegated-role-agents

#### Scenario: Planner role is delegated

- **WHEN** the orchestrator needs a run plan or task decomposition
- **THEN** it may dispatch the delegated planner role
- **AND** the planner prompt does not present itself as the direct user entrypoint for Ito work

#### Scenario: Worker and reviewer roles are delegated

- **WHEN** implementation or review work is needed inside an orchestrated run
- **THEN** worker and reviewer prompts are available as delegated role sub-agents
- **AND** they report results back to the orchestrator rather than independently owning the user-facing workflow

#### Scenario: Test runner role is delegated

- **WHEN** verification commands need to be run with curated output
- **THEN** the test-runner prompt is available as a delegated role sub-agent
- **AND** it reports pass/fail evidence to the direct entrypoint or orchestrator that requested it

### Requirement: Orchestration and multi-agent surfaces are consolidated

Ito SHALL reduce overlapping orchestration and multi-agent generated surfaces into a cohesive set with one authoritative instruction-backed workflow.

- **Requirement ID**: agent-surface-taxonomy:orchestration-consolidation

#### Scenario: Overlapping orchestration skills are consolidated

- **WHEN** Ito-managed skills or prompts duplicate `ito agent instruction orchestrate` policy
- **THEN** canonical policy is moved into the orchestrate instruction artifact
- **AND** remaining skills or prompts are either thin adapters, role-specific prompts, project-guidance surfaces, or removed Ito-managed assets

#### Scenario: Obsolete generated orchestration assets are cleaned up

- **WHEN** an Ito update removes or replaces an obsolete Ito-managed orchestration or multi-agent asset
- **THEN** the installer removes the obsolete generated asset when safe
- **AND** the replacement surface is documented in generated guidance or release notes

#### Scenario: Non-overlapping specialized workflows are retained deliberately

- **WHEN** an orchestration-adjacent skill such as test delegation remains separate
- **THEN** its trigger, responsibility, and relationship to `ito agent instruction orchestrate` are documented
- **AND** it does not duplicate canonical orchestration gate order, run-state, or remediation policy
<!-- ITO:END -->
