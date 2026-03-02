## Context

`ito init` includes an interactive worktree setup wizard that persists answers to config. Today, even when config already contains the relevant worktree keys, the wizard prompts as if nothing is configured. This creates unnecessary friction when re-running init.

## Goals / Non-Goals

**Goals:**

- If relevant config keys already exist, interactive init pre-fills the wizard defaults from resolved config.
- A user can accept existing values without re-selecting options (Enter/Next).
- Init only persists config keys when the user actually changes a value.

**Non-Goals:**

- Change config precedence rules or introduce new config files.
- Expand the wizard scope beyond existing worktree-related prompts in this change.

## Decisions

- Decision: Use resolved config as the source of defaults.
  - Rationale: respects the existing cascade (project overrides global) and avoids guessing based on which files exist.

- Decision: Treat "accept default" as a no-op for persistence.
  - Rationale: avoids rewriting config files and makes repeated `ito init` invocations stable.

- Decision: Missing keys still trigger prompts.
  - Rationale: preserves the wizard's role in first-time setup.

## Risks / Trade-offs

- [Ambiguity about which config file is the source] -> Always compute defaults from resolved config and keep the existing persistence target behavior unchanged.
- [User wants to re-evaluate choices] -> Defaults are shown and can be changed; accept-default keeps behavior minimal.
