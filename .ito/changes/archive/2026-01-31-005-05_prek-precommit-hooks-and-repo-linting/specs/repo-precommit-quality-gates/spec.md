## ADDED Requirements

### Requirement: Repo provides prek-compatible pre-commit config

The repository MUST include a `.pre-commit-config.yaml` compatible with `prek`.
The repository MUST document how to run hooks on-demand and how to install git hooks.

#### Scenario: Run hooks on demand

- **WHEN** a contributor runs `prek run --all-files`
- **THEN** the configured hooks run against the repository and exit successfully when the tree is clean

#### Scenario: Install git hooks

- **WHEN** a contributor runs `prek install`
- **THEN** future `git commit` executions invoke the configured hooks for the commit contents

### Requirement: Repo checks common file hygiene

The pre-commit configuration MUST include hooks to enforce common file hygiene.
At minimum, it MUST check and/or fix trailing whitespace, end-of-file newlines, and mixed line endings.

#### Scenario: Trailing whitespace is rejected or fixed

- **WHEN** a staged file contains trailing whitespace
- **THEN** the hook run fails or rewrites the file to remove trailing whitespace

### Requirement: Repo validates structured text formats

The pre-commit configuration MUST validate common structured formats used in the repo.
At minimum, it MUST validate JSON and YAML files.

#### Scenario: Invalid YAML is rejected

- **WHEN** a staged YAML file is syntactically invalid
- **THEN** the hook run fails and reports the file

### Requirement: Repo runs Rust formatting and linting hooks

The pre-commit configuration MUST run Rust formatting and linting checks consistent with the repo's supported workflow.

#### Scenario: Rust formatting is checked

- **WHEN** Rust sources are staged
- **THEN** the hook run checks formatting and fails if formatting is not compliant

#### Scenario: Rust clippy is checked

- **WHEN** Rust sources are staged
- **THEN** the hook run executes `cargo clippy` with the repo's defined lint policy and fails on violations
