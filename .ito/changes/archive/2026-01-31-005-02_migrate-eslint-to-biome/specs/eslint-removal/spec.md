# eslint-removal Specification

## Purpose

Remove ESLint tooling cleanly after migrating linting responsibilities to Biome.

## ADDED Requirements

### Requirement: ESLint is removed from dependencies and config

The project SHALL remove ESLint configuration and dependencies after Biome is adopted for linting.

#### Scenario: Tooling no longer references ESLint

- **WHEN** a developer inspects the repository configuration
- **THEN** `eslint` and `typescript-eslint` SHALL NOT be required for linting
- **AND** `eslint.config.js` SHALL NOT be present

### Requirement: Lint entrypoints remain stable

The project SHALL keep existing lint entrypoints (especially `bun run lint`) working and implemented via Biome.

#### Scenario: Existing entrypoints still work

- **WHEN** CI runs `bun run lint`
- **THEN** linting SHALL execute successfully using Biome
- **AND** the workflow SHALL NOT invoke ESLint
