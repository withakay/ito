## Purpose

Update `subagent-driven-development` skill to use ito workflow patterns, removing all deprecated references.

## MODIFIED Requirements

### Requirement: No superpowers references

The skill SHALL NOT reference deprecated `superpowers:*` skill syntax.

#### Scenario: Modern skill references

- **WHEN** the skill references other skills
- **THEN** it uses `ito-*` prefixed names without `superpowers:` prefix

### Requirement: References ito-apply-change-proposal for execution

The skill SHALL reference `ito-apply-change-proposal` for task execution instead of `executing-plans`.

#### Scenario: Execution handoff

- **WHEN** the skill describes how subagents execute tasks
- **THEN** it references `ito-apply-change-proposal`

### Requirement: References ito-write-change-proposal for planning

The skill SHALL reference `ito-write-change-proposal` for task creation instead of `writing-plans`.

#### Scenario: Planning reference

- **WHEN** the skill describes plan creation
- **THEN** it references `ito-write-change-proposal`

### Requirement: Uses ito tasks CLI for tracking

The skill SHALL use `ito tasks` CLI instead of TodoWrite.

#### Scenario: Task status updates

- **WHEN** the skill or subagents update task status
- **THEN** they use `ito tasks start/complete/shelve` commands

### Requirement: Uses ito change artifacts

The skill SHALL reference `.ito/changes/<id>/tasks.md` instead of `docs/plans/`.

#### Scenario: Task source

- **WHEN** the skill loads tasks
- **THEN** it reads from `.ito/changes/<id>/tasks.md`

### Requirement: Subagent context from ito CLI

The skill SHALL provide subagents with context from `ito agent instruction apply`.

#### Scenario: Subagent prompt

- **WHEN** the skill dispatches a subagent
- **THEN** the subagent receives context via `ito agent instruction apply --change <id>`
