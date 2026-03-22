## Why

Ito still resolves many change reads directly from `.ito/changes/`, which breaks the repository abstraction work already started and makes remote mode behave like a partial overlay instead of a real persistence mode. We need a single `ChangeRepository` model that can serve active and archived changes from either the filesystem or a remote backend without command handlers knowing which implementation they are using.

## What Changes

- Extend `ChangeRepository` semantics so one repository can enumerate and resolve both active and archived changes using lifecycle-aware queries instead of separate local directory assumptions.
- Add a real remote-backed `ChangeRepository` implementation for client use, with REST as the initial transport behind the abstraction.
- Wire change-reading command paths through `ChangeRepository` instead of direct `FsChangeRepository` construction.
- Ensure remote mode ignores stray local active-change markdown and treats repository output as the canonical source for change reads.

## Impact

- Affected specs: `change-repository`, `cli-list`, `cli-show`
- Affected code: `ito-domain` change repository contracts, `ito-core` change repository adapters, `ito-cli` read-oriented command handlers
- Behavioral change: in remote mode, change reads come from the selected repository implementation rather than local `.ito/changes/`

## Execution Guidance

- Start after `025-04_add-repository-runtime-factory` has settled the repository bundle/factory shape.
- This change can run in parallel with `025-02_wire-task-repository-backends` and `025-03_wire-module-repository-backends`.
- `025-05_mirror-specs-and-archives-to-backend` should wait for this change to settle the lifecycle-aware `ChangeRepository` direction.
