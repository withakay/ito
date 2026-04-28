<!-- ITO:START -->
## ADDED Requirements

### Requirement: Instruction artifacts are authoritative workflow sources

When Ito provides a baked-in `ito agent instruction <artifact>` workflow for a capability, the rendered instruction artifact SHALL be the authoritative source of truth for that workflow's agent-facing behavior.

- **Requirement ID**: instruction-source-of-truth:authoritative-artifacts

#### Scenario: Skill defers to matching instruction artifact

- **WHEN** an Ito skill starts a workflow that has a matching `ito agent instruction <artifact>` command
- **THEN** the skill directs the agent to render and follow that instruction artifact before executing workflow-specific steps
- **AND** the skill does not duplicate canonical workflow policy that belongs in the instruction template

#### Scenario: Agent defers to matching instruction artifact

- **WHEN** an installed Ito agent prompt coordinates or executes a workflow that has a matching `ito agent instruction <artifact>` command
- **THEN** the agent prompt directs the agent to render and follow that instruction artifact before relying on role-local guidance
- **AND** role-local guidance is limited to role boundaries, reporting format, and safety constraints not already covered by the instruction artifact

### Requirement: Skills and agents remain thin adapters

Ito-provided workflow skills and agent prompts SHALL prefer concise loader behavior over embedding detailed canonical instructions.

- **Requirement ID**: instruction-source-of-truth:thin-adapters

#### Scenario: Workflow detail moves from skill to instruction

- **WHEN** a workflow skill contains detailed policy that duplicates a baked-in instruction artifact's intended scope
- **THEN** that policy is migrated into the instruction template
- **AND** the skill retains only discovery, invocation, fallback, and handoff guidance

#### Scenario: No matching instruction artifact exists

- **WHEN** an Ito skill or agent covers a workflow without a matching instruction artifact
- **THEN** the skill or agent may contain workflow detail
- **AND** the workflow is a candidate for a future instruction artifact if the detail becomes reusable or cross-harness

### Requirement: Harness-installed templates preserve the source-of-truth boundary

Ito SHALL install harness command, skill, and agent templates that consistently point to the corresponding instruction artifact instead of becoming independent workflow definitions.

- **Requirement ID**: instruction-source-of-truth:harness-template-boundary

#### Scenario: Installed harness files use instruction invocation

- **WHEN** `ito init`, `ito init --upgrade`, or `ito update` installs Ito-managed skills, commands, or agents for a supported harness
- **THEN** files for workflows with baked-in instruction artifacts include the instruction invocation as their canonical first step
- **AND** they do not contain conflicting gate order, state model, remediation, or provider-operation policy
<!-- ITO:END -->
