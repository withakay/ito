<!-- ITO:START -->
## ADDED Requirements

### Requirement: OpenCode Agent Assets Install To Plural Agents Directory

`ito update` SHALL install OpenCode agent markdown assets under `.opencode/agents/`, matching OpenCode's current plural directory name, and SHALL NOT install refreshed OpenCode agent assets under the obsolete `.opencode/agent/` directory.

#### Scenario: Update installs OpenCode agents to plural directory

- **GIVEN** a project uses OpenCode tool assets
- **WHEN** `ito update` refreshes OpenCode agent assets
- **THEN** Ito writes those agent markdown files under `.opencode/agents/`
- **AND** Ito does not write refreshed OpenCode agent markdown files under `.opencode/agent/`

#### Scenario: Repeated update remains stable

- **GIVEN** `ito update` has installed OpenCode agent assets under `.opencode/agents/`
- **WHEN** `ito update` is executed again
- **THEN** the OpenCode agent assets remain under `.opencode/agents/`
- **AND** the obsolete `.opencode/agent/` path is not recreated by Ito
<!-- ITO:END -->
