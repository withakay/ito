## ADDED Requirements

### Requirement: Architecture policy checks use ecosystem-native tooling

Repository architecture policy checks MUST prefer standard Rust ecosystem tooling over bespoke scripts when equivalent policy coverage is practical.
The pre-commit and CI workflows MUST run the same architecture policy commands.

#### Scenario: Dependency direction is enforced with standard tooling

- **WHEN** architecture policy checks run locally or in CI
- **THEN** dependency direction constraints MUST be verified with declarative/configured ecosystem tooling (for example `cargo-deny`)

#### Scenario: Feature decoupling is enforced with Cargo-native checks

- **WHEN** `ito-cli` architecture checks run
- **THEN** Cargo-native no-default-features checks MUST verify that `ito-cli` does not pull `ito-web` unintentionally

#### Scenario: Equivalent commands run in pre-commit and CI

- **WHEN** comparing local hook configuration and CI workflow steps
- **THEN** both environments MUST execute the same architecture policy checks (or a documented equivalent command set)
