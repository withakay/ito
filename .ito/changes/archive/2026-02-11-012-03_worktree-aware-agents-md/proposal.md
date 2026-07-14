## Why

`ito init` asks the user their worktree preferences (strategy, directory name, integration mode) and persists them to config, but this information never reaches the files agents actually read during development -- AGENTS.md and the worktree skill. Agents encounter vague heuristics ("check existing dirs, grep AGENTS.md, ask user") instead of precise, actionable instructions derived from the user's explicit choices. This wastes tokens on discovery logic and risks agents creating worktrees in the wrong location or prompting the user for information they already provided.

## What Changes

- Reorder `ito init` so the worktree wizard runs **before** `install_default_templates()`, making config available at template render time
- Convert the root `AGENTS.md` project template from a static file to a Jinja2-rendered template, injecting a concrete "Worktree Workflow" section into the managed block based on the user's worktree config
- Add Jinja2 rendering support for skill templates in the installer pipeline (currently skills are verbatim copies with no variable substitution)
- Rewrite the `using-git-worktrees` skill template to use Jinja2 conditionals that render exact directories, exact `git worktree add` commands, and exact cleanup steps -- eliminating the vague directory-discovery fallback chain
- When worktrees are disabled, inject an explicit "Worktrees are not configured" section so agents know not to create them

## Capabilities

### New Capabilities

- `worktree-aware-template-rendering`: Jinja2 rendering pipeline for project templates (AGENTS.md) and skill templates, parameterized by the user's worktree configuration. Covers: loading worktree config before template installation, rendering templates with worktree context, and handling both enabled and disabled states.

### Modified Capabilities

- `ito-init`: Reorder init flow so worktree wizard runs before template installation; pass resolved worktree config into the installer
- `distribution`: Extend skill installation path to support Jinja2 rendering (currently verbatim byte copy only)

## Impact

- **`ito-cli/src/app/init.rs`**: Reorder worktree wizard to run before `install_default_templates()`; expand `WorktreeWizardResult` to carry config values; pass config to installer
- **`ito-core/src/installers/mod.rs`**: Accept optional worktree config in `install_default_templates()`; render AGENTS.md via Jinja2 when config is available; add rendering pass for skills
- **`ito-templates/src/lib.rs`**: Add rendering function for project templates and skills with worktree context
- **`ito-templates/assets/default/project/AGENTS.md`**: Convert to Jinja2 template with conditional worktree section inside managed block
- **`ito-templates/assets/skills/using-git-worktrees/SKILL.md`**: Rewrite with Jinja2 conditionals replacing the vague directory-discovery logic
- **`ito-cli/src/app/update.rs`**: Ensure `ito update` also passes worktree config when refreshing managed blocks
- **Existing tests**: `init_more.rs` tests for AGENTS.md marker handling will need updating to account for rendered content
