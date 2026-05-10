# Spec Delta: rust-packaging-transition

## Purpose

Define how Rust Ito is packaged and distributed without breaking existing npm-based installs.

## ADDED Requirements

### Requirement: Transition plan preserves `ito` command name

The transition plan MUST keep the user-facing `ito` command stable.

#### Scenario: npm-installed `ito` continues to work

- GIVEN a user who previously installed `@withakay/ito`
- WHEN they upgrade to a version that uses Rust binaries
- THEN `ito --help` and `ito --version` behave identically

### Requirement: Platform artifacts and verification are defined

The plan MUST define build artifacts per platform and how they are verified.

#### Scenario: Release checklist is explicit

- GIVEN the packaging documentation
- WHEN a release engineer follows the checklist
- THEN it includes commands to build artifacts
- AND it includes checksum/integrity verification
