## ADDED Requirements

### Requirement: Worktree config available at template render time

The installer SHALL have access to the resolved worktree configuration (enabled, strategy, layout, integration mode) before rendering project templates and skills.

#### Scenario: Config available during init

- **WHEN** `ito init` runs interactively and the worktree wizard completes
- **THEN** the resolved worktree configuration SHALL be passed to `install_default_templates()` before any templates are written

#### Scenario: Config available during update

- **WHEN** `ito update` runs and worktree configuration exists in the global config
- **THEN** the resolved worktree configuration SHALL be loaded and passed to the template installer

#### Scenario: No worktree config available

- **WHEN** `ito init` runs non-interactively without prior worktree configuration
- **THEN** the installer SHALL render templates with worktree disabled (default state)

### Requirement: AGENTS.md rendered with worktree context

The root `AGENTS.md` project template SHALL be rendered via Jinja2 with worktree configuration context, producing strategy-specific instructions inside the managed block.

#### Scenario: Worktrees enabled with checkout_subdir strategy

- **WHEN** the worktree config has `enabled=true` and `strategy=checkout_subdir`
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section specifying: the strategy name, the directory name (e.g., `ito-worktrees`), and the exact path pattern `.{{ layout_dir_name }}/` for creating worktrees
- **AND** SHALL include the exact `git worktree add` command for this strategy
- **AND** SHALL instruct agents not to ask the user where to create worktrees

#### Scenario: Worktrees enabled with checkout_siblings strategy

- **WHEN** the worktree config has `enabled=true` and `strategy=checkout_siblings`
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section specifying: the strategy name, the sibling directory pattern `../<project>-{{ layout_dir_name }}/`, and the exact `git worktree add` command for this strategy

#### Scenario: Worktrees enabled with bare_control_siblings strategy

- **WHEN** the worktree config has `enabled=true` and `strategy=bare_control_siblings`
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section specifying: the strategy name, the bare repo layout, and the exact `git worktree add` command for this strategy

#### Scenario: Worktrees disabled

- **WHEN** the worktree config has `enabled=false` or no worktree config exists
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section stating that worktrees are not configured and agents SHALL NOT create git worktrees unless the user explicitly requests it

#### Scenario: Managed block update preserves user content

- **WHEN** `ito update` re-renders AGENTS.md with updated worktree config
- **THEN** only the content inside the `<!-- ITO:START -->` / `<!-- ITO:END -->` markers SHALL be replaced
- **AND** user-authored content outside the markers SHALL be preserved

### Requirement: Skills rendered with worktree context

Skill templates that contain Jinja2 template syntax SHALL be rendered with worktree configuration context during installation, rather than copied verbatim.

#### Scenario: Skill with Jinja2 syntax is rendered

- **WHEN** a skill template file contains `{%` or `{{` Jinja2 syntax
- **THEN** the installer SHALL render it via Jinja2 with the worktree config context before writing to the destination

#### Scenario: Skill without Jinja2 syntax is copied verbatim

- **WHEN** a skill template file does not contain Jinja2 syntax
- **THEN** the installer SHALL copy it verbatim (preserving current behavior)

#### Scenario: Rendered skill reflects configured strategy

- **WHEN** the worktree skill is installed with `strategy=checkout_subdir` and `layout_dir_name=ito-worktrees`
- **THEN** the installed skill SHALL contain the exact directory path, exact `git worktree add` command, and exact cleanup steps for `checkout_subdir` -- not a fallback discovery chain

### Requirement: Worktree skill eliminates discovery heuristics

The installed `using-git-worktrees` skill SHALL provide precise, config-driven instructions and SHALL NOT include vague directory-discovery fallback logic.

#### Scenario: No directory guessing when config exists

- **WHEN** the worktree skill is rendered with a valid worktree config
- **THEN** the skill SHALL NOT contain instructions to "check existing directories", "grep AGENTS.md", or "ask the user" for worktree location
- **AND** SHALL instead provide the exact directory and commands based on config

#### Scenario: Disabled config produces clear guidance

- **WHEN** the worktree skill is rendered with `enabled=false`
- **THEN** the skill SHALL instruct agents to work in the current checkout and not create worktrees
