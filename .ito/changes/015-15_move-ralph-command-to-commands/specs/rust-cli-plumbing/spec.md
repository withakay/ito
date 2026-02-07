## ADDED Requirements

### Requirement: CLI command handlers live under commands

`ito-cli` command handlers SHALL live under `ito-rs/crates/ito-cli/src/commands/`.

The `ito-cli/src/app/` directory SHALL be reserved for shared application glue and helpers that are not a single command implementation.

#### Scenario: Ralph command handler is in commands

- **WHEN** inspecting the Rust source tree
- **THEN** `ito-rs/crates/ito-cli/src/commands/ralph.rs` exists
- **AND** `ito-rs/crates/ito-cli/src/app/ralph.rs` does not exist
