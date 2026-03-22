## MODIFIED Requirements

### Requirement: Agent instructions document multi-tenant backend usage

Ito SHALL provide an agent instruction artifact that documents how to use the multi-tenant backend.

The instruction MUST cover, at minimum:

- the required `{org}/{repo}`-scoped route prefix (`/api/v1/projects/{org}/{repo}`)
- how to configure backend client `{org}/{repo}` (`backend.project.org` / `backend.project.repo`)
- how to start and configure the backend server (`backendServer.*` and `ito backend serve`)
- how authentication works (admin tokens and derived per-project tokens)

#### Scenario: Agent can retrieve backend instructions

- **WHEN** an agent runs `ito agent instruction backend`
- **THEN** the CLI prints backend usage instructions
- **AND** the startup examples reference `ito backend serve`

### Requirement: Bootstrap instructions link to backend instruction artifact

Bootstrapped agent instructions SHALL reference the backend instruction artifact so agents can discover backend-specific guidance.

#### Scenario: Bootstrap output mentions backend artifact

- **WHEN** an agent runs `ito agent instruction bootstrap --tool <tool>`
- **THEN** the output includes a reference to `ito agent instruction backend`

### Requirement: Skills and commands reference backend instruction artifact when backend features are involved

When skills/commands/prompts describe backend-related workflows, they SHALL direct agents to use `ito agent instruction backend` as the source of truth.

#### Scenario: Workflow skill references backend instruction

- **GIVEN** a skill describes the Ito workflow
- **WHEN** backend mode is involved
- **THEN** the skill instructs the agent to consult `ito agent instruction backend`
- **AND** any backend startup examples reference `ito backend serve`
