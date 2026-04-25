<!-- ITO:START -->
## ADDED Requirements

### Requirement: Init wizard defaults come from existing config

Interactive `ito init` SHALL load any existing project config before prompting and SHALL use explicit configured values as the default selected choices for matching wizard questions.

- **Requirement ID**: cli-init:existing-config-wizard-defaults

#### Scenario: Existing tmux preference is selected

- **GIVEN** a project config contains `tools.tmux.enabled = true`
- **WHEN** the user runs interactive `ito init`
- **THEN** the tmux prompt defaults to `Yes`

#### Scenario: Existing worktree strategy is selected

- **GIVEN** a project config enables worktrees and sets the bare sibling strategy
- **WHEN** the user reaches the worktree section of interactive `ito init`
- **THEN** worktrees are selected as enabled
- **AND** the bare sibling strategy is selected by default

#### Scenario: Existing config values are preserved when accepted

- **GIVEN** a project config contains explicit setup values
- **WHEN** the user runs interactive `ito init` and accepts the defaults
- **THEN** Ito preserves those explicit values in the resulting config

### Requirement: Init setup coverage tracks current config settings

`ito init` SHALL support every project-setup-relevant config setting through either an interactive wizard prompt, a non-interactive flag, or a documented reason that the setting is intentionally not part of setup.

- **Requirement ID**: cli-init:setup-config-coverage

#### Scenario: Missing setup setting is detected by tests

- **WHEN** a new project-setup-relevant config field is added to the config model
- **AND** it is not covered by an init prompt, init flag, or documented exclusion
- **THEN** the init/config coverage test fails

#### Scenario: Non-setup config is explicitly excluded

- **WHEN** a config setting is runtime-only or otherwise not appropriate for `ito init`
- **THEN** the setting is listed as intentionally excluded from init setup coverage
<!-- ITO:END -->
