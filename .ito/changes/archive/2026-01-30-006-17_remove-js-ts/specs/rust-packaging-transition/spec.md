## MODIFIED Requirements

### Requirement: Transition plan preserves `ito` command name

The transition plan MUST keep the user-facing `ito` command stable.

#### Scenario: Users can upgrade without changing command name

- GIVEN a user who previously installed Ito via any supported distribution method
- WHEN they upgrade to a Rust-only version
- THEN `ito --help` and `ito --version` behave consistently

## ADDED Requirements

### Requirement: Distribution does not require Node, Bun, or npm

Ito distribution and installation MUST NOT require Node.js, Bun, or npm.

#### Scenario: Install and run without Node

- GIVEN a machine without Node.js or Bun installed
- WHEN a user installs Ito via a Rust-native method (for example, prebuilt binaries or `cargo install`)
- THEN `ito --version` runs successfully
