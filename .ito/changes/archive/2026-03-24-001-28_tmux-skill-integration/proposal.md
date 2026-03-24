<!-- ITO:START -->
## Why

The global OpenCode `tmux` skill provides valuable agent guidance for controlling tmux sessions programmatically, but it lives outside Ito's managed skill set and is not distributed with `ito init` / `ito update`. Integrating it as an Ito-managed embedded skill ensures agents working in any Ito-initialized project have access to tmux capabilities — which is a prerequisite for the `proposal-viewer-command` change (001-29) and any future Ito workflow steps that open interactive terminal panes.

## What Changes

- Embed the `tmux` skill (SKILL.md + helper scripts) into `ito-rs/crates/ito-templates/assets/skills/tmux/` so it is installed by `ito init` and updated by `ito update` alongside other Ito skills.
- Adapt the SKILL.md frontmatter and description to reflect its role as an Ito skill dependency (referencing upstream attribution).
- Update `ito-rs/crates/ito-templates/AGENTS.md` guidance to document the new skill and the pattern for including script assets alongside SKILL.md.

## Capabilities

### New Capabilities

- `ito-tmux-skill`: An Ito-managed skill that provides agents with instructions and helper scripts for controlling tmux sessions programmatically — opening popover panes, sending keystrokes, polling for output, and cleaning up sessions. Sourced from the upstream OpenCode tmux skill; adapted for Ito distribution.

### Modified Capabilities

- `cli-skills`: The skill installation surface (`ito init` / `ito update`) will now include the `tmux` skill directory with both SKILL.md and a `scripts/` subdirectory, establishing the pattern for skills with bundled asset files.

## Impact

- `ito-rs/crates/ito-templates/assets/skills/tmux/` — new directory with SKILL.md and scripts/
- `ito-rs/crates/ito-templates/src/lib.rs` — `include_dir!` already embeds the assets tree; no code change required if the directory is added under the correct path
- `ito-rs/crates/ito-templates/AGENTS.md` — updated guidance
- Downstream: agents in any Ito project will gain the `tmux` skill after `ito init` / `ito update`
<!-- ITO:END -->
