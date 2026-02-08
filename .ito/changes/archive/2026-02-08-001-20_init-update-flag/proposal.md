## Why

Running `ito init` on an existing project fails if files already exist (unless `--force` is used), but `--force` overwrites everything including user-edited files like `project.md` and `config.json`. Users need a way to re-run init to pick up new template files and refresh managed blocks without losing their customizations.

## What Changes

- Adds `--update` (`-u`) flag to `ito init` command
- Non-marker files that already exist are silently skipped instead of erroring
- Marker-managed files (AGENTS.md, CLAUDE.md, user-guidance.md) still get their managed blocks refreshed
- Marker-managed files without markers get the managed block prepended (preserving existing content)
- Adapter files, skills, and commands are overwritten as usual (they are ito-managed)
- Agent template files get their model field updated (like `ito update` behavior)
- New files that don't exist yet are created normally

## Capabilities

### New Capabilities

- None (this enhances an existing CLI command, no new spec-level capabilities)

### Modified Capabilities

- None (no spec-level behavior changes)

## Impact

- `ito-cli`: `InitArgs` struct gains `--update` flag, `handle_init`/`handle_init_clap` pass it through
- `ito-core`: `InitOptions` struct gains `update: bool` field, `write_one()` and `install_agent_templates()` use it to decide skip-vs-error behavior
- All callers of `InitOptions::new` updated to 3-arg form (init.rs, update.rs)
- 3 new integration tests added to `init_more.rs`
- 3 CLI help snapshots updated
