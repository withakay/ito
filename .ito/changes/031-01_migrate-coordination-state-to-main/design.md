<!-- ITO:START -->
## Context

Coordination worktree storage currently moves `.ito/changes`, `specs`, `modules`, `workflows`, and `audit` into an external checkout and replaces the repository paths with absolute symlinks. Configuration defaults to worktree coordination, setup adds a managed `.gitignore` block, and many commands attempt implicit synchronization. A failed non-fast-forward push can leave a valid local allocation and a different remote reservation commit, so neither a config-only check nor a symlink-only check is sufficient to decide that a repository is safe for main-authoritative writes.

The standard build will later compile coordination runtime code out, but it must retain enough configuration and filesystem awareness to recognize this legacy state and guide recovery. Migration is deliberately performed by an agent prompt because repositories can contain conflicting or partially migrated state that requires contextual review.

## Goals / Non-Goals

**Goals:**

- Detect configured, residual, broken, and ambiguous coordination state without mutation.
- Apply a single, testable read-warning/write-blocking policy before command side effects.
- Keep one recovery path available in every standard binary: `ito agent instruction migrate-to-main`.
- Emit repository-specific, reversible steps that establish real tracked Ito directories and prove parity.
- Preserve the old coordination worktree and branch throughout preparation and review.

**Non-Goals:**

- Automatically move or delete project state.
- Resolve conflicting source and destination content without human review.
- Remove coordination runtime implementation; feature gating belongs to `031-03_gate-experimental-backend-coordination`.
- Enforce proposal integration on main; implementation readiness belongs to `031-02_enforce-main-first-implementation`.
- Perform Ito's own migration; that rollout belongs to `031-06_migrate-ito-authority-and-release`.

## Approach

Add a small, unconditionally compiled core module for legacy coordination inspection. It consumes resolved configuration plus repository and Ito roots, and returns a structured report rather than a boolean. The report records configuration evidence, per-path filesystem kind and link target, managed gitignore evidence, and a classification of `main_compatible`, `legacy`, or `ambiguous`.

The detector uses `symlink_metadata` so broken links are visible. It does not require coordination feature code and never repairs paths. A real non-empty directory alongside any legacy link is ambiguous; a disabled or embedded configuration with five real directories and no marker is main-compatible.

The CLI classifies parsed commands through an exhaustive `CommandIntent` function before dispatch. Read-only commands emit one warning when the detector reports legacy or ambiguous state. Mutating commands return a typed error before repository construction, sync, filesystem writes, task mutation, worktree creation, or network activity. Unknown/new commands default to mutating until classified. The migration instruction and the minimum read-only diagnostic/configuration surface remain callable.

Add a `migrate-to-main.md.j2` instruction asset and dispatch arm. Rendering gathers only context that is safe without coordination runtime code: project root, Ito root, configured branch and storage, expected managed paths, observed evidence, and the configured main integration preference when available. The template directs the agent to create a dedicated migration branch, capture Git identities and inventories, compare content hashes, replace links with copied real directories, update config, validate, and prepare review. It never tells the agent to remove the external store.

## Contracts / Interfaces

- CLI: `ito agent instruction migrate-to-main [--json]`.
- Diagnostic remediation string: exact command `ito agent instruction migrate-to-main`.
- Core inspection API, conceptually:
  - `inspect_legacy_coordination(project_root, ito_root, config) -> LegacyCoordinationReport`
  - `LegacyCoordinationClass::{MainCompatible, Legacy, Ambiguous}`
  - evidence entries for configuration, each managed path, and gitignore marker.
- CLI policy API, conceptually:
  - `command_intent(&Commands) -> CommandIntent`
  - `CommandIntent::{ReadOnly, Mutating, Recovery}`.
- Instruction JSON output keeps the existing instruction envelope and uses `migrate-to-main` as its artifact identifier.

## Data / State

| Evidence | Main-compatible | Legacy | Ambiguous |
| --- | --- | --- | --- |
| Config | embedded/disabled | worktree/enabled | contradicts filesystem evidence |
| Managed paths | real directories | expected or broken coordination links | mixed links and non-empty real duplicates, or wrong targets |
| Gitignore marker | absent | present | present after apparent partial materialization |

The migration prompt records:

- source coordination worktree path, branch, and commit OID;
- source and destination relative-path inventories;
- deterministic content hashes for regular files and explicit link metadata;
- pre-migration configuration;
- validation results after materialization.

No migration-complete marker is introduced. Completion is proven by main-compatible detector output, matching inventory/hashes, Ito validation, Git review, and eventual integration to main.

## Decisions

- **Detect evidence, not only config.** Legacy upgrades can leave config, links, and markers out of sync.
- **Keep inspection outside the coordination feature.** A standard build must diagnose and escape legacy state even though it cannot operate the old synchronization subsystem.
- **Warn on reads and fail closed on writes.** Inspection remains possible, while further divergence is prevented.
- **Default new command kinds to mutating.** Adding a new command cannot accidentally bypass the guard.
- **Use an emitted prompt instead of a migration command.** Content conflicts and Git policy require contextual decisions and review.
- **Never delete the source.** Rollback evidence is more valuable than automatic cleanup; later manual cleanup is explicitly out of scope.
- **Require reviewed integration.** The migration is prepared on a branch and follows the repository's configured proposal integration workflow.

## Risks / Trade-offs

- A broad write guard can temporarily block useful maintenance commands. The classifier therefore has explicit recovery/read categories and diagnostic tests for every top-level command.
- Filesystem evidence differs across platforms. Tests use platform-aware symlink fixtures and preserve broken-link coverage; Windows junction behavior is tested where supported.
- Hashing large state can be expensive, but hashing is performed by the migration agent, not on every detector invocation.
- The prompt relies on agent compliance. It mitigates this with ordered stop conditions, exact inventories, validation commands, and a required reviewable diff.
- Keeping the external store leaves cleanup work, but avoids irreversible loss and permits independent verification.

## Verification Strategy

- Core unit tests cover every config/filesystem/marker combination, correct and wrong link targets, broken links, real empty and non-empty directories, and ambiguity.
- CLI unit tests exhaustively classify top-level commands and fail when a new command lacks an intentional classification.
- CLI integration tests prove reads warn once and succeed, writes fail before observable state changes, and diagnostics contain the exact remediation command.
- Template tests prove the instruction is embedded and rendered with project-specific evidence in default/no-coordination builds.
- Distribution tests prove all supported harnesses can invoke the instruction without installing a separate migration skill.
- A fixture migration demo compares source and destination inventories/hashes, validates the materialized project, and confirms the source worktree remains unchanged.

## Migration / Rollback

Rollout first adds the detector and prompt while coordination code still exists. Existing legacy projects receive warnings and mutation blocks only after the new binary is installed. They invoke the instruction, prepare the migration on a dedicated branch, and merge it only after parity review. Removing or disabling coordination runtime in the default build happens later.

Before migration, the agent records the source commit and hashes. If preparation fails, it discards or repairs only the migration branch; the external store remains authoritative and untouched. If a merged migration must be reverted, restore the prior config and tracked symlink entries from Git and point them at the retained coordination worktree.

## Open Questions

None. The approved policy is read warning, write blocking, prompt-driven reviewed migration, and no destructive cleanup.
<!-- ITO:END -->
