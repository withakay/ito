# Spec: agent-instructions

## Purpose

Define how the agent instruction dispatcher routes standard instruction artifacts and what behavior is guaranteed for key instruction entrypoints.

## Requirements

### Requirement: Instruction dispatcher supports standard instruction types

The instruction dispatcher SHALL support the following special-cased instruction types: `bootstrap`, `project-setup`, `schemas`, `new-proposal` (proposal without --change), `apply`, and `review`. Each type SHALL have its own handler and template. Unknown instruction types SHALL fall through to schema-based artifact resolution.

#### Scenario: Schemas instruction dispatched

- **WHEN** an agent runs `ito agent instruction schemas`
- **THEN** the dispatcher SHALL route to the `schemas` handler, NOT to the generic artifact resolver

#### Scenario: Schemas instruction supports JSON output

- **WHEN** an agent runs `ito agent instruction schemas --json`
- **THEN** the system SHALL return machine-readable JSON
- **AND** it SHALL include all embedded schema names and descriptions

#### Scenario: Review instruction dispatched

- **WHEN** an agent runs `ito agent instruction review --change <id>`
- **THEN** the dispatcher SHALL route to the `review` handler, NOT to the generic artifact resolver

#### Scenario: Review instruction requires change flag

- **WHEN** an agent runs `ito agent instruction review` without `--change`
- **THEN** the system SHALL return an error indicating that `--change` is required for review instructions
