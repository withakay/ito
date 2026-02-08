## ADDED Requirements

### Requirement: Change flags accept fuzzy change identifiers

Every CLI command that accepts `--change` or `--change-id` SHALL resolve the provided value using a shared change-target resolver.

The resolver SHALL accept non-canonical, partial inputs and resolve them to a canonical change ID when (and only when) the input yields a unique match.

Unless the command explicitly opts into searching archived changes, resolution SHALL search active changes only (excluding `.ito/changes/archive/`).

#### Scenario: Exact canonical change ID resolves

- **GIVEN** `.ito/changes/001-12_project-setup-wizard/` exists
- **WHEN** a user runs a command with `--change 001-12_project-setup-wizard`
- **THEN** the command resolves the change to `001-12_project-setup-wizard`

#### Scenario: Dropped leading zeros in module and change numbers resolve

- **GIVEN** `.ito/changes/001-12_project-setup-wizard/` exists
- **WHEN** a user runs a command with `--change 1-12_project-setup-wizard`
- **THEN** the command resolves the change to `001-12_project-setup-wizard`

#### Scenario: Numeric change identity without slug resolves when unique

- **GIVEN** `.ito/changes/001-12_project-setup-wizard/` exists
- **WHEN** a user runs a command with `--change 001-12`
- **THEN** the command resolves the change to `001-12_project-setup-wizard`

#### Scenario: Numeric identity resolves with dropped leading zeros when unique

- **GIVEN** `.ito/changes/001-12_project-setup-wizard/` exists
- **WHEN** a user runs a command with `--change 1-12`
- **THEN** the command resolves the change to `001-12_project-setup-wizard`

#### Scenario: Slug substring resolves when unique

- **GIVEN** `.ito/changes/001-12_project-setup-wizard/` exists
- **WHEN** a user runs a command with `--change setup-wizard`
- **THEN** the command resolves the change to `001-12_project-setup-wizard`

#### Scenario: Multi-token input resolves when unique

- **GIVEN** `.ito/changes/001-12_project-setup-wizard/` exists
- **WHEN** a user runs a command with `--change "setup wizard"`
- **THEN** the command resolves the change to `001-12_project-setup-wizard`

#### Scenario: Module-scoped fuzzy input resolves within the module

- **GIVEN** `.ito/changes/001-12_project-setup-wizard/` exists
- **AND** `.ito/changes/014-01_add-rust-crate-documentation/` exists
- **WHEN** a user runs a command with `--change "1:setup"`
- **THEN** the command resolves the change to `001-12_project-setup-wizard`

#### Scenario: Module-only input resolves only when unique

- **GIVEN** exactly one active change exists whose module is `001`
- **WHEN** a user runs a command with `--change 1`
- **THEN** the command resolves to that single change

#### Scenario: Module-only input is an ambiguity error when multiple exist

- **GIVEN** two or more active changes exist whose module is `001`
- **WHEN** a user runs a command with `--change 1`
- **THEN** the command fails with an ambiguity error
- **AND** the error suggests providing a more specific change id or slug

#### Scenario: Providing a change flag does not trigger interactive prompts

- **GIVEN** an interactive environment
- **WHEN** a user runs a command with `--change <value>`
- **THEN** the command SHALL NOT prompt for selection to resolve ambiguity
- **AND** ambiguity SHALL be reported as an error

#### Scenario: Archived changes are excluded by default

- **GIVEN** `.ito/changes/archive/2026-02-05-014-01_add-rust-crate-documentation/` exists
- **AND** `.ito/changes/014-01_add-rust-crate-documentation/` does NOT exist
- **WHEN** a user runs a command with `--change 014-01`
- **THEN** the command fails with a not-found error

#### Scenario: Commands may opt into matching archived changes

- **GIVEN** `.ito/changes/archive/2026-02-05-014-01_add-rust-crate-documentation/` exists
- **AND** the command explicitly opts into searching archived changes
- **WHEN** a user runs that command with `--change 014-01`
- **THEN** the command resolves the change to `014-01_add-rust-crate-documentation`

### Requirement: Ambiguity and not-found errors are actionable

When change resolution fails, the CLI SHALL fail with an actionable error.

#### Scenario: Ambiguous input produces an error with candidates

- **GIVEN** `.ito/changes/001-11_tdd-red-green-coverage-guidance/` exists
- **AND** `.ito/changes/001-12_project-setup-wizard/` exists
- **WHEN** a user runs a command with `--change 001-1`
- **THEN** the command fails with an ambiguity error
- **AND** the error includes a short list of candidate change IDs
- **AND** the error suggests disambiguating by providing a longer ID or the full canonical change ID

#### Scenario: Not-found input produces an error with suggestions

- **GIVEN** `.ito/changes/001-12_project-setup-wizard/` exists
- **WHEN** a user runs a command with `--change does-not-exist`
- **THEN** the command fails with a not-found error
- **AND** the error includes nearest-match suggestions when available
