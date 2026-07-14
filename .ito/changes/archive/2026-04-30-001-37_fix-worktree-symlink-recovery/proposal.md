<!-- ITO:START -->
## Why

New change worktrees in this repository are not consistently Ito-ready after creation. In a direct reproduction, both a raw `git worktree add ...` proposal worktree and a worktree created via `ito worktree ensure --change <id>` were missing the expected `.ito` coordination symlinks (`changes`, `specs`, `modules`, `workflows`, `audit`). That left the worktree unable to see shared Ito state and caused `ito create change` to fail first with a module-lookup error and then, after manual recovery, with a generic `I/O error: No such file or directory`.

We now know that `ito init --update --tools none` repairs the missing symlinks in an existing worktree, but that recovery path is accidental from the agent's perspective: `ito worktree ensure` is documented and presented as the one-step way to create and initialize a change worktree, yet it did not wire the coordination links in the reproduced session. Ito should either create those links automatically during worktree creation or detect and repair them before commands like `ito create change` proceed.

## What Changes

- Make `ito worktree ensure` produce a fully wired Ito worktree when coordination storage mode is `worktree`, including the `.ito` coordination symlinks.
- Add an explicit recovery path for existing worktrees with missing or stale coordination links so `ito init --update` or an equivalent flow is contractually supported rather than incidental.
- Improve `ito create change` so it detects missing coordination wiring and either repairs it automatically or fails with a concrete, actionable message instead of a generic I/O error.
- Update worktree guidance and tests so agents can rely on `ito worktree ensure` as the default path and can recover deterministically when a worktree was created by raw `git worktree add`.

## Change Shape

- **Type**: fix
- **Risk**: medium
- **Stateful**: yes
- **Public Contract**: cli, config
- **Design Needed**: yes
- **Design Reason**: The fix crosses coordination wiring, worktree initialization, change creation, and user-facing recovery guidance. The proposal needs a clear contract for automatic wiring versus explicit repair.

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `worktree-lifecycle`: `ito worktree ensure` must create a fully wired worktree, not just a Git worktree plus include/setup initialization.
- `coordination-worktree`: worktree-local `.ito` coordination links need an explicit repair contract for missing or stale wiring.
- `change-creation`: `ito create change` must detect or recover from missing coordination wiring and emit actionable errors.
- `cli-init`: `ito init --update` on an existing worktree should explicitly repair coordination symlinks when coordination storage mode is `worktree`.

## Impact

- Core worktree and coordination code in `ito-rs/crates/ito-core/src/worktree_ensure.rs`, `ito-rs/crates/ito-core/src/worktree_init.rs`, `ito-rs/crates/ito-core/src/coordination.rs`, and create-change paths under `ito-rs/crates/ito-core/src/create/`.
- CLI flows and tests around worktree creation, init/update, and change creation.
- Agent-facing worktree guidance in instruction templates under `ito-rs/crates/ito-templates/assets/instructions/agent/`.
- Regression coverage should reproduce the exact observed sequence recorded in `issues.md`.
<!-- ITO:END -->
