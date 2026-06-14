# Change: Archive And Change Discovery

## Why

Agents repeatedly need to answer simple questions such as which changes are archived, which changes are complete, and where a change moved after archival. Session mining found failures like `ito list` returning `Change not found: archive` and agents manually globbing `.ito/changes/archive` to find archived changes.

Ito should make active and archived change discovery first-class and consistent.

## What

Add archive and discovery surfaces:

```bash
ito archive list --json
ito archive show <change-id> --json
ito list --archived --json
ito list --all --json
```

Unify change resolution so commands can intentionally search active changes, archived changes, or both.

## Impact

Agents stop reverse-engineering archive paths. Users get a clearer model for active versus archived changes. Future archive automation can depend on a stable discovery API.

## Out Of Scope

This change does not redesign the archive directory layout. It adds discoverability and resolver consistency over the existing layout.

## Success Criteria

- Archived changes can be listed and shown without direct filesystem globbing.
- `ito list --archived --json` and `ito list --all --json` return machine-readable change summaries.
- Resolver errors distinguish active-only misses from archived-only matches.
- Tests cover archived lookup by full ID, partial ID, date-prefixed archive directory, and ambiguous matches.
