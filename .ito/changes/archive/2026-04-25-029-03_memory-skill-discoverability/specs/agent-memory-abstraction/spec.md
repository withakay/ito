<!-- ITO:START -->
## ADDED Requirements

### Requirement: Installed Ito memory skill

Ito SHALL install a shared `ito-memory` skill that explains how agents capture, search, and query project memory through the configured memory provider abstraction.

- **Requirement ID**: `agent-memory-abstraction:installed-ito-memory-skill`

#### Scenario: Skill is installed by Ito template distribution

- **WHEN** `ito init`, `ito init --upgrade`, or `ito update` installs shared Ito skills for a supported harness
- **THEN** the installed skill set includes `ito-memory`
- **AND** the skill is installed through the same shared skill distribution path as other `ito-*` skills

#### Scenario: Skill covers all memory operations

- **WHEN** an agent reads the `ito-memory` skill
- **THEN** the skill explains capture, search, and query workflows
- **AND** it directs the agent to use `ito agent instruction memory-capture`, `ito agent instruction memory-search`, and `ito agent instruction memory-query`
- **AND** it does not require a specific provider such as ByteRover

### Requirement: Memory instruction artifacts are discoverable in CLI help

The `ito agent instruction` help surface SHALL list the memory instruction artifacts and include examples for them so agents can discover the memory workflow without prior artifact-name knowledge.

- **Requirement ID**: `agent-memory-abstraction:memory-artifacts-in-help`

#### Scenario: Agent instruction help lists memory artifacts

- **WHEN** a user or agent runs `ito agent instruction --help`
- **THEN** the artifact list includes `memory-capture`, `memory-search`, and `memory-query`
- **AND** the descriptions distinguish capture, ranked search, and synthesized query behavior

#### Scenario: Agent instruction help includes memory examples

- **WHEN** a user or agent reads the `ito agent instruction --help` examples
- **THEN** the examples include at least one memory artifact invocation
- **AND** the examples show the required `--query` argument for search or query artifacts
<!-- ITO:END -->
