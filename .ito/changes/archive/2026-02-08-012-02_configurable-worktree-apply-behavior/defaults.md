## Worktree Strategy Defaults

This document defines the codified workflow strategy defaults for `worktrees.strategy`.

### Default Strategy

- Default `worktrees.strategy`: `checkout_subdir`
- Default `worktrees.layout.dir_name`: `ito-worktrees`
- Default `worktrees.default_branch`: `main` (fallback `master` if `main` does not exist)
- Default `worktrees.apply.integration_mode`: `commit_pr`
- Default `worktrees.apply.copy_from_main`: [`.env`, `.envrc`, `.mise.local.toml`]
- Default `worktrees.apply.setup_commands`: `[]`

### Strategy Path Conventions

Assume:

- `<project>` = project directory name (e.g., `myproject`)
- `<base>` = resolved `worktrees.layout.base_dir` or strategy default base
- `<change-id>` = Ito change ID (for example `012-02_configurable-worktree-apply-behavior`)

#### Strategy: `checkout_subdir`

```
<project>/                          # normal git checkout (default branch)
├── .git/
├── src/
└── .ito-worktrees/                 # gitignored subdirectory (configurable via layout.dir_name)
    ├── <change-id>/
    └── ...
```

- main worktree: the checkout itself (no separate `main` directory)
- change worktree path: `<base>/.<dir_name>/<change-id>` (dot-prefixed, where `<dir_name>` defaults to `ito-worktrees`)
- intent: keep change worktrees under a gitignored subdirectory inside the project
- gitignore: requires `.<dir_name>/` entry (Ito adds this automatically)

#### Strategy: `checkout_siblings`

```
~/Code/
├── <project>/                      # normal git checkout (default branch)
│   ├── .git/
│   └── src/
└── <project>-ito-worktrees/        # dedicated sibling folder (configurable via layout.dir_name)
    ├── <change-id>/
    └── ...
```

- main worktree: `<base>/<project>` (the original checkout)
- change worktree path: `<base>/<project>-<dir_name>/<change-id>` (where `<dir_name>` defaults to `ito-worktrees`)
- intent: keep change worktrees in a single dedicated sibling directory next to the checkout
- gitignore: not needed (worktrees are outside the checkout)

#### Strategy: `bare_control_siblings`

```
~/Code/<project>/                   # bare/control repo (no working tree)
├── .git
├── main/                           # main branch worktree
└── ito-worktrees/                  # dedicated subfolder (configurable via layout.dir_name)
    ├── <change-id>/
    └── ...
```

- main worktree path: `<base>/main`
- change worktree path: `<base>/<dir_name>/<change-id>` (where `<dir_name>` defaults to `ito-worktrees`)
- intent: bare/control workspace with `main` as a worktree and change worktrees grouped in a dedicated subfolder
- gitignore: not needed (bare repo has no working tree to pollute)

### Unsupported Topologies

Any topology outside the codified strategy enum is unsupported and SHALL be rejected at config validation time.
