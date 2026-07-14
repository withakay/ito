# Agent Workflow Docs Specification

## Purpose

Define the agent workflow documentation requirements so generated guidance is consistent and testable.

## Requirements

### Requirement: Workflow docs describe RED/GREEN and coverage target expectations

The agent workflow documentation SHALL describe the TDD RED/GREEN/REFACTOR loop and a default coverage target, including how to override targets per project.

#### Scenario: Agent workflow docs include testing loop and target

- **WHEN** a user reads the agent workflow documentation
- **THEN** they can find a section describing RED/GREEN/REFACTOR
- **AND** they can find a stated default coverage target (80%)
- **AND** they can find instructions on configuring overrides
