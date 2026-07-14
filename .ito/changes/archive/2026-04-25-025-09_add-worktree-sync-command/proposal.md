<!-- ITO:START -->
## Why

Coordination-worktree setups currently rely on scattered best-effort fetches and health checks, but there is no single command that validates `.ito/` wiring and pushes the coordination branch back to the remote. As more workflows run from sibling worktrees and skills call Ito commands frequently, that gap makes it easier for symlink drift, duplicate local state, stale coordination branches, and noisy repeated pushes to accumulate silently.

## What Changes

- Add a top-level `ito sync` command for worktree-backed coordination state.
- Have `ito sync` validate that `.ito/{changes,specs,modules,workflows,audit}` resolve to the expected coordination worktree targets before any remote sync happens.
- Have `ito sync` fetch coordination state, auto-commit pending coordination-worktree artifact changes when needed, and push valid local state to the configured remote.
- Track the last successful sync time and synchronized worktree state in repo-local metadata so redundant pushes can be skipped safely.
- Make the sync interval configurable, default it to 120 seconds, and add `ito sync --force` to bypass the quiet window when an immediate push is required.
- Keep automatic sync calls quiet and frequent by applying the sync interval only to redundant remote pushes while still running lightweight local validation on each invocation.
- Make worktree-mode archiving a two-stage flow: archive on the coordination branch first, sync that archive to other working copies, then integrate the archived result into `main` according to configuration.
- Add a configurable archive integration mode so Ito can archive with one of: direct merge to `main`, PR to `main`, PR with auto-merge, or coordination-only/manual main integration.
- Add an archive prompt to the `ito finish` workflow for completed changes so finish explicitly asks `Do you want to archive this change now?` and, when accepted, follows the coordination-first archive flow without lag.
- Keep sync/archive/finish workflow interactions driven by CLI-generated agent instructions and project templates, with skills and commands acting as thin wrappers around that source of truth.
- Add sync-specific agent guidance and update the relevant Ito skills/instructions so sync happens at mutation and handoff points without adding noisy output.

## Capabilities

### New Capabilities

- `cli-sync`: User-facing `ito sync` command for validating coordination-worktree wiring and synchronizing the coordination branch.

### Modified Capabilities

- `coordination-worktree`: Add stricter sync-time validation so `.ito/` links must resolve to the expected coordination worktree paths and duplicate local directories are treated as drift.
- `agent-instructions`: Add sync guidance so worktree-oriented agent workflows and skills invoke `ito sync` at the right points with quiet behavior.
- `config-defaults`: Add a default coordination sync interval for projects that do not configure one explicitly.
- `cli-config`: Allow the coordination sync interval to be configured via the existing config command surface.
- `cli-archive`: Add worktree-aware archive behavior that archives on the coordination branch first and then follows the configured `main`-integration policy.

## Impact

- CLI surface in `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/commands/mod.rs`, and a new sync command handler.
- Archive and finish command flow in `ito-rs/crates/ito-cli/src/commands/archive.rs`, finish/archive instruction generation, and related skill prompts.
- Coordination sync orchestration in `ito-rs/crates/ito-core/src/coordination.rs`, `ito-rs/crates/ito-core/src/coordination_worktree.rs`, and related git helpers.
- Config defaults and schema in `ito-rs/crates/ito-core/src/config/defaults.rs`, `ito-rs/crates/ito-config/**`, and `schemas/ito-config.schema.json`.
- Agent instruction generation in `ito-rs/crates/ito-cli/src/app/instructions.rs` and/or instruction template assets.
- Relevant skill prompts in `.opencode/skills/ito*/SKILL.md` and `.claude/skills/ito*/SKILL.md`.
<!-- ITO:END -->
