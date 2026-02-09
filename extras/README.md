# Extras

This directory contains optional tooling that is not part of Ito's core workflow.

## Git Worktree Helpers

The scripts in `extras/scripts/git-worktrees/` are designed to be installed as Git
external subcommands.

If you install the executables `git-wtclone`, `git-wtadd`, etc onto your `PATH`,
you can run them as:

```bash
git wtclone ...
git wtadd ...
git wtlist
git wtremove ...
git wtlock ...
```

### Install

Default install target is `~/.local/bin`:

```bash
./extras/install.sh git-worktrees
```

Custom install prefix:

```bash
./extras/install.sh git-worktrees --prefix /some/bin
```

Optional convenience aliases (written to your global git config):

```bash
./extras/install.sh git-worktrees --aliases
```

This installs:

- `git wta` -> `git wtadd`
- `git wtls` -> `git wtlist`
- `git wtrm` -> `git wtremove`

### Version

The bundled version lives at `extras/scripts/git-worktrees/VERSION`.
Each command supports `--version`.
