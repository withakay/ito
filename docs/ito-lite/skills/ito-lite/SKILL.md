---
name: ito-lite
description: Coordinate the prompt-only Ito Lite workflow for proposals, specs, tasks, reviews, implementation, and archive without the ito executable. Use when the user asks for Ito-style planning, change proposals, lightweight specs, or markdown-only change management.
compatibility: No external dependencies; markdown and file editing only.
---

# Ito Lite Coordinator

Ito Lite is a markdown-only version of the Ito change workflow. It preserves the core discipline of proposal-first changes without requiring any executable.

## Core Model

- `.ito-lite/specs/` is current truth: what the system is intended to do now.
- `.ito-lite/changes/<change-id>/` is proposed truth: what should change.
- `proposal.md` explains why.
- `specs/<capability>/spec.md` defines what with delta requirements.
- `design.md` explains how when the change is risky or architectural.
- `tasks.md` tracks implementation manually.
- Archive merges accepted deltas into current specs.
- `docs/ito-lite/agents/` contains prompt-only replicas of the Ito-created agent roles for delegation.

Use `.ito-lite/` by default. Use `.ito/` only if the user explicitly wants Ito-compatible paths and no Ito CLI will manage them.

## Route Requests

- Setup a project: use `ito-lite-setup`.
- Create or update a proposal package: use `ito-lite-proposal`.
- Review a proposal package before implementation: use `ito-lite-review`.
- Implement a change: use `ito-lite-apply`.
- Archive a completed change: use `ito-lite-archive`.

When delegating work, prefer the Ito Lite agent roles if they are available:

- `ito-quick` for small bounded edits.
- `ito-general` for default implementation and review work.
- `ito-thinking` for architecture, hard debugging, and complex planning.
- `ito-orchestrator` for multi-change coordination.
- `ito-planner`, `ito-researcher`, `ito-worker`, `ito-reviewer`, and `ito-test-runner` for orchestration sub-roles.

If no specialized skill is available, follow the workflows below directly.

## When To Create A Proposal

Create a proposal for:

- New capabilities or features.
- Breaking behavior, API, schema, CLI, or config changes.
- Architecture or pattern changes.
- Security, performance, state, or migration behavior changes.
- Ambiguous changes where specs reduce risk.

Skip a proposal for:

- Typos, formatting, comments, and simple docs edits with no behavior change.
- Straightforward bug fixes that restore already-specified behavior.
- Non-breaking dependency updates.
- Tests for existing behavior.

When in doubt, create a proposal.

## Schemas

Pick one workflow shape:

- `spec-driven`: default for new capabilities, broad changes, architecture, or ambiguity.
- `minimalist`: small bounded fixes and low-risk support work.
- `tdd`: regression-oriented work where a failing test should lead.
- `event-driven`: message/event-centric domains that need event discovery or AsyncAPI-style thinking.

Ito Lite stores schema choice as text in the proposal; it does not need a validator.

## Manual Validation Gate

Before implementation, validate the proposal package by checklist:

- Change directory exists under `.ito-lite/changes/<change-id>/`.
- `proposal.md` has Why, What Changes, Capabilities, and Impact.
- At least one delta spec exists under `specs/<capability>/spec.md`.
- Delta specs use only `ADDED`, `MODIFIED`, `REMOVED`, or `RENAMED Requirements` headers.
- Every requirement has `### Requirement:` and at least one `#### Scenario:`.
- Scenarios use clear `WHEN` and `THEN` behavior.
- `MODIFIED` requirements include the full updated requirement block.
- `REMOVED` requirements include Reason and Migration.
- Requirement IDs are all present or all omitted for the change.
- Tasks cover all requirement IDs when IDs are used.
- Design exists when the proposal says design is needed.

Do not claim validation passed unless this checklist was actually applied.

## Status Values

Manual task status values:

- `[ ] pending`
- `[>] in-progress`
- `[x] complete`
- `[-] shelved`

Keep exactly one task in progress unless the user explicitly asks for parallel execution.

## Archive Rule

Do not archive without explicit user confirmation. Archiving changes current specs, so treat it as an integration step.
