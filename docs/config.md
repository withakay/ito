# Configuration

Ito configuration is JSON-based and merges multiple sources (defaults, global config, and project config).

When in doubt, treat `schemas/ito-config.schema.json` as the source of truth for available keys and types.

## Configuration Files

### Global config (`config.json`)

This is per-user and applies to all projects.

- Path: run `ito config path`
- Default locations:
  - `$XDG_CONFIG_HOME/ito/config.json` (if `XDG_CONFIG_HOME` is set)
  - `~/.config/ito/config.json` (macOS/Linux)
  - `%APPDATA%\ito\config.json` (Windows)

You can manage this file via the CLI:

```bash
ito config list
ito config get worktrees.enabled
ito config set worktrees.enabled true
ito config unset worktrees.enabled
ito config schema
```

Notes:

- Values passed to `ito config set` are parsed as JSON by default. Use `--string` to force a string value.
- If the file is missing, Ito falls back to defaults.

### Repo root config (`ito.json` and `.ito.json`)

Optional project config files at the repository root:

- `ito.json`
- `.ito.json` (higher precedence; useful for repo-local overrides)

These files participate in project-level config merging.

### Ito directory config (`.ito/config.json`)

This is the team/shared project configuration file and is intended to be committed.

- Path: `.ito/config.json`
- Use `$schema` for editor completion:

```json
{
  "$schema": "../schemas/ito-config.schema.json"
}
```

## Merge Order and Semantics

Ito loads defaults (built into the binary) and then merges config files.

At a high level:

1. Built-in defaults
2. Global config (`~/.config/ito/config.json`)
3. Project config cascade:
   - `ito.json`
   - `.ito.json`
   - `.ito/config.json`
   - `$PROJECT_DIR/config.json` (if `PROJECT_DIR` is set)

Merge semantics:

- objects: recursively merged
- scalars: later source overrides earlier
- arrays: later source replaces earlier

## Common Settings

### Worktrees

Worktree behavior is controlled by the `worktrees` object.

Common keys:

- `worktrees.enabled`
- `worktrees.strategy` (`checkout_subdir`, `checkout_siblings`, `bare_control_siblings`)
- `worktrees.layout.base_dir`
- `worktrees.layout.dir_name`
- `worktrees.apply.enabled`
- `worktrees.apply.integration_mode` (`commit_pr` or `merge_parent`)
- `worktrees.apply.copy_from_main` (array of glob strings)
- `worktrees.apply.setup_commands` (array of shell strings)
- `worktrees.default_branch`

Example (global config):

```json
{
  "worktrees": {
    "enabled": true,
    "strategy": "bare_control_siblings",
    "layout": { "dir_name": "ito-worktrees" },
    "apply": {
      "enabled": true,
      "integration_mode": "commit_pr",
      "copy_from_main": [".env", ".envrc", ".mise.local.toml"],
      "setup_commands": []
    },
    "default_branch": "main"
  }
}
```

### Harness and agent model selection

Agent harness preferences live under `harnesses.<harness-id>`. Supported harness IDs include:

- `opencode`
- `claude-code`
- `codex`
- `github-copilot`

Each harness can set:

- `provider` (optional constraint)
- `agents` mapping for `ito-quick`, `ito-general`, `ito-thinking`

Example:

```json
{
  "harnesses": {
    "opencode": {
      "agents": {
        "ito-quick": "anthropic/claude-haiku-4-5",
        "ito-general": { "model": "openai/gpt-5.2-codex", "variant": "high", "temperature": 0.3 },
        "ito-thinking": { "model": "openai/gpt-5.2-codex", "variant": "xhigh", "temperature": 0.5 }
      }
    }
  }
}
```

### Testing defaults

Project-wide testing defaults live under `defaults.testing`.

Keys include:

- `defaults.testing.coverage.target_percent`
- `defaults.testing.tdd.workflow`

### Cache

Cache settings live under `cache`:

- `cache.ttl_hours`

### Change coordination

Change coordination settings live under `changes.coordination_branch`:

- `changes.coordination_branch.enabled`
- `changes.coordination_branch.name`

## Ito Directory Name (`projectPath`)

`projectPath` controls the Ito working directory name (defaults to `.ito`).

Resolution precedence (highest first):

1. `.ito.json` `projectPath`
2. `ito.json` `projectPath`
3. global config `config.json` `projectPath`
4. default: `.ito`

Note: `projectPath` is intentionally not read from `.ito/config.json` to avoid a resolution cycle.

## Per-change metadata (`.ito.yaml`)

Each change has a small metadata file:

- Path: `.ito/changes/<change-id>/.ito.yaml`
- Common fields:
  - `schema` (string)
  - `created` (`YYYY-MM-DD`)
  - `ignore_warnings` (array of validator warning IDs)

Example:

```yaml
schema: spec-driven
created: 2026-01-31
ignore_warnings: ["max_deltas"]
```

## Avoiding template overwrites

Some files are installed/updated by `ito init` / `ito update` and may be overwritten.

For project-specific guidance, prefer:

- `.ito/user-prompts/guidance.md`
- `.ito/user-prompts/<artifact-id>.md`
- `AGENTS.md` (repo)
