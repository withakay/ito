## Key points
- The `ito config` CLI is **global**: it reads and writes the user config at `~/.config/ito/config.json`.
- Repository behavior is driven by the **repo-local effective config** in `.ito/config.json`.
- Normal repo worktrees should explicitly set:
  - `changes.coordination_branch.enabled = true`
  - `changes.coordination_branch.name = ito/internal/changes`
  - `changes.coordination_branch.storage = worktree`
- The **coordination worktree** at `~/.local/share/ito/withakay/ito` must also keep the branch enabled and named `ito/internal/changes`, but use `storage = embedded`.
- This embedded storage setting avoids **self-symlink validation failures** because the coordination worktree is itself the storage target.
- The command supports common subcommands like `path`, `list`, `get`, `set`, `unset`, `schema`, and `help`.

## Structure / sections summary
- **Reason**: Explains that the document clarifies global vs repo-local config behavior and coordination worktree storage rules.
- **Raw Concept**: States the task, the intended config changes, the affected file (`.ito/config.json`), and the overall flow from global config to repo-local behavior.
- **Narrative**
  - **Structure**: Distinguishes the global CLI from repo-effective configuration.
  - **Dependencies**: Explains how normal worktrees depend on worktree-backed coordination storage, while the coordination worktree itself must be embedded.
  - **Highlights**: Lists supported config subcommands and notes the validation issue avoided by the embedded storage rule.
  - **Rules**: Gives explicit configuration requirements for normal worktrees and the coordination worktree.
  - **Examples**: Shows sample CLI usage (`ito config path`, `ito config get defaults.schema`, `ito config set defaults.schema "spec-driven"`).

## Notable entities, patterns, or decisions
- **Entities**
  - Global config file: `~/.config/ito/config.json`
  - Repo-local config file: `.ito/config.json`
  - Coordination worktree path: `~/.local/share/ito/withakay/ito`
  - Coordination branch name: `ito/internal/changes`
- **Pattern**
  - Split between **global user settings** and **repo-specific effective behavior**.
- **Decision**
  - Use `storage=worktree` for normal worktrees, but `storage=embedded` for the coordination worktree to prevent self-referential validation issues.
