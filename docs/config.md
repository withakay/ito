# Configuration

Ito reads configuration from a few different places, depending on what you're trying to control.

## Configuration Layers

Ito has a few different config locations with different purposes:

- **Repo root config**: `ito.json` and `.ito.json`
- **Project Ito-dir config**: `<ito-dir>/config.json`
- **User (global) config**: `<config-dir>/config.json`
- **Per-change metadata**: `<ito-dir>/changes/<change-id>/.ito.yaml`

## Project Config: `ito.json`

Location:

- `<repo-root>/ito.json`

Purpose:

- Configure project-level Ito behavior.
- Today, this file is primarily used to control the Ito working directory name.

This file also participates in cascading config merging (see `<ito-dir>/config.json`).

## Project Config: `.ito.json`

Location:

- `<repo-root>/.ito.json`

Purpose:

- Same role as `ito.json`, but higher precedence for repo-local overrides.

Supported keys:

- `projectPath` (string): overrides the directory name used for the Ito working directory in this repo.
  - Default is `.ito`.
  - This is useful for compatibility with older docs that refer to `ito/` instead of `.ito/`.

Example:

```json
{
  "projectPath": ".ito"
}
```

## Global Config: `config.json`

Location (directory):

- If `XDG_CONFIG_HOME` is set: `$XDG_CONFIG_HOME/ito/`
- Otherwise on Unix/macOS: `~/.config/ito/`
- On Windows: `%APPDATA%\ito\`

Location (file):

- `<config-dir>/config.json`

Environment variables used for resolution:

- `XDG_CONFIG_HOME`
- `HOME` (and `USERPROFILE` as a fallback)
- `APPDATA` (Windows)

Supported keys (Rust CLI today):

- `projectPath` (string): default Ito working directory name used when the repo does not have a `ito.json` override.

Ito directory name resolution for `projectPath` (highest precedence first):

1. `<repo-root>/.ito.json` `projectPath`
1. `<repo-root>/ito.json` `projectPath`
1. `<config-dir>/config.json` `projectPath`
1. default: `.ito`

Example:

```json
{
  "projectPath": ".ito"
}
```

Notes:

- If `config.json` is missing, Ito falls back to defaults.
- If `config.json` contains invalid JSON, Ito falls back to defaults (and prints a warning).

## Project Config: `<ito-dir>/config.json`

Location:

- `<ito-dir>/config.json` (default: `.ito/config.json`)

Purpose:

- Configure project-level Ito behavior (including AI tool and agent preferences).
- This file is intended to be checked into version control so teams get consistent behavior.

Cascading config (merged in order, later overrides earlier):

1. `<repo-root>/ito.json`
1. `<repo-root>/.ito.json`
1. `<ito-dir>/config.json`
1. If `PROJECT_DIR` is set: `$PROJECT_DIR/config.json`

Merge semantics:

- objects: recursively merged
- scalars: later source overrides earlier
- arrays: later source replaces earlier

Note: `projectPath` (the Ito working directory name) is resolved from repo-level config and global config. It does not consult `<ito-dir>/config.json` to avoid a resolution cycle.

### JSON schema metadata (`$schema`)

Generated project config includes a `$schema` field for editor autocomplete and validation.

- Canonical schema artifact path in this repo: `schemas/ito-config.schema.json`
- Generated config reference format:
  - `https://raw.githubusercontent.com/withakay/ito/v<version>/schemas/ito-config.schema.json`
- Runtime behavior:
  - Ito ignores the `$schema` field when loading config

Regenerate/update the committed schema artifact:

```bash
make config-schema
```

Verify the committed schema is current (used by CI):

```bash
make config-schema-check
```

Related commands (planned):

```bash
ito agent-config init
ito agent-config summary
ito agent-config get tools.opencode.default_model
ito agent-config set tools.opencode.context_budget 100000
```

## Per-Change Metadata: `.ito.yaml`

Location:

- `<repo-root>/.ito/changes/<change-id>/.ito.yaml`

Purpose:

- Store per-change metadata (such as schema choice) alongside the change.

Common fields:

- `schema` (string): workflow schema for the change (e.g. `spec-driven`).
- `created` (date): creation date (`YYYY-MM-DD`).

Optional fields used by validation:

- `ignore_warnings` (array of strings): suppress specific validator warnings for this change.
  - Example: `ignore_warnings: ["max_deltas"]`

Example:

```yaml
schema: spec-driven
created: 2026-01-31
ignore_warnings: ["max_deltas"]
```

## Where To Put Extra Guidance (Avoiding Overwrites)

Some files are installed/updated by Ito (`ito init`, `ito update`) and may be overwritten.

If you want to add project-specific guidance for humans and LLMs, prefer:

- `.ito/user-prompts/guidance.md` (shared guidance injected into agent instruction outputs)
- `.ito/user-prompts/<artifact-id>.md` (artifact-specific guidance, e.g. `proposal.md`, `apply.md`)
- `.ito/user-guidance.md` (legacy shared guidance fallback)
- `AGENTS.md` and/or `CLAUDE.md` (project-level guidance)
