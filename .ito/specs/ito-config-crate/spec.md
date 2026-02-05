# Ito Config Crate Specification

## Purpose

Define the `ito-config-crate` capability: configuration discovery, resolution, and merging.

## Requirements

### Requirement: ito-config crate exists as configuration layer

The `ito-config` crate SHALL exist and depend only on `ito-common` (no other `ito-*` dependencies). It SHALL provide configuration loading, resolution, and context management.

#### Scenario: Crate depends only on ito-common
- **WHEN** examining `ito-config/Cargo.toml`
- **THEN** the only `ito-*` dependency is `ito-common`

### Requirement: ItoContext struct

The crate SHALL provide a `ItoContext` struct that holds resolved configuration state including config directory, project root, ito path, and merged configuration values.

#### Scenario: Create context from project root
- **WHEN** calling `ItoContext::resolve(fs, project_root)`
- **THEN** returns context with resolved paths and merged configuration

#### Scenario: Context includes all resolved paths
- **WHEN** examining a resolved `ItoContext`
- **THEN** it contains `config_dir`, `project_root`, `ito_path`, and `config` fields

### Requirement: Cascading configuration loading

The crate SHALL load configuration from multiple sources (global, project, ito-dir) and merge them with appropriate precedence (ito-dir > project > global).

#### Scenario: Merge global and project config
- **WHEN** global config has `key=1` and project config has `key=2`
- **THEN** resolved config has `key=2` (project wins)

#### Scenario: Ito-dir config has highest precedence
- **WHEN** global has `key=1`, project has `key=2`, ito-dir has `key=3`
- **THEN** resolved config has `key=3` (ito-dir wins)

### Requirement: Ito directory resolution

The crate SHALL provide functions to resolve the ito directory name (`.ito` by default, configurable) and locate ito directories from a given path.

#### Scenario: Default ito directory name
- **WHEN** no configuration overrides the ito directory name
- **THEN** the ito directory name is `.ito`

#### Scenario: Find ito directory from nested path
- **WHEN** calling `find_ito_dir` from `/project/src/deep/nested`
- **THEN** returns `/project/.ito` if it exists

### Requirement: UI options resolution

The crate SHALL provide functions to resolve UI options (no_color, interactive mode) from environment and configuration.

#### Scenario: Respect NO_COLOR environment variable
- **WHEN** `NO_COLOR` environment variable is set
- **THEN** `UiOptions::no_color()` returns true

#### Scenario: Detect interactive mode
- **WHEN** stdout is a TTY
- **THEN** `UiOptions::interactive()` returns true
