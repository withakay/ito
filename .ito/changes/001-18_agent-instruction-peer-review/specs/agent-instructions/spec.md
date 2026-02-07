# Spec: agent-instructions

## Purpose

Extend the agent instruction dispatcher to handle the `review` instruction type as a special-cased handler, following the same pattern as `apply`, `bootstrap`, `project-setup`, and `new-proposal`.

## MODIFIED Requirements

### Requirement: Instruction dispatcher supports standard instruction types

The instruction dispatcher SHALL support the following special-cased instruction types: `bootstrap`, `project-setup`, `new-proposal` (proposal without --change), `apply`, and `review`. Each type SHALL have its own handler and Jinja2 template. Unknown instruction types SHALL fall through to schema-based artifact resolution.

#### Scenario: Review instruction dispatched

- **WHEN** an agent runs `ito agent instruction review --change <id>`
- **THEN** the dispatcher SHALL route to the `review` handler, NOT to the generic artifact resolver

#### Scenario: Review instruction requires change flag

- **WHEN** an agent runs `ito agent instruction review` without `--change`
- **THEN** the system SHALL return an error indicating that `--change` is required for review instructions
