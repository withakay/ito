# Cascading Config Specification

## Purpose

Define the `cascading-config` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Cascading project config sources

The system SHALL load project configuration by cascading multiple config files, merging them in precedence order.

Precedence order (lowest to highest):

1. `<repo-root>/ito.json`
1. `<repo-root>/.ito.json`
1. `<itoDir>/config.json`
1. If `PROJECT_DIR` is set: `$PROJECT_DIR/config.json`

#### Scenario: Later config overrides earlier

- **WHEN** a key is present in multiple config sources
- **THEN** the value from the highest-precedence source is used

### Requirement: Merge semantics

The system SHALL deep-merge JSON objects when combining sources.

#### Scenario: Objects merge recursively

- **WHEN** two sources contain objects at the same key
- **THEN** their child keys are merged recursively

#### Scenario: Arrays are replaced

- **WHEN** two sources contain arrays at the same key
- **THEN** the higher-precedence array replaces the lower-precedence array

### Requirement: Invalid JSON does not break commands

The system MUST treat invalid JSON in optional config sources as non-fatal.

#### Scenario: Invalid project config is ignored

- **WHEN** a config file exists but contains invalid JSON
- **THEN** the system ignores that file for config merging
- **AND** continues using other sources and defaults
