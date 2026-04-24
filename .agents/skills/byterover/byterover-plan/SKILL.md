---
name: byterover-plan
description: "Create structured implementation plans informed by existing knowledge. Queries architecture, conventions, and related implementations, then produces a goal-backward task breakdown with dependencies. Stores the plan via brv curate."
---

# ByteRover Context-Aware Planning

A structured workflow for creating implementation plans that leverage ByteRover's existing knowledge. Uses goal-backward methodology to derive concrete tasks from desired outcomes.

## When to Use

- Before implementing a new feature
- When tackling a complex task that needs decomposition
- When planning a refactor across multiple files
- When scope is unclear and needs structured analysis

## Prerequisites

Run `brv status` first. If errors occur, instruct the user to resolve them in the brv terminal. See the byterover skill's TROUBLESHOOTING.md for details.

The user must describe what they want to build or change.

## Process

### Phase 1: Gather Existing Context

Query the knowledge base for everything relevant to the planned work:

```bash
brv query "What is the architecture and structure of [affected area]?"
brv query "What conventions and patterns are used for [type of work: API, UI, data, etc.]?"
brv query "Are there existing implementations similar to [planned feature]?"
brv query "What concerns or tech debt exist in [affected area]?"
brv query "What testing approach is used for [type of work]?"
```

Adapt queries to the specific task. The goal is to understand:
- What patterns already exist that should be followed
- What concerns or constraints exist in the affected area
- What similar work has been done before

### Phase 2: Goal-Backward Analysis

Start from the desired outcome, not from tasks:

**1. State the goal as an outcome:**
> "When this is done, [observable result]"

**2. Derive observable truths (3-7):**
For each truth, ask: "What must be TRUE for this outcome to exist?"

Example:
> Goal: "Users can reset their password via email"
> - Truth 1: Password reset endpoint exists and validates tokens
> - Truth 2: Email service sends reset links with time-limited tokens
> - Truth 3: Reset form accepts new password and calls the endpoint
> - Truth 4: Tests verify the full flow (request → email → reset → login)

**3. For each truth, derive required artifacts:**
- Specific files to create or modify
- Specific wiring (exports → imports, API → consumer)
- Specific tests

### Phase 3: Task Breakdown

Decompose into concrete, sequenced tasks (3-7 per plan):

For each task, specify:

```
Task: [Name]
Files: [Exact paths to create/modify]
Action: [What to implement and how — reference existing patterns from Phase 1]
Verify: [How to prove this task is complete — run test, check behavior]
Done: [Acceptance criteria — observable outcome]
Dependencies: [Which tasks must complete first]
```

**Sizing guidance:**
- Each task should be completable in 15-60 minutes
- If a task would take longer, break it down further
- Keep total plan to 3-7 tasks (if more needed, create multiple plans)

**Ordering rules:**
- Data models and interfaces first
- Implementation before tests (but tests in same plan)
- Backend before frontend for full-stack features

### Phase 4: Risk and Concern Check

Query known issues in the affected area:

```bash
brv query "What known issues or tech debt exist in [affected files/modules]?"
brv query "What are the common pitfalls for [type of work]?"
```

For each concern that overlaps with the plan:
- Assess whether the plan addresses it or makes it worse
- Adjust tasks if needed to mitigate risks
- Note unresolvable risks for the user

### Phase 5: Store the Plan

Curate the complete plan for future reference:

```bash
brv curate "Implementation plan for [feature name]: [number] tasks covering [brief scope]. Goal: [outcome]. Key files: [main files]. Approach: [architecture decision]" -f [key files referenced in plan]
```

If the plan is large, break into multiple curate commands:

```bash
brv curate "Plan [feature] task 1-3: [summary of first tasks]" -f [relevant files]
brv curate "Plan [feature] task 4-7: [summary of remaining tasks]" -f [relevant files]
```

### Completion

Present the plan to the user:

1. **Goal** — What this achieves (outcome statement)
2. **Context used** — What knowledge informed the plan (patterns followed, concerns addressed)
3. **Tasks** — Ordered list with files, actions, and verification
4. **Dependencies** — Which tasks depend on others
5. **Risks** — Known concerns and mitigations
6. **Estimated scope** — Number of files, rough complexity

Ask the user to review and approve before execution.

## Important Rules

1. **Plans must be executable** — Specific enough that a different agent could implement without clarification
2. **Every task needs file paths** — No vague "update the service" — specify exactly which files
3. **Goal-backward, not forward** — Derive tasks from outcomes, don't list tasks and hope they achieve the goal
4. **Reuse existing patterns** — Reference patterns from the knowledge base instead of inventing new ones
5. **Right-size tasks** — 3-7 tasks per plan, 15-60 minutes each
6. **Store the plan** — Always curate the plan via `brv curate` for future reference
7. **Max 5 files per curate** — Break down large plans into multiple curate operations
8. **Verify curations** — After storing critical context, run `brv curate view <logId>` to confirm what was stored (logId is printed by `brv curate` on completion). Run `brv curate view --help` to see all options.
