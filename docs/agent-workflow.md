# Agent Workflow (Human Guide)

This page explains how to work with an agent harness (OpenCode, Claude Code, etc.) when contributing to this repository.

The goal is to keep agent work aligned with the repo's spec-driven workflow, without requiring you to memorize Ito internals.

## Source of truth

When something disagrees, treat these as canonical:

- `AGENTS.md`
- `.ito/AGENTS.md`

This page is an overview and a set of practical prompts/checklists for humans.

## Mental model

- You provide intent, constraints, and review.
- The agent reads the repo instructions, then uses Ito CLI instruction artifacts to stay on-rails.
- The agent should prefer the harness file tools (read/edit/search) and use shell commands for builds/tests.

## Typical contributor flow with an agent

### 0) Clear legacy coordination state

If Ito warns that coordination state is legacy or ambiguous, read-only inspection remains available but mutation is intentionally blocked. Before creating or applying a proposal, ask the agent to run:

```bash
ito agent instruction migrate-to-main
```

Follow the emitted non-destructive migration procedure. It inventories and hashes the five managed Ito state directories, stops on conflicts, preserves the source coordination worktree for rollback, validates the embedded result, and integrates the reviewed migration according to repository policy. Do not begin implementation until the migration is on main.

### 1) Decide whether a proposal is needed

Use a proposal for new capabilities, breaking changes, architectural shifts, or significant security/perf changes.

If you're unsure, default to proposal-first. It is cheaper than a large rewrite.

### 2) Decide whether domain discovery is needed

Use the normal fast path for routine, one-context work with clear vocabulary.

Ask the agent to run DDD-oriented discovery before scaffolding when the work is ambiguous, architectural, cross-context, public-contract-changing, policy-heavy, sequencing-heavy, or uses overloaded domain terms. The discovery output should stay proportional:

- `direct`: no extra discovery for routine work.
- `lightweight`: resolve canonical terms, rejected aliases, and open questions.
- `bounded-context`: name primary/supporting contexts, model ownership, relationship pattern or provisional unknown, consistency expectations, and translation boundaries.
- `rigorous domain-grill`: challenge fuzzy plans one decision at a time, using repo evidence before asking questions.

For domain-heavy changes, expect the agent to carry the discovery handoff into proposal/spec/task language. Durable terms and boundaries can be proposed in `CONTEXT.md`, `CONTEXT-MAP.md`, or ADR files, but they become canonical only through review and archive/merge.

### 3) Pick or create a change

Humans usually start by asking the agent to discover existing work and scaffold the right thing:

```bash
ito list --specs
ito list --modules
ito list
```

Then either:

```bash
ito create module <name>
ito create change <name> --module <module-id>
```

### 4) Review and integrate the proposal

Treat the proposal package as the unit of review: proposal, delta specs, design, and tasks. Validate it before integration:

```bash
ito validate <change-id> --strict
```

The default `pull_request` mode expects that proposal-only package to be reviewed and merged through a PR into the target branch. Repositories that deliberately avoid a PR can opt into `changes.proposal.integration_mode = "direct_merge"` and merge the proposal-only commit into local main through their normal guarded workflow.

Do not begin implementation on the proposal branch. After integration, verify the exact authoritative Git tree that Ito will use:

```bash
ito change preflight <change-id> --for prepare --refresh
```

### 5) Use instruction artifacts to drive implementation

The most reliable way to keep an agent aligned is to have it fetch and follow the change-specific instructions:

```bash
ito agent instruction proposal --change <change-id>
ito agent instruction specs --change <change-id>
ito agent instruction tasks --change <change-id>
ito agent instruction apply --change <change-id>
```

The proposal/spec/design/task instructions author the review package. `apply` is available only after prepare readiness succeeds. As reviewer, expect the agent to identify the authority ref/OID and proposal integration OID before implementing.

Create or reuse the implementation worktree through the guarded command, which bases a new worktree on the captured authority OID and rejects a stale existing worktree:

```bash
CHANGE_DIR=$(ito worktree ensure --change <change-id>)
cd "$CHANGE_DIR"
ito change preflight <change-id> --for execute
```

### 6) Implement tasks and keep task state accurate

For enhanced `tasks.md`, prefer task commands so audit events stay consistent:

```bash
ito tasks status <change-id>
ito tasks next <change-id>
ito tasks start <change-id> <task-id>
ito tasks complete <change-id> <task-id>
```

Task start/complete, Ralph/loop iterations, and orchestration dispatch all enforce the same execute-readiness report before mutation. Iteration remains a default workflow option after that gate passes.

#### Migrating an in-flight change

For a change created before main-first enforcement, migrate in this order:

1. Stop implementation and preserve the current branch/worktree.
2. Split or identify the proposal-only commit containing the complete reviewed artifacts.
3. Review and integrate that proposal package into main using the configured PR or direct-merge mode.
4. Refresh authority and run prepare preflight.
5. Recreate or rebase the implementation worktree so its history contains the reported proposal integration OID, then run execute preflight.
6. Resume task or iteration work only after the gate passes.

Ito may emit an agent migration prompt when it detects legacy authority or coordination state. The prompt is guidance; inspect it and run the proposed migration rather than copying legacy state into the current checkout.

If someone edits `tasks.md` directly, reconcile immediately:

```bash
ito audit reconcile --fix
```

### 7) Validate before calling something done

At minimum:

```bash
ito validate <change-id> --strict
ito audit validate
ito audit reconcile
```

For changes with a `domain-discovery.md` handoff, also check that proposal validation either enables or intentionally skips these opt-in rules:

- `ubiquitous_language_consistency`
- `context_boundary_consistency`
- `domain_documentation_consistency`

### 8) Archive after merge/deploy

Before archive, confirm any approved domain-doc updates from the change package are promoted into the discovered `CONTEXT.md`, `CONTEXT-MAP.md`, or ADR locations. Do not promote rejected or unresolved discovery notes.

```bash
ito agent instruction archive --change <change-id>
ito archive <change-id>
```

## Worktrees (this repo)

This repo uses a bare/control repo with worktrees.

Rules of thumb:

- Do work inside a worktree (not the bare repo root).
- Let `ito worktree ensure --change <change-id>` create implementation worktrees under `ito-worktrees/`; do not bypass its captured-OID readiness check with a manual Worktrunk command.
- Do not remove the locked `main` worktree.

Also: when testing changes, use the binary built in the same worktree you edited (worktrees do not share `target/`).

## Agent Adapter Maintenance

After upgrading Ito-managed skills, commands, or agent templates, ask your agent to run `/ito-update-repo --dry-run`. This still refreshes managed files, but stops before deleting anything while it audits stale stamps, missing stamps, and orphaned Ito-managed assets.

## Practical prompting (what to ask the agent)

Good prompts include:

- The change id (or ask the agent to create one)
- What "done" means (behavior, tests, docs)
- Any hard constraints (no breaking changes, keep API stable, etc.)

Examples:

```text
Read `AGENTS.md` and `.ito/AGENTS.md`, then run `ito list` and tell me which change you will work in.
```

```text
For change 001-02_update-foo, fetch `ito agent instruction apply --change 001-02_update-foo` and follow it. Keep tasks updated via `ito tasks start/complete`.
```

```text
Before you say "done", show the exact verification commands you ran (and whether they passed).
```

## Known rough edges

- If you are running `make docs-site-serve`, do not run a clean rebuild in parallel. Stop the server first, then rebuild.

<!-- TODO (internal): add harness-specific tips for OpenCode vs Claude Code (how to paste instruction artifacts, how to preserve command outputs, etc.). -->
<!-- TODO (internal): add one small, stable example change id that new contributors can practice on. -->
