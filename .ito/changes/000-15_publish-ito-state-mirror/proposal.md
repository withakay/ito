<!-- ITO:START -->
## Why

Active Ito change/spec state is currently invisible in plain GitHub views and non-Ito checkouts because worktree-backed coordination relies on `.ito/...` symlinks into a separate coordination worktree. We need a committed, read-only published mirror on `main` so agents and humans can inspect in-flight Ito state without knowing anything about the coordination branch wiring.

## What Changes

- Add a published Ito mirror capability that emits a read-only snapshot of active changes, archived changes, and canonical specs into a committed path on `main`.
- Use `docs/ito` as the default published path while allowing projects to override the destination in Ito config.
- Define drift rules so the coordination branch remains the only writable source of truth and direct edits to the published mirror are treated as generated-output drift.
- Define a publication workflow that updates the committed mirror on `main` without requiring plain consumers to follow coordination-worktree symlinks.

## Change Shape

- **State model**: keep one writable source of truth (coordination state), publish a second generated/read-only surface.
- **Integration model**: publish mirror content onto `main` as committed files so GitHub and plain checkouts can read it.
- **Risk focus**: avoid introducing ambiguous dual-authoring between `.ito/...` coordination state and the published mirror path.

## Capabilities

### New Capabilities

- `published-ito-mirror`: Published, read-only mirror of Ito state for plain GitHub views and non-Ito checkouts.

### Modified Capabilities

- `ito-config-crate`: Add configuration for the published mirror path, defaulting to `docs/ito`.

## Impact

- Affected code: `ito-config`, `ito-core`, and likely CLI/instruction surfaces that manage publication and drift detection.
- Affected systems: coordination branch workflow, archive/publication lifecycle, and plain repository browsing on GitHub.
- Affected artifacts: committed mirror files under `docs/ito` by default, plus configuration/schema updates describing the override path.
<!-- ITO:END -->
