## MODIFIED Requirements

### Requirement: ito-logging is a leaf crate

The `ito-logging` crate SHALL have no dependencies on other `ito-*` crates. It SHALL accept configuration values (like `config_dir`) as explicit parameters rather than importing configuration types.

#### Scenario: Crate has no ito dependencies
- **WHEN** examining `ito-logging/Cargo.toml`
- **THEN** there are no dependencies on other `ito-*` crates

### Requirement: Logger accepts explicit paths

The `Logger::new()` constructor SHALL accept `config_dir: Option<PathBuf>` as an explicit parameter instead of a `ConfigContext` reference.

#### Scenario: Create logger with explicit config dir
- **WHEN** calling `Logger::new(Some(PathBuf::from("/home/user/.config/ito")), command, subcommand)`
- **THEN** logger writes telemetry to that directory

#### Scenario: Create logger without config dir
- **WHEN** calling `Logger::new(None, command, subcommand)`
- **THEN** logger operates without writing telemetry to disk

#### Scenario: CLI provides config dir to logger
- **WHEN** CLI initializes logging
- **THEN** CLI resolves config dir via ito-config and passes it to Logger::new()
