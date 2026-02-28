## ADDED Requirements

### Requirement: Interactive init wizard pre-fills defaults from existing config

When `ito init` runs in interactive mode and a project already has relevant configuration values, the init wizard SHALL use those values as defaults so users do not have to re-select settings.

#### Scenario: Worktree enablement prompt defaults from config

- **GIVEN** `worktrees.enabled` is already set in resolved config
- **WHEN** the user runs `ito init` interactively
- **THEN** the worktree enablement question defaults to the configured value
- **AND** accepting the default keeps the configured value

#### Scenario: Worktree strategy prompt defaults from config

- **GIVEN** `worktrees.enabled=true` in resolved config
- **AND** `worktrees.strategy` is already set in resolved config
- **WHEN** the user runs `ito init` interactively
- **THEN** the worktree strategy selection defaults to the configured strategy
- **AND** accepting the default keeps the configured strategy

#### Scenario: Integration mode prompt defaults from config

- **GIVEN** `worktrees.enabled=true` in resolved config
- **AND** `worktrees.apply.integration_mode` is already set in resolved config
- **WHEN** the user runs `ito init` interactively
- **THEN** the integration mode selection defaults to the configured value
- **AND** accepting the default keeps the configured value

#### Scenario: Config is only written when a value changes

- **GIVEN** the wizard defaults are accepted without changes
- **WHEN** `ito init` completes
- **THEN** Ito does not persist duplicate config writes for unchanged keys
