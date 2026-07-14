# Change: Managed File Ownership

## Why

Ito installs and updates files under `.ito/`, `.opencode/`, `.github/`, `.codex/`, and managed sections of guidance files. Agents often have to infer which edits are durable and which will be overwritten by `ito init` or `ito update`.

Ito should make file ownership and update effects explicit before agents edit generated files.

## What

Add managed-file inspection and dry-run update surfaces:

```bash
ito managed status --json
ito managed diff --json
ito update --dry-run --json
```

Ito should report whether a file is generated, marker-managed, user-owned, or unknown, and where durable project-specific overrides belong.

## Impact

Agents can avoid editing generated files accidentally. Users can preview update effects and understand where to put durable guidance.

## Out Of Scope

This change does not move all templates or redesign the installer. It adds ownership metadata and preview behavior over the existing installation/update system.

## Success Criteria

- `ito managed status --json` reports ownership for known managed files and marker blocks.
- `ito managed diff --json` reports pending managed-block/template updates without applying them.
- `ito update --dry-run --json` previews created, updated, skipped, and overwritten files.
- Agent instructions tell agents to inspect managed status before editing managed paths.
