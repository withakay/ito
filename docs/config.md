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

### Workflow Schemas

Ito ships embedded workflow schemas that can be used without installation:

- `spec-driven`: full proposal, specs, design, and tasks workflow.
- `minimalist`: OpenSpec-derived lightweight specs and tasks workflow.
- `event-driven`: OpenSpec-derived event-storming, event-modeling, specs, design, AsyncAPI, and tasks workflow.
- `tdd`: test-first workflow.

Project schemas under `.ito/templates/schemas/<name>/schema.yaml` take precedence over user-level schemas, and user-level schemas take precedence over embedded schemas. Export embedded schemas with:

```bash
ito templates schemas export --to .ito/templates/schemas
```

The embedded OpenSpec-derived schemas include Ito-authored `validation.yaml` files. They perform presence checks and task tracking validation, and emit an informational manual semantic-validation note because Ito does not yet implement full OpenSpec semantic validation for those workflows.

### Proposal integration

Ito expects a proposal to be reviewed and integrated into the configured target branch before implementation begins. `changes.proposal.integration_mode` selects the authority used to prove that hand-off:

- `pull_request` (default) uses the target branch's tracked upstream ref, normally `refs/remotes/origin/main`.
- `direct_merge` is an explicit opt-in that uses the local target branch, normally `refs/heads/main`.

```json
{
  "changes": {
    "proposal": {
      "integration_mode": "pull_request"
    }
  }
}
```

There is no fallback from a missing pull-request authority to local `main`. Repositories that deliberately integrate proposals without a remote pull-request workflow must select `direct_merge`.

The lifecycle is the same in both modes:

1. Author and strictly validate a proposal-only package.
2. Review and integrate that package into the configured target branch.
3. Run `ito change preflight <change-id> --for prepare --refresh`.
4. Create/reuse the implementation checkout with `ito worktree ensure --change <change-id>`.
5. Run implementation commands only after `ito change preflight <change-id> --for execute` passes.

`prepare` reads required artifacts directly from one captured authority commit. `execute` additionally proves that the selected change worktree contains the proposal integration commit and belongs to the full change ID. Local copies, coordination links, and backend state cannot satisfy either gate.

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

`ito init` uses existing project-local, project, or global worktree config as interactive defaults when present. `ito init` and `ito update` also accept worktree flags for non-interactive setup:

```bash
ito init --worktrees --worktree-strategy bare_control_siblings --worktree-integration-mode commit_pr
ito update --no-worktrees
```

Supported strategies are `checkout_subdir`, `checkout_siblings`, and `bare_control_siblings`. Supported integration modes are `commit_pr` and `merge_parent`.

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

### Backend API

Backend mode enables multi-agent coordination through a shared backend API. Settings live under `backend`:

- `backend.enabled` — Enable backend API integration (default: `false`)
- `backend.url` — Base URL for the backend API (default: `http://127.0.0.1:9010`)
- `backend.token` — Explicit authentication token (optional; overrides env var)
- `backend.token_env_var` — Environment variable holding the bearer token (default: `ITO_BACKEND_TOKEN`)
- `backend.backup_dir` — Directory for artifact backup snapshots during sync (default: `~/.ito/backups`)
- `backend.timeout_ms` — Request timeout in milliseconds (default: `30000`)
- `backend.max_retries` — Maximum retry attempts for transient failures (default: `3`)
- `backend.project.org` — Organization namespace for backend routes (required when `backend.enabled` is true; or via `ITO_BACKEND_PROJECT_ORG`)
- `backend.project.repo` — Repository namespace for backend routes (required when `backend.enabled` is true; or via `ITO_BACKEND_PROJECT_REPO`)

Example:

```json
{
  "backend": {
    "enabled": true,
    "url": "https://ito-backend.example.com",
    "token_env_var": "ITO_BACKEND_TOKEN",
    "project": {
      "org": "your-org",
      "repo": "your-repo"
    },
    "timeout_ms": 15000,
    "max_retries": 2
  }
}
```

See [Backend Client Mode](backend-client-mode.md) for usage and troubleshooting.

### Change coordination

Change coordination settings live under `changes.coordination_branch`:

- `changes.coordination_branch.enabled`
- `changes.coordination_branch.name`
- `changes.coordination_branch.storage`
- `changes.coordination_branch.worktree_path`

Coordination-worktree storage is a legacy layout. When Ito detects its configuration, managed links, or `.gitignore` markers, read-only commands emit a remediation warning and stateful commands stop before dispatch. Run `ito agent instruction migrate-to-main` (or the installed `/ito-migrate-to-main` prompt) to inventory, verify, and migrate the state into real directories on a reviewed main-bound branch. The source coordination worktree is retained as rollback evidence.

#### Legacy sync behavior

Older coordination-worktree projects may still contain automatic instruction
sync settings and the `ito sync` command. These are compatibility surfaces, not
a recommended workflow. While legacy or ambiguous evidence remains, the
pre-dispatch guard suppresses incidental sync for allowed reads and blocks
stateful commands. Do not pass `--sync` or run `ito sync` to deepen the legacy
state; render and follow `migrate-to-main` instead.

#### Instruction sync behavior

Apply rendering captures its proposal inputs from authoritative Git before any
optional coordination sync. Other change-scoped instructions retain their
legacy per-artifact coordination policy. `archive` and `finish` use dedicated
handlers, but they also run a best-effort compatibility sync before rendering.

| Artifact | Default sync | `--sync` flag |
| --- | --- | --- |
| `apply` | **No coordination sync** — renders accepted inputs from a captured authority commit | Opt-in compatibility sync after authority capture; does not refresh proposal authority |
| `proposal` | **Yes** — always fetches coordination state | Ignored (always syncs) |
| `review` | **Yes** — always fetches coordination state | Ignored (always syncs) |
| `archive`, `finish` | **Yes** — dedicated handlers always fetch coordination state | Ignored (always syncs) |
| Other (`specs`, `tasks`, `design`, …) | **No** | Ignored (never syncs) |

The `--sync` flag changes compatibility-sync behavior only for `apply`; it does
not change the captured proposal authority. Use
`ito change preflight <id> --for prepare --refresh` to refresh pull-request
authority, then rerun apply rendering. To refresh legacy modules/workflows/audit
state independently, run `ito sync` (a no-op unless coordination-worktree
storage is active).

### Agent memory

Agent memory is configured under the optional top-level `memory` section,
keyed by operation name. Each operation (`capture`, `search`, `query`) is
*independently optional* and picks one of two shapes:

- **`command` shape** — Ito renders a shell command line, substituting
  the operation's input placeholders at instruction-emission time.
- **`skill` shape** — Ito directs the agent to invoke an installed skill
  with structured inputs and an opaque `options` payload.

There is **no default provider**. A freshly initialized Ito project has
no `memory` section; running `ito agent instruction memory-capture` (or
`memory-search` / `memory-query`) on it prints provider-setup guidance
rather than failing.

#### Command-shape placeholder rules

When `kind` is `"command"`, Ito substitutes a fixed set of placeholders
into the configured `command` template:

| Placeholder | Type | Render rule |
| --- | --- | --- |
| `{context}` | scalar string (capture) | shell-quoted, empty when unset |
| `{query}` | scalar string (search, query) | shell-quoted |
| `{scope}` | scalar string (search) | shell-quoted, empty when unset |
| `{limit}` | integer (search) | decimal literal; default `10` for `memory-search` |
| `{files}` | list of paths (capture) | expands to repeated `--file 'path'` flags |
| `{folders}` | list of paths (capture) | expands to repeated `--folder 'path'` flags |

Unknown `{placeholder}` tokens are preserved literally — Ito does not
emit a validation error for them.

The rendered output is executable as-is; the agent does not perform
further substitution.

#### Worked example: command shape (generic)

```jsonc
{
  "memory": {
    "capture": {
      "kind": "command",
      "command": "<your-cli> store \"{context}\" {files} {folders}"
    },
    "search": {
      "kind": "command",
      "command": "<your-cli> search \"{query}\" --limit {limit}"
    },
    "query": {
      "kind": "command",
      "command": "<your-cli> ask \"{query}\""
    }
  }
}
```

(Replace `<your-cli>` with whatever the configured backend is — for
example, the ByteRover integration shipped by change
`029-01_add-byterover-integration` configures `brv curate / brv search /
brv query` via this exact shape.)

#### Worked example: skill shape (generic)

```jsonc
{
  "memory": {
    "capture": {
      "kind": "skill",
      "skill": "ito-memory-markdown",
      "options": { "root": ".ito/memories" }
    },
    "search": {
      "kind": "skill",
      "skill": "ito-memory-markdown"
    }
  }
}
```

Ito does not interpret the `options` map — it is passed through
verbatim to the configured skill, so each skill defines its own option
schema.

#### Mixing shapes

Operations are configured independently, so `capture` may use a skill
while `search` and `query` use inline commands:

```jsonc
{
  "memory": {
    "capture": { "kind": "skill", "skill": "ito-memory-markdown" },
    "search":  { "kind": "command", "command": "rg \"{query}\" .ito/memories" }
    // query intentionally omitted — `memory-query` will print setup
    // guidance until configured.
  }
}
```

#### Apply / finish reminders

When `memory.capture` is configured, both
`ito agent instruction apply --change <id>` and
`ito agent instruction finish --change <id>` append a *Capture memories*
reminder section. The reminder tells the agent what to capture
(decisions, gotchas, patterns) and how to invoke
`ito agent instruction memory-capture`. The reminder is omitted when
`memory.capture` is unconfigured, regardless of whether `search` or
`query` are configured.

#### Agent discoverability

Memory no longer adds a standalone Ito skill. `ito-research` owns configured
memory search/query and `ito-archive` owns durable capture follow-through.
Both remain provider-agnostic and route through `ito agent instruction
memory-capture`, `ito agent instruction memory-search`, and `ito agent
instruction memory-query`.

The memory instruction artifacts are also listed in `ito agent instruction
--help`, which is the main low-context discovery surface for LLM agents. Use
that help output when checking which instruction artifacts are available.

`finish` also always appends a *Refresh archive and specs* wrap-up
reminder covering specs / docs / archive checks. The archive check is
suppressed when finish has already prompted to run `ito archive`, so the
agent never sees the archive step listed twice in one finish output.

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
