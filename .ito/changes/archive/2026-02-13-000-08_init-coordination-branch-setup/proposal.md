## Why

Users currently discover coordination-branch readiness only when `ito create change` attempts to reserve metadata. That delays feedback and makes first-run setup feel fragile when `origin` is missing or lacks push permissions.

## What Changes

- Add an opt-in `ito init` flag to proactively set up the configured coordination branch on `origin`.
- Reuse existing coordination branch config (`changes.coordination_branch.name`) so setup follows project defaults and overrides.
- Provide clear success/failure output so users know whether branch provisioning succeeded, was already ready, or needs remote/auth fixes.
- Keep default `ito init` behavior unchanged unless the new flag is passed.

## Capabilities

### New Capabilities

- `init-coordination-setup`: Opt-in coordination branch provisioning during `ito init`.

### Modified Capabilities

- `cli-init`: Extend init command behavior with an optional branch-setup flow.
- `change-coordination-branch`: Add explicit provisioning behavior for the coordination branch before first change creation.

## Impact

- **CLI surface**: Adds one `ito init` option and associated output.
- **Git integration**: Adds a setup path that checks and creates the coordination branch on `origin` when needed.
- **Testing**: Requires new init integration tests for branch-exists, branch-created, and remote failure scenarios.
