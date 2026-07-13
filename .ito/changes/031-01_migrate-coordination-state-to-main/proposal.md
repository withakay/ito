<!-- ITO:START -->
## Why

Ito currently allows authoritative change state to live in a separate coordination worktree and exposes that state through absolute `.ito/*` symlinks. In practice the branch can diverge, pushes can fail after local allocation, and proposal state can remain invisible from `main`. Before Ito can make tracked main-branch artifacts authoritative, existing repositories need a lossless and reviewable path out of legacy coordination storage.

This change adds a compatibility bridge rather than performing migration automatically. Ito will recognize legacy coordination state, keep read-only inspection available with a warning, block mutations that could deepen divergence, and emit an agent instruction that prepares a validated migration proposal without deleting the old store.

## What Changes

- Add a centralized legacy-coordination detector covering worktree storage configuration, the managed coordination symlinks, and legacy coordination `.gitignore` markers.
- Classify CLI operations as read-only or mutating when legacy storage is detected. Reads continue with a migration warning; writes fail before mutation and identify the remediation instruction.
- Add `ito agent instruction migrate-to-main` to the standard instruction bundle, independent of whether coordination-branch runtime code is compiled.
- Make the emitted prompt guide an agent through snapshotting and hashing both stores, copying coordinated state into real tracked `.ito/{changes,specs,modules,workflows,audit}` directories, disabling worktree coordination, validating parity, and preparing a migration PR.
- Leave the external coordination worktree and branch untouched so rollback and forensic comparison remain possible.
- Do not add an automatic or dedicated imperative migration command.

## Change Shape
- **Type**: migration
- **Risk**: high
- **Stateful**: yes
- **Public Contract**: cli, config
- **Design Needed**: yes
- **Design Reason**: The detector, read/write policy, safety invariants, and reversible agent-driven migration sequence cross CLI, configuration, filesystem, Git, and template boundaries.

## Capabilities
### New Capabilities
- `coordination-main-migration`: Detect legacy coordination storage and provide a safe, agent-driven migration path that makes tracked main-branch Ito artifacts authoritative.

### Modified Capabilities
- `coordination-worktree-migration`: Extend the migration contract with the reverse transition from coordination worktree storage to tracked main storage while preserving the old store.
- `agent-instructions`: Expose the `migrate-to-main` remediation instruction from the standard binary and identify it consistently in warnings and errors.

## Impact

- Configuration and detection: `ito-config` change/coordination DTOs and new `ito-core` legacy-state inspection.
- CLI policy: command dispatch/runtime intent classification, diagnostic wording, and mutation preflight behavior.
- Templates: embedded agent instruction assets, CLI help, managed harness distribution, and rendering tests.
- Filesystem/Git safety: symlink inspection, content snapshots and hashes, real-directory materialization, validation, and PR-oriented integration guidance.
- Tests: detector matrices, read-warning and write-block integration tests, prompt rendering tests, and legacy-state fixtures.
<!-- ITO:END -->
