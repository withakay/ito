---
name: ito-lite-proposal
description: Create prompt-driven Ito Lite change proposal packages in markdown. Use for drafting proposal.md, spec deltas, optional design.md, and tasks.md without the ito CLI or validation executable.
compatibility: No external dependencies; markdown and file editing only.
---

# Ito Lite Proposal Authoring

Use this skill to create or update `.ito-lite/changes/<change-id>/` by writing markdown artifacts directly.

## Start With Scope

Do not create files until the change shape is clear enough. Ask the fewest questions needed to determine:

- What problem does this solve?
- Why now?
- What does success look like?
- What is in scope?
- What is explicitly out of scope?
- Is this a feature, fix, refactor, migration, contract change, or event-driven change?

If the request is vague, propose 2-3 interpretations and ask which one fits.

## Choose Schema

- `spec-driven`: new capabilities, cross-cutting behavior, architecture, or ambiguity.
- `minimalist`: bounded fixes and small low-risk support work.
- `tdd`: regression-first work.
- `event-driven`: event/message-centric workflows.

Record the schema in `proposal.md` under `Change Shape` or an opening note.

## Choose Change ID

Use a unique, kebab-case, verb-led ID:

- `add-two-factor-auth`
- `update-export-format`
- `remove-legacy-login`

If using modules, use `NNN-CC_change-name`, such as `001-01_init-repo`. Module `000` means standalone.

## Files To Create

```text
.ito-lite/changes/<change-id>/
├── proposal.md
├── tasks.md
├── design.md                # optional
└── specs/
    └── <capability>/
        └── spec.md
```

Create one spec delta file per affected capability.

## proposal.md Template

```markdown
# Change: <brief title>

## Why

<1-2 paragraphs explaining the problem or opportunity.>

## What Changes

- <specific change>
- <mark breaking changes with **BREAKING**>

## Change Shape

- **Schema**: <spec-driven|minimalist|tdd|event-driven>
- **Type**: <feature|fix|refactor|migration|contract|event-driven>
- **Risk**: <low|medium|high>
- **Stateful**: <yes|no>
- **Public Contract**: <none|openapi|jsonschema|asyncapi|cli|config>
- **Design Needed**: <yes|no>
- **Design Reason**: <why a design doc is or is not needed>

## Capabilities

### New Capabilities

- `<capability-name>`: <what this capability covers>

### Modified Capabilities

- `<existing-capability>`: <what requirement changes>

## Impact

- **Affected code**: <files/systems>
- **Affected APIs/contracts**: <none or list>
- **Dependencies**: <none or list>
- **Risk**: <risk and mitigation>
```

## Spec Delta Rules

Use only these headers:

- `## ADDED Requirements`
- `## MODIFIED Requirements`
- `## REMOVED Requirements`
- `## RENAMED Requirements`

Every requirement must have:

- `### Requirement: <name>`
- Normative text using `SHALL` or `MUST`.
- At least one `#### Scenario: <name>` using exactly four `#` characters.
- Clear `WHEN` and `THEN` bullets.

Requirement IDs are optional. If one requirement has an ID, every requirement in the change must have an ID.

## spec.md Template

```markdown
## ADDED Requirements

### Requirement: <requirement name>

The system SHALL <required behavior>.

- **Requirement ID**: <capability>:<requirement-name>

#### Scenario: <scenario name>

- **WHEN** <condition or trigger>
- **THEN** <expected outcome>
```

For `MODIFIED`, copy the entire existing requirement block from `.ito-lite/specs/<capability>/spec.md` before editing it. Do not write partial deltas.

For `REMOVED`, include:

```markdown
**Reason**: <why removing>
**Migration**: <what users or systems should do instead>
```

For `RENAMED`, use:

```markdown
## RENAMED Requirements

- FROM: `### Requirement: <old name>`
- TO: `### Requirement: <new name>`
```

## design.md Template

Create `design.md` only when architecture, migration, external dependencies, security, performance, data model changes, or ambiguity warrant it.

```markdown
## Context

<background, current state, constraints, stakeholders>

## Goals / Non-Goals

- **Goals**: <list>
- **Non-Goals**: <list>

## Decisions

### Decision: <name>

- **Choice**: <what will be done>
- **Rationale**: <why>
- **Alternatives Considered**: <options and why rejected>

## Risks / Trade-offs

- <Risk> -> <Mitigation>

## Migration Plan

<steps and rollback, or "Not required">

## Open Questions

- <question or "None">
```

## tasks.md Template

```markdown
# Tasks: <change-id>

## Execution Notes

- **Tracking**: Manual markdown statuses only
- **Status legend**: `[ ] pending` | `[>] in-progress` | `[x] complete` | `[-] shelved`

## Wave 1: <name>
- **Depends On**: none

### Task 1.1: <task name>

- **Status**: [ ] pending
- **Updated At**: YYYY-MM-DD
- **Description**: <what this task does>
- **Files**: <paths or TBD>
- **Dependencies**: None
- **Action**: <specific implementation action>
- **Verify**: <specific verification command or manual check>
- **Done When**: <observable completion criteria>
- **Requirements**: <requirement ids or N/A>
```

## Self-Validation Before Presenting

Apply this checklist before telling the user the proposal package is ready:

- Proposal has Why, What Changes, Capabilities, Impact.
- Capabilities in proposal match spec delta directories.
- Every requirement has at least one correctly formatted scenario.
- `MODIFIED` requirements are full replacement blocks.
- Requirement IDs are consistent.
- Tasks map to requirements and have concrete verification.
- Design exists if the change is risky, architectural, stateful, or contract-facing.
