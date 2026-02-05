# rust-port-research Specification

## Purpose

Define the Rust port strategy and produce research artifacts (including a parity matrix) that drive a byte-for-byte compatible Rust CLI implementation.

## Requirements

### Requirement: Research artifacts exist in canonical locations

The repository MUST include the Rust port research outputs at canonical locations used by downstream changes.

#### Scenario: Required documents are present

- **WHEN** an engineer navigates to `.ito/research/`
- **THEN** `.ito/research/SUMMARY.md` MUST exist
- **AND** `.ito/research/parity-matrix.md` MUST exist
- **AND** `.ito/research/investigations/rust-cli-ux.md` MUST exist
- **AND** `.ito/research/investigations/parity-testing.md` MUST exist
- **AND** `.ito/research/investigations/rust-crate-architecture.md` MUST exist
- **AND** `.ito/research/investigations/packaging-distribution.md` MUST exist

### Requirement: Parity matrix covers CLI surface and installed prompt outputs

The parity matrix MUST be complete enough to drive parity tests for every stable command and the outputs of installed prompts/templates.

#### Scenario: Parity matrix is reviewable and actionable

- **WHEN** `.ito/research/parity-matrix.md` is reviewed
- **THEN** it MUST enumerate stable commands and key flags
- **AND** it MUST describe JSON shapes, exit codes, and error text expectations where applicable
- **AND** it MUST call out filesystem effects for `init` and `update` (including installed prompt outputs)

### Requirement: Parity testing approach is executable and treats TypeScript as oracle

The research MUST specify a concrete parity testing approach that treats the TypeScript CLI as the behavior oracle.

#### Scenario: Testing strategy is concrete

- **WHEN** a developer reads `.ito/research/investigations/parity-testing.md`
- **THEN** it MUST specify how to execute the TypeScript CLI (oracle) and Rust CLI (candidate)
- **AND** it MUST specify how to compare stdout, stderr, and exit code deterministically
- **AND** it MUST specify how to compare filesystem outputs deterministically
- **AND** it MUST specify how to handle interactive flows via PTY where required

### Requirement: Research artifacts exist and are maintained

The repository MUST include the required research outputs for the Rust port and keep them consistent with the current TypeScript CLI behavior.

#### Scenario: Required research files are present

- **WHEN** a developer inspects `.ito/research/`
- **THEN** the following files exist:
  - `.ito/research/SUMMARY.md`
  - `.ito/research/investigations/rust-cli-ux.md`
  - `.ito/research/investigations/parity-testing.md`
  - `.ito/research/investigations/rust-crate-architecture.md`
  - `.ito/research/investigations/packaging-distribution.md`

### Requirement: Parity strategy treats TypeScript as the oracle

The parity strategy MUST treat the existing TypeScript `ito` CLI as the behavior oracle and define test mechanisms for stdout, stderr, exit code, JSON shapes, and filesystem side effects.

#### Scenario: Parity testing plan covers non-mutating and mutating commands

- **WHEN** reading `.ito/research/investigations/parity-testing.md`
- **THEN** it specifies how to compare:
  - non-mutating commands (help/version/list/show/validate)
  - mutating commands (init/update/installers)
  - interactive flows via PTY where required
