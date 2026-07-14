<!-- ITO:START -->
## Why

Creating a missing coordination branch from the caller's current `HEAD` can leak code-branch history into the internal coordination branch. The coordination branch should start as an isolated metadata branch with no relationship to implementation history.

## What Changes

- Initialize a missing remote coordination branch from an empty root commit instead of pushing `HEAD`.
- Materialize the empty tree in the repository before creating the root commit, so setup does not depend on pre-existing object database contents.
- Ensure change reservation also initializes and checks out the coordination branch before committing reservation metadata.
- Keep existing behavior when the remote coordination branch already exists.

## Change Shape

- **Type**: fix
- **Risk**: medium
- **Stateful**: yes
- **Public Contract**: cli, config
- **Design Needed**: no
- **Design Reason**: The implementation is limited to the existing coordination-branch setup path and is covered by focused unit tests.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `change-coordination-branch`: missing remote coordination branches must be initialized from an empty root commit rather than from the current code branch.

## Impact

- Affected code: `ito-rs/crates/ito-core/src/git.rs`.
- Affected behavior: first-time setup of `origin/<coordination-branch>` when coordination branch sync or reservation is enabled.
- Affected tests: coordination branch setup unit tests for empty-tree initialization and error handling.
<!-- ITO:END -->
