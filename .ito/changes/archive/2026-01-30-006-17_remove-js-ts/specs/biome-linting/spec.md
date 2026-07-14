## REMOVED Requirements

### Requirement: Linting uses Biome

This requirement is removed; linting SHALL be performed by Rust tooling.
**Reason**: Biome tooling is removed.
**Migration**: Use `cargo clippy` via `make lint`.

#### Scenario: Rust lint replaces Biome

- **WHEN** a developer runs lint checks
- **THEN** linting SHALL be performed by Rust tooling

### Requirement: Restrict problematic Inquirer imports

This requirement is removed; the system SHALL NOT impose TypeScript-specific import restrictions.
**Reason**: TypeScript codebase is removed; the Inquirer constraint is no longer relevant.
**Migration**: None.

#### Scenario: No inquirer restriction

- **WHEN** a developer modifies Rust code
- **THEN** there is no requirement to lint `@inquirer/*` imports
