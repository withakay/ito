## Key points

- `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and
  `.ito/audit` are tracked canonical inputs on `main`.
- Never ignore or untrack those paths to silence release-plz dirty checks.
- The historical projected-symlink remediation is retired for this repository.
- `docs/ito` has no generation or publication step.
- `allow_dirty = false` and `publish_allow_dirty = false` remain enforced.
- Cargo-dist packages `ito-cli` with `default-features = false` and only the
  `web` feature; backend and coordination-branch remain experimental.
- Workspace changelog/dependency updates remain enabled, and only `ito-cli`
  creates Git tags.

## Operational rule

Resolve release-tree dirtiness at its source. Do not use `.gitignore` or
`git rm --cached` to conceal reviewed Ito authority, and do not tag, push, or
publish from a migration verification worktree.
