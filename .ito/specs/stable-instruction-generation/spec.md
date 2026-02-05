# Stable Instruction Generation Specification

## Purpose

Define the `stable-instruction-generation` capability: instruction artifacts must be consistent, testable, and configurable.

## Requirements

### Requirement: Instruction artifacts include configurable testing policy guidance

The CLI SHALL allow instruction artifacts to include a short "Testing Policy" guidance section that reflects project configuration.

#### Scenario: Proposal instructions show default testing policy

- **GIVEN** no explicit testing policy override is configured
- **WHEN** an agent runs `ito agent instruction proposal --change "<change-id>"`
- **THEN** the instruction output includes guidance to use a RED/GREEN/REFACTOR workflow
- **AND** it references a default coverage target of 80%

#### Scenario: Proposal instructions honor configured override

- **GIVEN** a project config source sets a coverage target override
- **WHEN** an agent runs `ito agent instruction proposal --change "<change-id>"`
- **THEN** the instruction output references the configured target instead of 80%
