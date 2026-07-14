## Why

`ito` can create and update local session state at `.ito/session.json`, which is useful during development but should not be committed. Without a default ignore rule, this file shows up as untracked noise and can be accidentally included in commits.

## What Changes

- `ito init` updates the repository root `.gitignore` to ignore `.ito/session.json` (creating `.gitignore` if it does not exist).
- The update is idempotent and preserves any existing `.gitignore` content.

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `cli-init`: Ensure `ito init` adds an ignore rule for `.ito/session.json`.

## Impact

- Affects init-time file generation behavior (writes/updates `.gitignore` in the repo root).
- Requires Rust implementation changes in the init/install path and tests to ensure idempotent behavior.
