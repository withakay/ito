# Ito Lite Requirements And Specifications

This document collates the requirements, artifact formats, workflow rules, and validation expectations for a prompt-driven Ito Lite process. It is extracted from the full Ito change proposal workflow and rewritten so agents can operate using markdown only, without the `ito` executable or any other external dependency.

## Goal

Ito Lite SHALL provide a portable, prompt-only change proposal workflow that preserves the core Ito discipline:

- Current specs describe what is built.
- Active changes describe what should change.
- Proposal artifacts explain why the change exists.
- Spec deltas define required behavior.
- Tasks map implementation work to requirements.
- Review validates quality before implementation.
- Archive merges accepted deltas into current specs.

## Non-Goals

- Ito Lite does not provide a CLI, background service, remote coordination, worktree automation, or audit log implementation.
- Ito Lite does not require scripts, package managers, network access, shell commands, or executable installers.
- Ito Lite does not attempt to be byte-for-byte compatible with Ito-managed generated files.

## Repository Layout

Ito Lite SHOULD use `.ito-lite/` to avoid conflicting with full Ito installations:

```text
.ito-lite/
├── project.md
├── specs/
│   └── <capability>/
│       └── spec.md
├── modules/
│   └── <NNN_module-name>/
│       └── module.md
├── changes/
│   ├── <change-id>/
│   │   ├── proposal.md
│   │   ├── design.md
│   │   ├── tasks.md
│   │   ├── review.md
│   │   └── specs/
│   │       └── <capability>/
│   │           └── spec.md
│   └── archive/
│       └── YYYY-MM-DD-<change-id>/
└── wiki/
```

If a project is not using the real Ito executable and wants direct compatibility with Ito paths, `.ito/` MAY be used instead of `.ito-lite/`.

## Workflow Stages

### Stage 1: Setup

Project setup SHALL create `.ito-lite/project.md` and the base directories. Setup is complete when `project.md` contains:

```markdown
<!-- ITO-LITE:PROJECT_SETUP:COMPLETE -->
```

The project file SHOULD capture:

- What the project does.
- Who it is for.
- Primary stack and important frameworks.
- Build, test, lint, and run commands if available.
- The most important instruction for future AI assistants.

### Stage 2: Creating Changes

Create a change proposal for:

- New features or capabilities.
- Breaking API, schema, behavior, or configuration changes.
- Architecture or pattern changes.
- Security, performance, or migration behavior changes.
- Ambiguous work where a durable decision record would reduce risk.

Skip a proposal for:

- Typos, formatting, comments, and documentation-only corrections with no behavior change.
- Straightforward bug fixes that restore already-specified behavior.
- Non-breaking dependency updates.
- Tests for existing behavior.

When in doubt, create a proposal.

### Stage 3: Reviewing Changes

Before implementation, the proposal package SHOULD be reviewed for internal consistency, scope, testability, risk, and task quality.

Review verdicts are:

- `approve`
- `request-changes`
- `needs-discussion`

Implementation SHOULD NOT begin until blocking review findings are resolved or explicitly accepted by the user.

### Stage 4: Implementing Changes

Implementation SHALL be task-driven. The agent MUST read `proposal.md`, `design.md` when present, spec deltas, and `tasks.md` before editing code.

Task statuses are manual markdown states:

- `[ ] pending`
- `[>] in-progress`
- `[x] complete`
- `[-] shelved`

Agents SHOULD keep exactly one task in progress at a time unless the user explicitly asks for parallel work.

### Stage 5: Archiving Changes

After implementation and verification, archiving SHALL:

- Confirm every required task is complete or explicitly shelved with a reason.
- Merge deltas from `changes/<change-id>/specs/` into `.ito-lite/specs/`.
- Move `changes/<change-id>/` to `changes/archive/YYYY-MM-DD-<change-id>/`.
- Record archive notes in the archived change.
- Refresh any `.ito-lite/wiki/` synthesis if present.

## Change IDs

Change IDs SHOULD be unique, kebab-case, and verb-led:

- `add-two-factor-auth`
- `update-export-format`
- `remove-legacy-login`
- `refactor-plugin-loading`

When using modules, use `NNN-CC_change-name`:

- `001-01_init-repo`
- `012-06_update-agent-routing`

`NNN` is a three-digit module ID. `CC` is a two-digit change number within the module. Module `000` means standalone or ungrouped.

## Proposal Artifact

Path:

```text
.ito-lite/changes/<change-id>/proposal.md
```

Required sections:

- `## Why`
- `## What Changes`
- `## Capabilities`
- `## Impact`

Optional section:

- `## Change Shape`

Template:

```markdown
# Change: <brief title>

## Why

<1-2 paragraphs explaining the problem or opportunity.>

## What Changes

- <specific change>
- <mark breaking changes with **BREAKING**>

## Change Shape

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

## Spec Delta Artifact

Path:

```text
.ito-lite/changes/<change-id>/specs/<capability>/spec.md
```

Allowed delta headers:

- `## ADDED Requirements`
- `## MODIFIED Requirements`
- `## REMOVED Requirements`
- `## RENAMED Requirements`

Every requirement SHALL use:

```markdown
### Requirement: <name>
```

Every requirement SHALL have at least one scenario using exactly four hash marks:

```markdown
#### Scenario: <name>
```

Normative requirement text SHOULD use `SHALL` or `MUST`. Avoid `should` or `may` unless intentionally non-normative.

### Requirement IDs

Requirement IDs are optional. If any requirement in a change includes a Requirement ID, all requirements in that change MUST include one.

Format:

```markdown
- **Requirement ID**: <capability>:<requirement-name>
```

### ADDED Requirements

Use `ADDED` when introducing a new standalone behavior or sub-capability.

```markdown
## ADDED Requirements

### Requirement: Two-Factor Authentication

The system SHALL require a second factor during login when two-factor authentication is enabled for the account.

- **Requirement ID**: auth:two-factor-authentication

#### Scenario: OTP challenge required

- **WHEN** valid primary credentials are submitted for an account with two-factor authentication enabled
- **THEN** the system SHALL require an OTP challenge before completing login
```

### MODIFIED Requirements

Use `MODIFIED` when changing behavior, scope, or acceptance criteria for an existing requirement. The modified block MUST include the full updated requirement, not a partial fragment.

Workflow:

1. Locate the existing requirement in `.ito-lite/specs/<capability>/spec.md`.
2. Copy the entire requirement block from `### Requirement:` through all scenarios.
3. Paste it under `## MODIFIED Requirements`.
4. Edit the full block to describe the new intended behavior.
5. Keep the requirement header text matching the current requirement unless also using `RENAMED`.

### REMOVED Requirements

Use `REMOVED` when behavior is intentionally deprecated or deleted. Include `Reason` and `Migration`.

```markdown
## REMOVED Requirements

### Requirement: Legacy Export

**Reason**: Replaced by CSV and JSON export requirements.
**Migration**: Existing callers SHALL use `/api/v2/export`.
```

### RENAMED Requirements

Use `RENAMED` for name-only changes. If behavior changes too, combine `RENAMED` with a `MODIFIED` block using the new name.

```markdown
## RENAMED Requirements

- FROM: `### Requirement: Login`
- TO: `### Requirement: User Authentication`
```

## Design Artifact

Path:

```text
.ito-lite/changes/<change-id>/design.md
```

Create design only when at least one applies:

- Cross-cutting change across multiple modules, services, or subsystems.
- New architectural pattern.
- New external dependency.
- Significant data model changes.
- Security, performance, migration, or rollback complexity.
- Ambiguity that benefits from technical decisions before coding.

Template:

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

## Tasks Artifact

Path:

```text
.ito-lite/changes/<change-id>/tasks.md
```

Tasks SHOULD use waves and metadata:

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

Task quality requirements:

- Tasks are small, actionable, and independently reviewable.
- Every task has concrete `Done When` criteria.
- Every task has `Verify`, even if verification is a manual inspection checklist.
- Tasks map to one or more requirements when Requirement IDs are used.
- Dependencies are realistic and acyclic.

## Manual Validation Checklist

Use this checklist in place of `ito validate --strict`.

### Structure

- [ ] `.ito-lite/project.md` exists and setup is complete.
- [ ] Change directory exists under `.ito-lite/changes/<change-id>/`.
- [ ] `proposal.md` exists.
- [ ] `tasks.md` exists.
- [ ] At least one delta spec exists under `specs/<capability>/spec.md`.
- [ ] `design.md` exists when the proposal says design is needed.

### Proposal

- [ ] `Why` explains the problem or opportunity.
- [ ] `What Changes` is specific and bounded.
- [ ] Breaking changes are marked `**BREAKING**`.
- [ ] Capabilities list matches spec delta directories.
- [ ] Impact covers affected code, contracts, dependencies, and risk.

### Specs

- [ ] Only allowed delta headers are used.
- [ ] Every requirement starts with `### Requirement:`.
- [ ] Every requirement has at least one `#### Scenario:`.
- [ ] Scenarios use clear `WHEN` and `THEN` bullets.
- [ ] Normative requirements use `SHALL` or `MUST`.
- [ ] `MODIFIED` requirements include full updated blocks.
- [ ] `REMOVED` requirements include `Reason` and `Migration`.
- [ ] Requirement ID usage is consistent across the whole change.
- [ ] Requirements do not contradict current specs or each other.

### Design

- [ ] Design exists if risk, migration, external dependencies, or architecture decisions warrant it.
- [ ] Decisions include rationale and alternatives.
- [ ] Risks include mitigations.
- [ ] Migration and rollback are covered or explicitly not required.

### Tasks

- [ ] Tasks cover every requirement.
- [ ] Tasks include files, dependencies, action, verify, done-when, and status.
- [ ] Task order supports incremental delivery.
- [ ] Verification commands or manual checks are concrete.
- [ ] No task is too broad to review.

## Manual Traceability Checklist

When Requirement IDs are used:

- [ ] List every `- **Requirement ID**` from all delta specs.
- [ ] List every task `- **Requirements**` value.
- [ ] Confirm every requirement ID appears in at least one task.
- [ ] Confirm every task requirement ID exists in a delta spec.

## Manual Archive Merge Rules

When archiving, merge deltas into `.ito-lite/specs/<capability>/spec.md`:

- `ADDED`: append each new requirement to the capability spec.
- `MODIFIED`: replace the matching existing requirement block with the full modified block.
- `REMOVED`: remove the matching requirement block from the current spec and preserve removal rationale in the archived change.
- `RENAMED`: rename the matching requirement header; if behavior also changed, apply the corresponding `MODIFIED` block after renaming.

After merging:

- [ ] Re-run the manual validation checklist against current specs.
- [ ] Move the change directory to `.ito-lite/changes/archive/YYYY-MM-DD-<change-id>/`.
- [ ] Add archive notes summarizing what was merged, what was skipped, and any follow-up.

## Skill Requirements

The portable skill set SHALL:

- Contain only markdown files.
- Avoid scripts, shell commands, package managers, network access, and executable dependencies.
- Use Agent Skills-compatible frontmatter with `name` and `description`.
- Keep skills copyable as directories.
- Provide enough templates and checklists for agents to operate without the Ito CLI.
- Default to `.ito-lite/` paths and allow `.ito/` only by explicit project choice.

## Portable Agent Requirements

Ito Lite SHALL include prompt-only replicas of the canonical agent surfaces that full Ito creates.

### Agent Inventory

The bundle SHALL include these agent prompts:

- `ito-quick`: fast delegated small-task agent.
- `ito-general`: balanced direct development agent.
- `ito-thinking`: high-capability reasoning and architecture agent.
- `ito-orchestrator`: coordinator-only multi-change orchestration agent.
- `ito-planner`: delegated planning agent for run order, gates, dependencies, and blockers.
- `ito-researcher`: read-only context gathering agent.
- `ito-worker`: delegated implementation/remediation work-packet agent.
- `ito-reviewer`: read-only reviewer for gate evidence and worker output.
- `ito-test-runner`: non-mutating curated test runner.

### Agent Adaptation Rules

Ito Lite agent replicas SHALL:

- Be markdown-only prompt files.
- Avoid `ito agent instruction`, `ito patch`, `ito write`, `ito tasks`, and all other Ito executable commands.
- Default to `.ito-lite/` artifacts.
- Preserve the original role boundaries: researcher/reviewer/planner/orchestrator do not edit files; worker/general/quick/thinking may edit when assigned.
- Treat `.ito-lite/changes/<change-id>/` as active proposed work and `.ito-lite/specs/` as current truth.
- Keep current specs unchanged during implementation; update current specs only during archive.
- Include clear output contracts so orchestrators can consume worker, reviewer, researcher, planner, and test-runner results.

### Harness Copying Rules

Agent prompts SHOULD be copyable into harness-specific project-agent directories with minimal frontmatter adaptation:

- OpenCode: `.opencode/agents/ito-*.md`
- Claude Code: `.claude/agents/ito-*.md`
- GitHub Copilot: `.github/agents/ito-*.md`
- Pi: `.pi/agents/ito-*.md`
- Codex-style skill agents: `.agents/skills/<agent-name>/SKILL.md`

If full Ito is installed in the target project, Ito Lite agent prompts SHOULD NOT overwrite Ito-managed agent files. Use `ito-lite-*` names or keep the prompts as reference material.
