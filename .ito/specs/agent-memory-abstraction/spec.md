<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Installed Ito memory skill

Ito SHALL install a shared `ito-memory` skill that explains how agents capture, search, and query project memory through the configured memory provider abstraction. The skill SHALL be a thin entrypoint that routes agents to `ito agent instruction memory-capture`, `ito agent instruction memory-search`, and `ito agent instruction memory-query` as the authoritative memory workflow instructions.

- **Requirement ID**: agent-memory-abstraction:installed-ito-memory-skill

#### Scenario: Skill is installed by Ito template distribution

- **WHEN** `ito init`, `ito init --upgrade`, or `ito update` installs shared Ito skills for a supported harness
- **THEN** the installed skill set includes `ito-memory`
- **AND** the skill is installed through the same shared skill distribution path as other `ito-*` skills

#### Scenario: Skill covers all memory operations

- **WHEN** an agent reads the `ito-memory` skill
- **THEN** the skill explains capture, search, and query workflows
- **AND** it directs the agent to use `ito agent instruction memory-capture`, `ito agent instruction memory-search`, and `ito agent instruction memory-query`
- **AND** it does not require a specific provider such as ByteRover

#### Scenario: Memory operation detail comes from instruction artifacts

- **WHEN** an agent needs to capture, search, or query memory through Ito
- **THEN** the `ito-memory` skill directs the agent to render the corresponding memory instruction artifact
- **AND** provider routing, required flags, output expectations, and fallback guidance are sourced from the rendered instruction rather than duplicated in the skill
<!-- ITO:END -->
