<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Skills are managed via init/update (not CLI)

The system SHALL NOT expose skill inventory management as part of the supported CLI UX. `ito init` and `ito update` SHALL install the canonical seven-skill lifecycle inventory for each configured harness, preserving any bundled resource subdirectories belonging to those retained skills.

#### Scenario: Skills are refreshed by init/update
- **WHEN** a user runs `ito init` or `ito update`
- **THEN** the system installs or refreshes exactly `ito`, `ito-proposal`, `ito-research`, `ito-apply`, `ito-review`, `ito-archive`, and `ito-loop` as Ito-managed skills
- **AND** retained skill resources are written alongside `SKILL.md` with their directory structure preserved

#### Scenario: Skills commands remain callable but hidden
- **WHEN** a user executes a legacy `ito skills <subcommand>` compatibility path
- **THEN** the command executes according to its compatibility contract
- **AND** prints a deprecation warning pointing to `ito init` and `ito update`
- **AND** remains hidden from supported help and shell completions

#### Scenario: Harness adapters cannot add helper skills
- **WHEN** a harness manifest adapts the canonical skill inventory to harness-specific paths
- **THEN** it installs all seven retained skills
- **AND** it MUST NOT add delegated roles, command helpers, or harness-only Ito skills to the installed skill set
<!-- ITO:END -->
