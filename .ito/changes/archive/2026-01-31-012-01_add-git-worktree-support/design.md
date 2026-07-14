## Context

Git worktrees provide a native way to check out multiple branches from the same repository into separate directories. For Ito, this enables a clean separation between:

- A small, stable "control" directory (repo root) used to run Ito and read `.ito/` artifacts
- A `main` worktree used for day-to-day work on the default branch
- One worktree per change branch for implementation

The user also needs a deterministic way to keep local environment files available in new worktrees without committing them.

## Goals / Non-Goals

**Goals:**
- Provide an opt-in workspace layout that creates `./main` and per-change worktrees under the repo root.
- Ensure Ito can tell the agent exactly which directory to work in for a given change.
- Copy a configurable set of local environment files into new change worktrees (initial defaults: `.env`, `.envrc`, Mise local config).

**Non-Goals:**
- A full GitHub-aware cloning command in this change (e.g., discovering default branch via GitHub API).
- Managing secret values themselves (Ito only copies files; it does not parse or store secret contents).
- Universal support for every ecosystem's local config conventions in v1 (provide configuration hooks instead).

## Decisions

- **Layout**: Use repo root as the Ito control directory, create `./main` for default branch, and create `./changes/<change-id>` for change worktrees.
- **Default branch**: Prefer configured value; otherwise `main`, then fallback to `master` if needed.
- **Local file copying**: Copy from `./main` into the change worktree for files that exist. Treat missing files as non-fatal.
- **Deterministic scripts**: Include a copy/pasteable shell snippet in apply instructions so agents can execute a known-good sequence (`git worktree add`, branch creation, copy local files).
- **Configurability**: Keep defaults in global config and expose them through `ito config`.

## Risks / Trade-offs

- **Security risk**: Copying `.env`-style files can increase accidental exposure if users commit them. Mitigation: ensure recommended ignores are documented and keep behavior opt-in.
- **Repo assumptions**: Nested worktrees and sparse checkouts can be surprising. Mitigation: constrain to a simple directory convention and provide strong, deterministic instructions.
- **Cross-platform shell**: Scripts may differ across shells/OS. Mitigation: keep scripts POSIX-ish and add Rust-side implementation where possible.

## Migration Plan

- Ship as opt-in behavior (flag or config).
- Keep existing single-workdir behavior as default.
- Add tests around path computation, idempotent worktree creation, and local file copy behavior.

## Open Questions

- What is the canonical Mise local config filename to support by default (e.g., `mise.local.toml` vs `.mise.local.toml`)?
- Should Ito manage ignores for local files via `.git/info/exclude` instead of editing `.gitignore`?
