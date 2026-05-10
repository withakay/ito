## ADDED Requirements

### Requirement: Domain enums SHALL NOT depend on CLI frameworks

Domain-layer enums (`ito-core`, `ito-domain`) that represent closed sets of values SHALL implement `Display`, `FromStr`, and an iteration method using only `std` types. They SHALL NOT depend on `clap` or any other adapter-layer crate.

#### Scenario: HarnessName enum has no clap dependency

- **GIVEN** the `HarnessName` enum in `ito-core`
- **WHEN** compiling `ito-core` without the `clap` feature
- **THEN** the crate SHALL compile successfully
- **AND** `HarnessName` SHALL implement `Display` and `FromStr`
- **AND** `HarnessName` SHALL provide a `user_facing()` method returning an iterator over user-visible variants

### Requirement: Adapter-layer bridge enums SHALL derive ValueEnum

Each domain enum used as a CLI argument SHALL have a corresponding bridge enum in `ito-cli` that derives `clap::ValueEnum`. The bridge enum SHALL be connected to the domain enum via an exhaustive `From` implementation.

#### Scenario: HarnessArg bridges to HarnessName

- **GIVEN** `HarnessArg` in `ito-cli` deriving `clap::ValueEnum`
- **AND** `HarnessName` in `ito-core`
- **WHEN** a new variant is added to `HarnessName`
- **THEN** the `From<HarnessArg> for HarnessName` impl SHALL fail to compile until `HarnessArg` is updated
- **AND** vice versa

### Requirement: Internal-only variants SHALL be hidden from CLI help

Variants intended only for testing or internal use (e.g. `Stub`) SHALL be hidden from `--help` output and shell completions but SHALL remain accepted as input values.

#### Scenario: Stub harness hidden from help but accepted as input

- **GIVEN** the `--harness` flag on `ito ralph`
- **WHEN** a user runs `ito ralph --help`
- **THEN** `stub` SHALL NOT appear in the possible values
- **AND** running `ito ralph --harness stub` SHALL be accepted

### Requirement: CLI aliases SHALL be declared on the bridge enum

User-facing aliases (e.g. `copilot` for `github-copilot`) SHALL be declared as `#[value(alias = "...")]` on the bridge enum variant, not as separate match arms or constants.

#### Scenario: Copilot alias accepted via ValueEnum

- **WHEN** a user runs `ito ralph --harness copilot`
- **THEN** the system SHALL resolve this to the GitHub Copilot harness
- **AND** the alias SHALL be declared on the `HarnessArg` variant, not in application code

### Requirement: Unknown values SHALL produce clap-level errors with suggestions

When a user provides an invalid value for a bridged argument, clap SHALL reject it at parse time with the list of valid values, rather than the application producing a custom error after parsing.

#### Scenario: Invalid harness value rejected by clap

- **WHEN** a user runs `ito ralph --harness does-not-exist`
- **THEN** clap SHALL reject the value before application code runs
- **AND** the error message SHALL include the list of valid values
