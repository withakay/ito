## REMOVED Requirements

### Requirement: Harness can run oracle and candidate

This requirement is removed; the test suite SHALL NOT depend on a TypeScript oracle.
**Reason**: TypeScript oracle is removed.
**Migration**: Use Rust tests as the primary validation surface.

#### Scenario: No oracle execution

- **WHEN** running the test suite
- **THEN** tests SHALL NOT execute a TypeScript CLI oracle

### Requirement: Parity comparisons are deterministic

This requirement is removed; parity comparisons SHALL NOT be required.
**Reason**: Parity harness is removed.
**Migration**: Ensure Rust tests avoid nondeterminism.

#### Scenario: Rust tests are deterministic

- **WHEN** Rust tests compare outputs
- **THEN** outputs are normalized as needed

### Requirement: Baseline parity tests exist

This requirement is removed; baseline behavior MUST be validated by Rust tests.
**Reason**: Parity tests are removed.
**Migration**: Add Rust tests for `--help` and `--version`.

#### Scenario: Rust tests cover baseline CLI behavior

- **WHEN** running `cargo test --workspace`
- **THEN** tests MUST cover `ito --help` and `ito --version`

### Requirement: Harness compares outputs and exit codes

This requirement is removed; stdout/stderr/exit codes MUST be asserted in Rust tests.
**Reason**: Parity harness is removed.
**Migration**: Covered by Rust tests.

#### Scenario: Rust asserts stdout/stderr/exit codes

- **WHEN** Rust tests run CLI commands
- **THEN** they MUST assert stdout/stderr and exit codes as needed

### Requirement: Harness supports fixture repositories

This requirement is removed; fixture-based testing MUST be implemented in Rust.
**Reason**: Parity harness is removed.
**Migration**: Use Rust fixture helpers.

#### Scenario: Rust tests run in fixtures

- **WHEN** Rust tests need isolated repos
- **THEN** they MUST use fixture repositories

### Requirement: Harness can test interactive flows via PTY

This requirement is removed; interactive testing SHALL be implemented without a TS oracle.
**Reason**: Parity harness is removed.
**Migration**: Use Rust PTY testing where required.

#### Scenario: Rust PTY tests

- **WHEN** testing interactive commands
- **THEN** Rust tests MAY use PTY-based approaches
