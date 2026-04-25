<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: AGENTS.md rendered with worktree context

The root `AGENTS.md` project template SHALL be rendered via Jinja2 with worktree configuration context, producing strategy-specific instructions inside the managed block.

Because `AGENTS.md` is a file written to disk and expected to be committed, it MUST remain portable:

- The managed block MUST NOT embed machine-specific absolute paths.
- The managed block SHOULD use repo-relative paths and clearly state assumptions (for example, that commands are run from the repo/worktree root).
- When worktrees are enabled, the managed block SHALL instruct agents to use Worktrunk (`wt`) for worktree creation and switching instead of raw `git worktree add` commands.
- When Ito's default `ito-worktrees` layout is configured, the managed block SHALL document the local Worktrunk path configuration needed to keep Worktrunk-created worktrees in that layout.

- **Requirement ID**: `worktree-aware-template-rendering:agents-md-rendered-with-worktree-context`

#### Scenario: Worktrees enabled with checkout_subdir strategy

- **WHEN** the worktree config has `enabled=true` and `strategy=checkout_subdir`
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section specifying: the strategy name, the directory name (e.g., `ito-worktrees`), and the repo-relative path pattern `.{{ layout_dir_name }}/` for creating worktrees
- **AND** SHALL include the Worktrunk command for creating or switching to the change branch with the configured base branch
- **AND** SHALL instruct agents not to ask the user where to create worktrees

#### Scenario: Worktrees enabled with checkout_siblings strategy

- **WHEN** the worktree config has `enabled=true` and `strategy=checkout_siblings`
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section specifying: the strategy name, the repo-relative sibling directory pattern `../<project>-{{ layout_dir_name }}/`, and the Worktrunk command for creating or switching to the change branch with the configured base branch

#### Scenario: Worktrees enabled with bare_control_siblings strategy

- **WHEN** the worktree config has `enabled=true` and `strategy=bare_control_siblings`
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section specifying: the strategy name, the bare repo layout with repo-relative paths, the `../{{ layout_dir_name }}/<full-change-id>` path convention, and the Worktrunk command for creating or switching to the change branch from the configured base branch

#### Scenario: Local Worktrunk path configuration documented

- **WHEN** worktrees are enabled and the rendered worktree root uses `ito-worktrees`
- **THEN** the rendered AGENTS.md managed block SHALL include a `.config/wt.toml` or operation-local Worktrunk configuration example that maps `worktree-path` to the Ito worktree root using `{{ branch | sanitize }}`

#### Scenario: Worktrees disabled

- **WHEN** the worktree config has `enabled=false` or no worktree config exists
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section stating that worktrees are not configured and agents SHALL NOT create git worktrees unless the user explicitly requests it

#### Scenario: Managed block update preserves user content

- **WHEN** `ito update` re-renders AGENTS.md with updated worktree config
- **THEN** only the content inside the `<!-- ITO:START -->` / `<!-- ITO:END -->` markers SHALL be replaced
- **AND** user-authored content outside the markers SHALL be preserved
<!-- ITO:END -->
