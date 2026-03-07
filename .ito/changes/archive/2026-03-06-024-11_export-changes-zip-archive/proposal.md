## Why

Teams need a portable, verifiable snapshot of change history for backup, handoff, and migration workflows, especially before destructive local cleanup during backend cutover. A canonical zip export gives one deterministic artifact that can be stored, reviewed, and transferred safely.

## What Changes

- Add a backend export command that packages active and archived changes into a single zip archive.
- Define a canonical archive layout and manifest so exports are consistent across environments.
- Include integrity metadata so exported bundles can be validated before use.
- Add clear CLI output for destination path, exported counts, and validation summary.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `backend-change-sync`: Add canonical zip export behavior for change artifacts and lifecycle metadata.
- `change-repository`: Ensure export can enumerate complete active and archived change sets from backend-backed state.

## Impact

- **Affected workflows**: backend operations gain a standard backup/export step via `ito backend export`.
- **Affected code**: `ito-cli` backend command surface, `ito-core` export orchestration, archive packaging utilities.
- **Data portability**: exported change history becomes transferable as a single artifact.
- **Operational safety**: teams can preserve a validated snapshot before migration or cleanup operations.
