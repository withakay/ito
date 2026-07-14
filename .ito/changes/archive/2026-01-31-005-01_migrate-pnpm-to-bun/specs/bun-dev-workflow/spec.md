## Purpose

Define the requirements for developer workflows using Bun for local development tasks including building, testing, and linting.

## ADDED Requirements

### Requirement: Package.json script execution

The system SHALL execute package.json scripts using Bun's runtime.

#### Scenario: Run named script

- **WHEN** a developer runs `bun run <script-name>`
- **THEN** Bun SHALL execute the script defined in `package.json` scripts section
- **AND** the script SHALL complete with appropriate exit code

### Requirement: Build workflow

The system SHALL support building the project using Bun.

#### Scenario: Run build locally

- **WHEN** a developer runs `bun run build`
- **THEN** the project SHALL compile TypeScript to JavaScript in `dist/`
- **AND** build artifacts SHALL be identical to pnpm build output

### Requirement: Test workflow

The system SHALL support running tests using Bun's command runner.

#### Scenario: Run tests locally

- **WHEN** a developer runs `bun run test`
- **THEN** vitest SHALL execute all test files
- **AND** test results SHALL be displayed

#### Scenario: Run tests in watch mode

- **WHEN** a developer runs `bun run test:watch`
- **THEN** vitest SHALL run in watch mode
- **AND** tests SHALL re-run on file changes

#### Scenario: Run tests with coverage

- **WHEN** a developer runs `bun run test:coverage`
- **THEN** vitest SHALL generate coverage reports
- **AND** coverage data SHALL be written to coverage directory

### Requirement: Linting workflow

The system SHALL support linting using Bun's command runner.

#### Scenario: Run linter locally

- **WHEN** a developer runs `bun run lint`
- **THEN** ESLint SHALL check all configured files
- **AND** linting errors SHALL be reported

### Requirement: Makefile integration

The system SHALL support common development tasks via Make targets using Bun.

#### Scenario: Make test target

- **WHEN** a developer runs `make test`
- **THEN** the Makefile SHALL invoke `bun run test`

#### Scenario: Make build target

- **WHEN** a developer runs `make build`
- **THEN** the Makefile SHALL invoke `bun run build`

#### Scenario: Make lint target

- **WHEN** a developer runs `make lint`
- **THEN** the Makefile SHALL invoke `bun run lint`

### Requirement: Executable execution via bunx

The system SHALL support running executables from node_modules using bunx.

#### Scenario: Run binary with bunx

- **WHEN** a developer runs `bunx <binary>`
- **THEN** Bun SHALL execute the binary from `node_modules/.bin/`
- **AND** the binary SHALL run with appropriate arguments

### Requirement: Development CLI workflow

Test helpers SHALL use Bun to build the CLI before testing.

#### Scenario: Test helper builds CLI

- **WHEN** test helper detects missing `dist/cli/index.js`
- **THEN** test helper SHALL run `bun run build`
- **AND** build SHALL complete before tests execute

### Requirement: Global development installation

The system SHALL support installing the local package globally for development testing.

#### Scenario: Install package globally for testing

- **WHEN** a developer runs `make dev-install`
- **THEN** the package SHALL be available globally via Bun's global install mechanism
- **AND** the `ito` command SHALL execute the local development version
