## ADDED Requirements

### Requirement: Agent Workflow Documentation

The project SHALL provide comprehensive documentation of the actual implemented Ito workflow as used by AI coding agents in `docs/agent-workflow.md`.

#### Scenario: Document the actions-on-a-change model

- **WHEN** a user reads the agent workflow documentation
- **THEN** they SHALL understand the five core actions: proposal, research, apply, review, and archive
- **AND** they SHALL understand when to use each action

#### Scenario: Document slash commands

- **WHEN** a user reads the agent workflow documentation
- **THEN** they SHALL find documentation for each slash command (`/ito-proposal`, `/ito-apply`, `/ito-research`, `/ito-review`, `/ito-archive`)
- **AND** they SHALL understand the purpose and usage of each command

#### Scenario: Provide practical examples

- **WHEN** a user reads the agent workflow documentation
- **THEN** they SHALL find end-to-end examples showing the complete workflow from proposal creation to archiving
