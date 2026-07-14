## MODIFIED Requirements

### Requirement: Cascading project config sources

The system SHALL load project configuration by cascading multiple config files, merging them in precedence order.

Precedence order (lowest to highest):

1. `<repo-root>/ito.json`
1. `<repo-root>/.ito.json`
1. `<itoDir>/config.json`
1. `<itoDir>/config.local.json`
1. `<repo-root>/.local/ito/config.json`
1. If `PROJECT_DIR` is set: `$PROJECT_DIR/config.json`

#### Scenario: Later config overrides earlier

- **WHEN** a key is present in multiple config sources
- **THEN** the value from the highest-precedence source is used

#### Scenario: Per-developer overlay overrides committed project config

- **GIVEN** a key is present in `<itoDir>/config.json`
- **AND** the same key is present in `<itoDir>/config.local.json`
- **WHEN** configuration is resolved
- **THEN** the value from `<itoDir>/config.local.json` is used
