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

### 1) Decide whether a proposal is needed

Use a proposal for new capabilities, breaking changes, architectural shifts, or significant security/perf changes.

If you're unsure, default to proposal-first. It is cheaper than a large rewrite.

### 2) Pick or create a change

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

### 3) Use instruction artifacts to drive execution

The most reliable way to keep an agent aligned is to have it fetch and follow the change-specific instructions:

```bash
ito agent instruction proposal --change <change-id>
ito agent instruction specs --change <change-id>
ito agent instruction tasks --change <change-id>
ito agent instruction apply --change <change-id>
```

As reviewer, you should expect the agent to quote the relevant parts of these instructions back to you (briefly) before implementing.

### 4) Implement tasks and keep task state accurate

For enhanced `tasks.md`, prefer task commands so audit events stay consistent:

```bash
ito tasks status <change-id>
ito tasks next <change-id>
ito tasks start <change-id> <task-id>
ito tasks complete <change-id> <task-id>
```

If someone edits `tasks.md` directly, reconcile immediately:

```bash
ito audit reconcile --fix
```

### 5) Validate before calling something done

At minimum:

```bash
ito validate <change-id> --strict
ito audit validate
ito audit reconcile
```

### 6) Archive after merge/deploy

```bash
ito agent instruction archive --change <change-id>
ito archive <change-id>
```

## Worktrees (this repo)

This repo uses a bare/control repo with worktrees.

Rules of thumb:

- Do work inside a worktree (not the bare repo root).
- Create feature worktrees under `ito-worktrees/`.
- Do not remove the locked `main` worktree.

Also: when testing changes, use the binary built in the same worktree you edited (worktrees do not share `target/`).

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

## Placeholders to fill

- Placeholder: add harness-specific tips for OpenCode vs Claude Code (how to paste instruction artifacts, how to preserve command outputs, etc.).
- Placeholder: add one small, stable example change id that new contributors can practice on.
