<!-- ITO:START -->
## Context

Ito's repository currently resolves `.ito/changes`, `specs`, `modules`, `workflows`, and `audit` through absolute symlinks into an external coordination checkout. The repository config enables coordination worktree storage, `docs/ito` claims to be the generated read-only view for plain checkouts, wiki metadata indexes that mirror, and tmux remains enabled in project configuration. This means the project that defines Ito does not yet follow the simpler main-authoritative workflow being established by module `031`.

This change is the rollout consumer of the preceding work, not a substitute for it. `031-01` supplies legacy detection and reversible migration guidance; `031-02` defines proposal-first/main-first implementation; `031-03` makes backend and coordination runtime support explicit experiments while preserving recovery; `031-04` removes tmux; and `031-05` defines the consolidated default lifecycle skills. Their integrated behavior must exist before this repository cuts over or regenerates release assets.

The migration is high risk because source state is external, the destination paths are currently symlinks, the mirror may contain unmatched historical content, and the resulting commit changes both authority and distribution claims. The design therefore separates evidence capture, materialization, parity, regeneration, and release into stop-gated phases.

## Goals / Non-Goals

**Goals:**

- Prove the five prerequisite changes are integrated on the base `main` commit.
- Capture reproducible evidence for every external managed path before mutation.
- Materialize byte-equivalent real tracked Ito directories while keeping the external checkout untouched.
- Make embedded, backend-disabled tracked state on `main` the only writable authority.
- Retire `docs/ito` and its configuration only after normalized parity is independently reproducible.
- Align raw specs, wiki metadata, canonical templates, generated assets, docs, schema, CI, and release evidence.
- Preserve a rollback route until the cutover is merged and independently reviewed.

**Non-Goals:**

- Implement or revise the product behavior owned by `031-01` through `031-05`.
- Delete the old external coordination checkout, branch, or objects.
- Automatically reconcile conflicting source, destination, or mirror content.
- Disable optional Git implementation worktrees; they may remain a branch-isolation mechanism but cannot own Ito state.
- Publish a release or create a version tag from the migration worktree.
- Reintroduce a second generated representation of canonical Ito state.

## Approach

Use a six-gate cutover executed from a branch based on the `main` commit containing `031-01` through `031-05`.

1. **Dependency gate.** Record the merge commit, strict validation result, and required checks for each prerequisite. Stop if any evidence is missing or the branch is not based on that `main` commit.
2. **Immutable evidence gate.** Resolve the five managed links without following them blindly; record link targets, external Git branch/commit/status, relative file inventories, file modes where relevant, and SHA-256 content hashes. Run the inventory/hash procedure twice and require identical output. Record the external status and hashes again after every mutating phase.
3. **Materialization gate.** Copy through a staging directory on the migration branch, reject destination collisions, replace only the five managed links with real directories, and compare the result to the source manifest. Update `.gitignore` and project config so the directories are tracked, coordination is disabled with embedded storage, backend remains disabled, and tmux configuration is absent.
4. **Mirror gate.** Compare `.ito/changes` active/archive content and `.ito/specs` with `docs/ito` through an explicit path-normalization map. Any content that exists only in the mirror is preserved and investigated. Once every difference is accounted for, record parity and remove the mirror, its path setting, publication code/workflows/tests, and current-authority documentation.
5. **Convergence gate.** Update authoritative specs and wiki sources first, then canonical project/agent guidance, templates, docs, schema sources, and finally generated harness assets. Run schema and asset generation twice; the second pass must be clean. Audit current wording for obsolete authority, tmux, mirror, and removed default-skill claims.
6. **Release gate.** Run the default and all-features verification lanes defined by the integrated `031-03`, documentation/schema/template checks, and non-publishing release planning. Obtain two independent reviews, resolve blocking findings, and map each requirement ID to evidence before the change is release-ready.

## Contracts / Interfaces

- Authority paths: `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` are real tracked directories after cutover.
- Project configuration: `changes.coordination_branch.enabled=false`, `changes.coordination_branch.storage=embedded`, and `backend.enabled=false`; the tmux tool setting and published-mirror path setting are absent.
- Migration evidence: a human-readable manifest records source Git identity, repository-relative paths, link targets, inventories, hashes, normalization decisions, commands, timestamps, and reviewer attestations without embedding machine-specific paths in durable guidance.
- Wiki source contract: current synthesis indexes `.ito/specs` and `.ito/changes`; `docs/ito` may appear only in migration history.
- Release contract: the default distributed `ito` binary uses the standard feature set; experimental support is verified in a separate all-features lane and is not silently added to default artifacts.

## Data / State

| Phase | Repository managed paths | External coordination state | `docs/ito` | Writable authority |
| --- | --- | --- | --- | --- |
| Before | Absolute symlinks | Present and authoritative | Generated view | External checkout |
| Snapshotted | Unchanged symlinks | Hashed and unchanged | Unchanged | External checkout |
| Materialized | Real tracked directories on cutover branch | Re-hashed and unchanged | Retained for comparison | External until merge |
| Parity-proven | Real tracked directories | Re-hashed and unchanged | Accounted for, ready to remove | External until merge |
| Integrated | Real tracked directories on `main` | Retained rollback evidence | Removed | `main` |

The source manifest covers regular-file bytes, relative paths, file type, and executable mode where applicable. Symlink target strings are recorded separately. Directory timestamps, ownership, and absolute source paths are evidence, not parity inputs. Audit data is copied byte-for-byte; no reconciliation or new audit event is generated solely to describe the copy.

## Decisions

- **Prerequisites are hard gates.** A self-migration cannot safely rely on behavior that is still proposed or only present on another branch.
- **Copy, never move.** The external state remains the rollback and forensic baseline; cleanup requires a later explicit decision.
- **Hash source and destination independently.** A successful copy command is not evidence of complete content parity.
- **Stop on ambiguity.** Missing paths, dirty external state, unexpected links, destination collisions, or unexplained mirror-only content require review rather than automatic conflict resolution.
- **Authority changes only on merge.** The cutover branch is reviewable prepared state; `main` becomes authoritative when that reviewed branch integrates.
- **Retire rather than redirect the mirror.** Keeping `docs/ito` as another generated view would preserve the duplication and drift risk this migration removes.
- **Canonical sources precede generated outputs.** Schema and harness regeneration must reflect reviewed source changes and must be idempotent.
- **Standard and experimental verification remain distinct.** The release proves both without expanding the standard binary's default feature surface.
- **Independent reviews examine different failure modes.** One review focuses on Rust/config/template/release correctness; the other focuses on state preservation, parity, docs, and requirement coverage.

## Risks / Trade-offs

- **Concurrent external writes could invalidate hashes.** Freeze Ito mutations during snapshot/copy, record source Git status, and re-hash after every phase; stop on change.
- **Symlink replacement can accidentally target the source.** Inspect with link metadata, operate only on repository link entries, stage copies outside both source and destination, and review resolved paths before replacement.
- **Git does not preserve every filesystem attribute.** Define parity around tracked content and executable mode; record but do not compare irrelevant timestamps or ownership.
- **Mirror layout differs from canonical layout.** Use a reviewed normalization map and require every unmatched artifact to be explained before deletion.
- **Regeneration may recreate removed assets.** Update canonical manifests/install profiles first and require a clean second generation plus negative path/reference checks.
- **All-features success could hide default-feature coupling.** Run lanes independently from clean builds and inspect the default release plan separately.
- **The retained external checkout consumes space and may confuse operators.** Mark it as rollback-only in migration evidence, but defer deletion until after release observation.

## Verification Strategy

- Strictly validate and record `031-01` through `031-05` before any cutover mutation.
- Generate deterministic, sorted manifests and SHA-256 lists twice for the source, then compare source-to-staging, source-to-materialized destination, and source-before-to-source-after.
- Assert the five repository paths are directories, not symlinks; assert they are tracked; resolve config and verify embedded/disabled coordination, disabled backend, and absent tmux settings.
- Compare normalized active, archived, and spec inventories with `docs/ito`; retain a written disposition for every difference.
- Run `ito validate 031-06_migrate-ito-authority-and-release --strict` and `ito trace 031-06_migrate-ito-authority-and-release` after spec and task changes.
- Run schema generation/check, managed-asset regeneration twice, stale-reference/path audits, docs-site checks, and repository checks.
- Run the integrated default-feature CI commands and the explicit `--all-features` build/test/lint lane from clean target state.
- Run release planning/smoke verification without publishing, pushing, tagging, or modifying the retained external store.
- Record and resolve two independent review reports, then complete a requirement-to-evidence audit before declaring release readiness.

## Migration / Rollback

Implementation starts only from the verified post-`031-05` `main`. Freeze Ito mutations, capture source evidence, prepare the materialized state and documentation changes on the `031-06` branch, and merge through normal review. Do not archive this change or publish a reset release until the merged `main` checkout resolves the real `.ito` directories and all release gates pass.

Before merge, rollback means discarding or repairing only the cutover branch; the external coordination checkout remains authoritative and untouched. After merge but before external cleanup, rollback is a reviewed revert that restores the prior tracked link/config entries and points back to the retained, hash-verified external state. Deleting the external checkout is intentionally excluded from this change, so rollback evidence survives the initial release.

## Open Questions

None. The approved decisions are dependency-gated cutover, copy-and-prove preservation, embedded tracked authority on `main`, mirror retirement after parity, no tmux, disabled backend by default, source-first regeneration, dual-lane verification, and two independent reviews.
<!-- ITO:END -->
