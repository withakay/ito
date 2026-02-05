## REMOVED Requirements

### Requirement: Package.json script execution

This requirement is removed; the system SHALL NOT require Bun for script execution.
**Reason**: Rust-only project; Bun is no longer supported.
**Migration**: Use `make` and `cargo` workflows.

#### Scenario: Bun script runner is not required

- **WHEN** a developer looks for `bun run <script-name>`
- **THEN** the repository SHALL NOT require Bun to build/test/lint Ito

### Requirement: Build workflow

This requirement is removed; the system SHALL build Ito using Rust tooling.
**Reason**: TypeScript build output (`dist/`) is removed.
**Migration**: Use `cargo build` via `make build`.

#### Scenario: Rust build replaces Bun build

- **WHEN** a developer runs `make build`
- **THEN** the build SHALL compile the Rust CLI from `ito-rs/`

### Requirement: Test workflow

This requirement is removed; the system SHALL run tests using Rust tooling.
**Reason**: Vitest-based tests are removed.
**Migration**: Use `cargo test --workspace` via `make test`.

#### Scenario: Rust tests replace vitest

- **WHEN** a developer runs `make test`
- **THEN** tests SHALL run using Rust tooling without Node/Bun

### Requirement: Linting workflow

This requirement is removed; the system SHALL run linting using Rust tooling.
**Reason**: Biome/ESLint-based linting is removed.
**Migration**: Use `cargo fmt` and `cargo clippy` via `make lint`.

#### Scenario: Rust lint replaces JS lint

- **WHEN** a developer runs `make lint`
- **THEN** linting SHALL run using Rust tooling without Node/Bun

### Requirement: Makefile integration

This requirement is removed; the Makefile MUST NOT invoke Bun.
**Reason**: Make targets no longer invoke Bun.
**Migration**: Make targets invoke Rust commands.

#### Scenario: Makefile uses Rust commands

- **WHEN** a developer runs `make build`, `make test`, or `make lint`
- **THEN** the Makefile SHALL run Rust commands (cargo) only

### Requirement: Executable execution via bunx

This requirement is removed; the system SHALL NOT require `bunx`.
**Reason**: No Node executables are required.
**Migration**: Use Rust-installed tooling where needed.

#### Scenario: No bunx usage

- **WHEN** a developer follows the documented workflows
- **THEN** they SHALL NOT need `bunx` for Ito

### Requirement: Development CLI workflow

This requirement is removed; the system SHALL NOT depend on `dist/` JavaScript artifacts.
**Reason**: The TypeScript CLI entrypoint is removed.
**Migration**: Build and run the Rust CLI.

#### Scenario: No dist-based CLI build

- **WHEN** a developer builds Ito
- **THEN** it SHALL NOT rely on `dist/cli/index.js`

### Requirement: Global development installation

This requirement is removed; the system SHALL provide a Rust-native installation path.
**Reason**: Bun global install is removed.
**Migration**: Use `make rust-install`.

#### Scenario: Rust install replaces bun global install

- **WHEN** a developer wants to install Ito locally
- **THEN** `make rust-install` SHALL install the `ito` binary
