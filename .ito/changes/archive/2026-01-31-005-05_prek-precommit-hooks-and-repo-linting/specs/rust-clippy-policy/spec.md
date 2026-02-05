## ADDED Requirements

### Requirement: Repo defines a consistent clippy policy

The repository MUST define a documented clippy policy that is run in local hooks and in CI.
The policy MUST be enforced consistently across the Rust workspace.

#### Scenario: Clippy policy runs in CI

- **WHEN** CI runs the repo lint workflow
- **THEN** `cargo clippy` runs with the same lint policy as local hooks and fails the job on violations

### Requirement: Clippy policy is curated and maintainable

The clippy policy MUST be curated to prioritize high-signal lints and allow local suppression when justified.
The repo MUST document how to add, remove, or locally allow specific lints.

#### Scenario: Local suppression is possible

- **WHEN** a lint is not appropriate for a specific code path
- **THEN** code can use `#[allow(clippy::<lint>)]` (with a brief justification) without disabling the policy globally

### Requirement: Clippy policy aligns with repo Rust style guidance

The clippy policy MUST enable lints (or configurations) that reinforce the repo's Rust style guidance where practical.

#### Scenario: Style-aligned lints are enabled

- **WHEN** the policy is evaluated
- **THEN** the enabled lint set includes style-aligned items where they provide clear signal and acceptable noise
