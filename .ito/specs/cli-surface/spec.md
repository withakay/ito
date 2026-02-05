# Cli Surface Specification

## Purpose

Define the `cli-surface` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Preferred help surface is small and stable

The CLI SHALL expose a small stable top-level command surface that is the only supported UX shown in `ito --help`.

#### Scenario: Top-level help shows only stable commands and visible experimentals

- **WHEN** users execute `ito --help`
- **THEN** it lists the stable commands:
  - `init`, `update`
  - `dashboard`
  - `status`, `ralph`
  - `create`, `list`, `show`, `validate`, `archive`, `split`
  - `config`, `completions`
- **AND** it lists only the visible experimental commands:
  - `x-templates`, `x-schemas`

#### Scenario: Top-level help hides deprecated and internal commands

- **WHEN** users execute `ito --help`
- **THEN** it does not list deprecated shims or internal commands, including:
  - legacy noun-group shims: `change`, `spec`, `module`, `completion`, `skills`, `view`
  - legacy verb shims: `get`, `set`, `unset`, `reset`, `edit`, `path`, `generate`, `install`, `uninstall`
  - hidden experimental commands: all `x-*` except `x-templates` and `x-schemas`

### Requirement: Skills are not a user-facing CLI surface

The CLI SHALL NOT expose skills management as part of the supported CLI UX.

#### Scenario: Skills are not visible in help or completion

- **WHEN** users execute `ito --help` or use shell completion
- **THEN** skills operations are not suggested or documented
- **AND** users are guided to `ito init` and `ito update` for installing/updating the project instruction set

### Requirement: Deprecated noun-group shims remain callable but hidden

The CLI SHALL keep legacy noun-group entrypoints as deprecated compatibility shims.

#### Scenario: Deprecated shims remain callable

- **WHEN** users execute any deprecated shim:
  - `ito change <subcommand>`
  - `ito spec <subcommand>`
  - `ito module <subcommand>`
  - `ito completion <subcommand>`
  - `ito skills <subcommand>`
  - `ito config <subcommand>`
- **THEN** the command executes successfully with existing behavior
- **AND** prints a deprecation warning pointing to the equivalent verb-first command(s)

#### Scenario: Deprecated shims are omitted from completion

- **WHEN** users use shell completion
- **THEN** deprecated shims are not suggested as top-level commands

### Requirement: Deprecated verb shims remain callable but hidden

The CLI SHALL keep legacy verb entrypoints as deprecated compatibility shims.

#### Scenario: Deprecated verbs remain callable

- **WHEN** users execute any deprecated verb shim:
  - `ito get|set|unset|reset|edit|path ...`
  - `ito generate|install|uninstall ...`
- **THEN** the command executes successfully with existing behavior
- **AND** prints a deprecation warning pointing to the equivalent stable command group:
  - `ito config ...` for configuration operations
  - `ito completions ...` for completion operations

### Requirement: Deprecated dashboard alias remains callable but hidden

The CLI SHALL keep the legacy `view` entrypoint as a deprecated alias for `dashboard`.

#### Scenario: View alias delegates to dashboard

- **WHEN** users execute `ito view`
- **THEN** it behaves like `ito dashboard`
- **AND** prints a deprecation warning pointing to `ito dashboard`
