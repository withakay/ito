## ADDED Requirements

### Requirement: XDG data path resolution for coordination worktree

The configuration system SHALL resolve the central worktree storage path using XDG conventions when no explicit `worktree_path` is configured.

- **Requirement ID**: cascading-config:xdg-worktree-path

#### Scenario: XDG_DATA_HOME is set

- **WHEN** `$XDG_DATA_HOME` is set to `/custom/data`
- **AND** the project is `withakay/ito`
- **THEN** the resolved worktree path is `/custom/data/ito/withakay/ito/`

#### Scenario: XDG_DATA_HOME is not set

- **WHEN** `$XDG_DATA_HOME` is not set
- **AND** the project is `withakay/ito`
- **THEN** the resolved worktree path is `~/.local/share/ito/withakay/ito/`

#### Scenario: Org and repo derived from git remote

- **WHEN** no `backend.project.org` or `backend.project.repo` is configured
- **AND** the git remote `origin` URL is `git@github.com:withakay/ito.git`
- **THEN** `<org>` resolves to `withakay` and `<repo>` resolves to `ito`

#### Scenario: Org and repo from backend config take precedence

- **WHEN** `backend.project.org` is `myorg` and `backend.project.repo` is `myrepo`
- **THEN** the resolved worktree path uses `myorg/myrepo` regardless of git remote
