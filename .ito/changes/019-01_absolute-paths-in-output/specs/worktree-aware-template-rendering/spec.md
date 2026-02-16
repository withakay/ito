<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: AGENTS.md rendered with worktree context

The root `AGENTS.md` project template SHALL be rendered via Jinja2 with worktree configuration context, producing strategy-specific instructions inside the managed block.

Because `AGENTS.md` is a file written to disk and expected to be committed, it MUST remain portable:

- The managed block MUST NOT embed machine-specific absolute paths.
- The managed block SHOULD use repo-relative paths and clearly state assumptions (for example, that commands are run from the repo/worktree root).

#### Scenario: Worktrees enabled with checkout_subdir strategy

- **WHEN** the worktree config has `enabled=true` and `strategy=checkout_subdir`
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section specifying: the strategy name, the directory name (e.g., `ito-worktrees`), and the repo-relative path pattern `.{{ layout_dir_name }}/` for creating worktrees
- **AND** SHALL include the exact `git worktree add` command for this strategy using the repo-relative paths
- **AND** SHALL instruct agents not to ask the user where to create worktrees

#### Scenario: Worktrees enabled with checkout_siblings strategy

- **WHEN** the worktree config has `enabled=true` and `strategy=checkout_siblings`
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section specifying: the strategy name, the repo-relative sibling directory pattern `../<project>-{{ layout_dir_name }}/`, and the exact `git worktree add` command for this strategy using repo-relative paths

#### Scenario: Worktrees enabled with bare_control_siblings strategy

- **WHEN** the worktree config has `enabled=true` and `strategy=bare_control_siblings`
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section specifying: the strategy name, the bare repo layout with repo-relative paths, and the exact `git worktree add` command for this strategy using repo-relative paths

#### Scenario: Worktrees disabled

- **WHEN** the worktree config has `enabled=false` or no worktree config exists
- **THEN** the rendered AGENTS.md managed block SHALL contain a "Worktree Workflow" section stating that worktrees are not configured and agents SHALL NOT create git worktrees unless the user explicitly requests it

#### Scenario: Managed block update preserves user content

- **WHEN** `ito update` re-renders AGENTS.md with updated worktree config
- **THEN** only the content inside the `<!-- ITO:START -->` / `<!-- ITO:END -->` markers SHALL be replaced
- **AND** user-authored content outside the markers SHALL be preserved
<!-- ITO:END -->
