## Context

Ito already supports a worktree-oriented workspace flow, but the behavior is split across defaults and instruction templates with limited project-level control. Teams need a stable way to configure whether worktrees are used during `apply`, which local files are copied from `./main`, what one-time setup commands should run in the worktree, and how post-implementation integration should be guided.

This change targets configuration and instruction generation only. It does not redesign Git semantics or require a new long-running service.

## Goals / Non-Goals

**Goals:**

- Define a nested `worktrees` config object with explicit, typed keys for enablement, workflow strategy, apply behavior, copy patterns, setup commands, and integration preference.
- Keep current behavior backward-compatible when new keys are omitted.
- Make `ito agent instruction apply` deterministic and configuration-aware.
- Include cleanup guidance so merged worktree branches are easy to remove safely.

**Non-Goals:**

- Automatically deleting branches/worktrees without explicit user action.
- Designing a full interactive TUI for worktree lifecycle management.
- Replacing Git primitives with custom branch orchestration.

## Decisions

- Introduce a single policy namespace under `worktrees` rather than scattered top-level keys.
  - Rationale: keeps worktree behavior discoverable and extensible.
  - Alternative considered: continue adding independent top-level keys; rejected due to schema drift and poor discoverability.

- Add a strict workflow strategy enum (`worktrees.strategy`) for workspace topology so users can align Ito with known repo shapes.
  - Supported values:
    - `checkout_subdir`: standard checkout with change worktrees under a gitignored `.<dir_name>/` subdirectory inside the project (dot-prefixed, defaults to `.ito-worktrees/`).
    - `checkout_siblings`: standard checkout with change worktrees grouped in a dedicated `<project>-ito-worktrees/` sibling directory next to the checkout.
    - `bare_control_siblings`: bare/control repo with `main` as a worktree and change worktrees grouped in an `ito-worktrees/` subfolder inside the bare repo directory. This strategy models the common "bare clone" workflow where the repo is cloned with `git clone --bare` and worktrees are added as siblings. See the **Bare Clone Workflow Example** section below.
  - Rationale: different teams already use different layouts, but Ito should support a finite, testable set.
  - Alternative considered: open-ended custom path templates; rejected to avoid combinatorial complexity and brittle instruction generation.

- For `checkout_siblings` and `bare_control_siblings`, group change worktrees under a single dedicated `ito-worktrees` directory rather than scattering them as direct siblings.
  - `checkout_siblings` uses `<project>-<dir_name>/` next to the checkout directory.
  - `bare_control_siblings` uses `<dir_name>/` inside the bare repo directory.
  - `checkout_subdir` uses `.<dir_name>/` (dot-prefixed) inside the checkout.
  - The directory name is configurable via `worktrees.layout.dir_name`, defaulting to `ito-worktrees`.
  - Rationale: keeps the parent directory clean (one folder instead of N per change), makes Ito-managed worktrees immediately identifiable, and simplifies cleanup (delete one directory). Making the name configurable accommodates teams with existing conventions (e.g., `.worktrees`, `wt`) without requiring a new strategy.
  - Alternative considered: flat sibling worktrees (one directory per change at the same level as the checkout); rejected because it clutters the parent directory and makes it hard to distinguish Ito worktrees from other projects.

- Keep optional `worktrees.layout.base_dir` as a bounded override for deterministic directory placement inside a chosen strategy.
  - Rationale: preserves flexibility for monorepo/local preferences without introducing unlimited topology variants.

- Treat copy patterns as explicit glob-style include patterns resolved relative to repository root, and copy from `./main` into the change worktree.
  - Rationale: matches user expectation for local env files and avoids accidental broad copies.
  - Alternative considered: shell snippets for arbitrary copy logic; rejected for portability and safety.

- Support setup commands as an ordered list of shell commands rendered into apply instructions, but executed by the operator/agent rather than silently auto-run by Ito.
  - Rationale: keeps the system transparent and auditable while still enabling steps like `direnv allow`.
  - Alternative considered: automatic command execution during instruction generation; rejected because generation should remain read-only.

- Add an integration preference policy used by apply instructions:
  - `commit_pr` (default): commit and open PR workflow.
  - `merge_parent`: merge into parent branch workflow.
  - Rationale: the request explicitly needs both outcomes with project-level defaulting.

- Add cleanup guidance as part of apply instructions after integration.
  - Rationale: ensures worktree lifecycle has a clear endpoint and prevents stale workspace accumulation.

- Add interactive worktree setup prompts during `ito init` and `ito update`.
  - `ito init` always runs the worktree setup wizard: asks whether to enable worktrees, which strategy to use (presenting the three supported options with `checkout_subdir` as recommended default), and which integration mode to prefer (`commit_pr` or `merge_parent`).
  - `ito update` runs the same wizard only when `worktrees.strategy` is not yet set in config (first upgrade scenario). If already configured, the prompt is skipped and existing config is preserved.
  - Choices are auto-persisted to the project or global config file immediately after the user answers.
  - After persisting, the CLI prints the config file path and the keys that were written, so the user knows where to adjust settings later.
  - Non-interactive mode (`--no-interactive`) skips the prompts and uses defaults.
  - Rationale: users should be guided through worktree setup at the natural touchpoints (first init, upgrade) rather than having to discover `ito config set` on their own.
  - Alternative considered: inform-only mode that prints instructions but doesn't persist; rejected because it adds friction and users are unlikely to follow up with manual config edits.

## Bare Clone Workflow Example

The `bare_control_siblings` strategy is modelled after the common bare-clone worktree pattern. A reference implementation is the `wtclone` script (`~/.local/bin/wtclone.sh`, based on [this blog post](https://morgan.cugerone.com/blog/workarounds-to-git-worktree-using-bare-repository-and-cannot-fetch-remote-branches/)):

```bash
#!/usr/bin/env bash
# Usage: wtclone [-vh] REPO_URL [DIR_NAME]
# Clone a repository into a bare worktree layout.
set -e

url=$1
basename=${url##*/}
name=${2:-${basename%.*}}

mkdir "$name"
cd "$name"

# Clone as bare repo under .bare, then point .git at it
git clone --bare "$url" .bare
echo "gitdir: ./.bare" > .git

# Allow fetching remote branches (bare repos don't set this by default)
git config remote.origin.fetch "+refs/heads/*:refs/remotes/origin/*"
git fetch origin

# Add worktree for the default branch
main_branch=$(git branch --show-current)
git worktree add "$main_branch"

# Point bare HEAD to a detached ref so the bare dir itself stays clean
git symbolic-ref HEAD refs/heads/bare
```

This produces the layout that `bare_control_siblings` expects:

```
~/Code/myproject/          # bare/control repo
├── .bare/                 # git object store
├── .git                   # gitdir pointer to .bare
├── main/                  # default-branch worktree
└── ito-worktrees/         # Ito-managed change worktrees (configurable via layout.dir_name)
    ├── 012-02_config/
    └── 013-01_feature/
```

Key points for Ito's strategy implementation:
- The bare repo lives at the project root; `main/` is an explicit worktree, not the repo root itself.
- Change worktrees are grouped under `<dir_name>/` (default `ito-worktrees/`) inside the bare repo directory, not as direct siblings.
- The `git symbolic-ref HEAD refs/heads/bare` trick keeps the bare directory from appearing to be on a real branch.

## Risks / Trade-offs

- [Risk] Config surface area expands and can confuse users → Mitigation: provide conservative defaults and clear instruction output.
- [Risk] Command hooks may be unsafe if misconfigured → Mitigation: require explicit configuration and render commands verbatim in visible instructions.
- [Risk] Glob copy patterns may over-match → Mitigation: document path resolution rules and keep default list minimal.
- [Risk] Multiple integration modes may create inconsistent team practice → Mitigation: add a single policy key with project-wide default.
- [Risk] Strategy list may not cover every team workflow → Mitigation: document unsupported patterns explicitly and reject invalid strategy values early.
- [Risk] Interactive prompts during init/update may annoy users who don't want worktrees → Mitigation: "No" is the first option for enablement, and `--no-interactive` skips prompts entirely with safe defaults (disabled).
- [Risk] Users upgrading may not notice the new prompt → Mitigation: update prompt is only shown once (when config is missing), making it a one-time event that's hard to miss.

## Migration Plan

1. Extend config schema/defaults with the `worktrees` object and nested keys, including strict `worktrees.strategy` and optional base directory.
2. Add backward-compatible config loading that recognises the legacy camelCase keys from `012-01`:
   - `worktrees.defaultBranch` → read as `worktrees.default_branch`
   - `worktrees.localFiles` → read as `worktrees.apply.copy_from_main`
   - When a legacy key is encountered, emit a deprecation warning with the recommended new key name.
   - New keys take precedence if both old and new are present.
3. Update config command path handling for nested worktree keys.
4. Update apply instruction renderer to consume layout policy and emit deterministic path/setup sections.
5. Add behavior for missing strategy policy: include a one-time ask-user prompt in apply instructions and provide supported options with a recommended default.
6. Add interactive worktree setup wizard to `ito init` (always runs) and `ito update` (runs when worktree config not yet set).
7. Keep fallback behavior for existing configs that only define legacy keys.
8. Validate with strict change validation and update tests for each supported workflow strategy.

Rollback strategy:

- Revert to previous instruction rendering and keep legacy keys only.
- Ignore unknown nested keys gracefully if partial deployment occurs.

## Open Questions

- Should the one-time strategy selection flow be persisted automatically by Ito, or kept as explicit user config edits?
  - **Resolved**: Auto-persist. `ito init` and `ito update` write choices directly to config and display the file path.
