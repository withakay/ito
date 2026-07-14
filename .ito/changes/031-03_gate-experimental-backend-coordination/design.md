<!-- ITO:START -->
## Context

The repository has a root Cargo workspace containing `ito-cli`, `ito-core`, `ito-backend`, `ito-web`, and supporting crates. Today a plain workspace build selects every member because the root manifest has no `default-members`. The CLI declares `web` and `backend` as default features, while `ito-core` has no features and exports backend and coordination modules unconditionally. Consequently, removing the CLI's optional `ito-backend` edge is not enough to remove backend runtime code from the standard binary, and there is no equivalent boundary around coordination code.

Runtime configuration compounds the build problem. Backend configuration defaults disabled, but coordination configuration currently defaults enabled with worktree storage. A default binary that simply compiles out the implementation cannot silently reinterpret existing configuration: repositories may still contain symlinks to a coordination worktree, and fallback writes could update a source the binary can no longer synchronize.

This change is part of the `031_ito-core-reset` sequence. `031-01_migrate-coordination-state-to-main` owns the migration contract and the `migrate-to-main` instruction. `031-02_enforce-main-first-implementation` owns main-first workflow policy. This change owns the Cargo/build boundary and the compiled-capability contract; it must preserve the recovery surface introduced by `031-01`.

## Goals / Non-Goals

**Goals:**

- Ship a default `ito` binary with web and the complete spec-driven lifecycle, including Ralph and loop iteration, but without backend or coordination implementation code.
- Allow backend and coordination support to be enabled independently or together in experimental builds.
- Keep manifests, core modules, CLI command dispatch, config parsing, automation, and release artifacts aligned on the same feature matrix.
- Diagnose legacy or explicit requests for compiled-out behavior with a typed error before any fallback repository or write path is selected.
- Keep migration-to-main rendering available in every standard binary.
- Continue testing experimental code explicitly without letting its success mask failures in the shipped default feature set.

**Non-Goals:**

- Delete backend or coordination source code.
- Implement the coordination-to-main data migration; that belongs to `031-01`.
- Define the main-first proposal approval workflow; that belongs to `031-02`.
- Remove local SQLite-backed validation or task analysis.
- Promise removal of `rusqlite`, `sha2`, or `hex` from the default dependency graph.
- Stabilize the experimental backend or coordination contracts.

## Approach

### Cargo feature graph

The root workspace keeps all existing members but declares `ito-cli` as its only default member. Plain root Cargo commands therefore target the primary product, while explicit `--workspace` commands can still cover experimental members.

`ito-core` gains two additive features with no default features:

| Feature | Owns | Does not imply |
| --- | --- | --- |
| `backend` | Remote runtime resolution, HTTP repositories, backend sync/import/archive/event forwarding, auth/token helpers, backend project stores and backend mutation branches | `coordination-branch` |
| `coordination-branch` | Reservation/fetch/push, coordination worktree lifecycle, link wiring/repair, coordination-specific validation and path helpers | `backend` |

`ito-cli` defaults to `web` only. Its `backend` feature enables both the optional `ito-backend` dependency and `ito-core/backend`. Its `coordination-branch` feature enables `ito-core/coordination-branch`. An optional aggregate experimental feature may enable both, but neither individual feature may route through that aggregate.

Every internal dependency on `ito-core` declares `default-features = false`. The `ito-backend` manifest explicitly requests `ito-core/backend`. `ito-web` requests core without experimental features. This makes feature intent visible at each adapter boundary rather than relying on Cargo's workspace-wide feature unification.

Core implementation modules and backend/coordination branches inside shared modules use `cfg(feature = ...)`. Shared configuration DTOs and a small compiled-capability facade remain unconditional. Where a shared module currently mixes filesystem, SQLite, and remote behavior, only the remote variants/imports/factories are gated; filesystem behavior remains the default implementation.

Backend-only dependencies that have no default consumer become optional core dependencies. Windows `junction` support is tied to `coordination-branch`. Dependencies with real default consumers remain unconditional: `rusqlite` is still used by validation and task analysis, while `sha2` and `hex` are still used by front-matter behavior.

### Runtime capability preflight

Configuration deserialization remains feature-neutral. After Ito resolves cascading configuration but before it initializes a repository, audit store, coordination link, or other mutable subsystem, a compiled-capability preflight compares requested behavior with compile-time capabilities.

The error contract is a typed feature-unavailable variant carrying:

- the stable feature identifier (`backend` or `coordination-branch`),
- what requested it, such as `backend.enabled`, `changes.coordination_branch.storage`, or a compatibility command,
- a concise recovery action.

The dispatcher preserves lightweight compatibility parsing for experimental-only commands when their implementation is compiled out. Those commands may be hidden from default help, but invoking them reaches the same typed error rather than an unrelated unknown-command or fallback path.

The preflight classifies recovery-safe commands separately. Help, version, configuration diagnostics/migration, init/update migration paths, and `ito agent instruction migrate-to-main` remain usable when legacy coordination config is present. All ordinary stateful commands fail before they can follow old coordination symlinks or select filesystem persistence as a backend substitute.

New/default configuration becomes main-compatible: backend remains disabled, and coordination defaults disabled with embedded/main-tracked storage. Compiling an experimental feature makes it available but does not activate it; users still opt in through configuration.

### Migration instruction boundary

The `migrate-to-main.md.j2` asset, instruction identifier, rendering context, CLI dispatch, JSON envelope, and help text are part of the standard instruction bundle. They must not import coordination implementation modules. The renderer may inspect configuration and filesystem metadata through feature-neutral DTOs and read-only helpers, then emit instructions for an agent to execute.

This is a hard sequencing gate: the feature split must not ship until the standard binary can render the migration instruction and tests prove that behavior without experimental features.

### Automation and distribution

Build automation gains two named lanes:

- **Default/shipping lane:** build, test, lint, docs, and coverage using the same default feature set as cargo-dist. It asserts that `ito-backend` is absent from the normal dependency graph and that Ralph, loop, web, and migration instruction surfaces remain available.
- **Experimental/all-features lane:** explicitly select workspace members and all features, exercising backend and coordination implementations without affecting artifact defaults.

Make targets mirror these lanes so local and CI commands do not disagree. Existing `--workspace` commands are updated deliberately because Cargo `default-members` does not affect an explicit workspace selection.

Cargo-dist continues packaging only `ito-cli`; the CLI's default feature declaration determines the standard GitHub Release and Homebrew binary. Release evidence records or tests the compiled feature set so a future manifest edit cannot reintroduce experimental code unnoticed. `ito-backend` remains a version-aligned publishable crate if crates.io users are expected to install `ito-cli --features backend`.

## Contracts / Interfaces

### Cargo contracts

- Root workspace default member: `ito-cli`.
- CLI default features: `web` only.
- Independent CLI/core features: `backend`, `coordination-branch`.
- Backend adapter dependency: core defaults disabled, core backend enabled explicitly.
- Web adapter dependency: core defaults disabled, no experimental feature implied.

### Error contract

Requests for compiled-out behavior return a typed feature-unavailable error. Human rendering names the missing feature, the requesting config or command, and recovery. Machine-readable rendering retains a stable error kind and feature identifier. No unavailable-feature path may construct a fallback repository or mutate project state.

### CLI compatibility contract

- Ralph and loop remain standard commands.
- `ito agent instruction migrate-to-main [--json]` remains a standard command.
- Experimental commands are available in feature-enabled builds.
- A recognized experimental command in a default build reports feature unavailability through a compatibility dispatch path.

## Data / State

This change adds no new persistent application data. It changes how existing configuration is interpreted relative to compiled capabilities.

| Parsed state | Compiled capability | Result |
| --- | --- | --- |
| Backend disabled | Backend absent or present | Filesystem/main-compatible runtime continues |
| Backend enabled | Backend present | Remote runtime resolution continues |
| Backend enabled | Backend absent | Typed `backend` feature-unavailable error before repository/audit initialization |
| Coordination disabled/embedded | Coordination absent or present | Main-compatible storage continues |
| Coordination enabled/worktree | Coordination present | Experimental coordination behavior continues |
| Coordination enabled/worktree | Coordination absent | Typed `coordination-branch` feature-unavailable error before link or artifact access; migration instruction remains available |

Legacy config fields remain round-trippable and visible to schema/config diagnostics. Migration of legacy coordination data remains explicit and reversible under `031-01`; feature gating never rewrites state automatically.

## Decisions

- **Backend and coordination are separate features.** They solve different problems and must be independently testable. An aggregate feature is convenience only.
- **Core defaults are empty; CLI defaults express the product.** This prevents adapter choices from leaking through core defaults and makes the shipped surface legible in one manifest.
- **The standard CLI keeps web.** This change removes backend and coordination from defaults without expanding scope into the proposal viewer/web surface.
- **Iteration remains default.** Ralph and loop are core product workflows, not backend or coordination extensions.
- **Configuration DTOs remain unconditional.** A default build must understand legacy config well enough to diagnose and migrate it.
- **Unavailable means error, never fallback.** Silent fallback could redirect writes or create divergent sources of truth.
- **Recovery rendering is feature-neutral.** The migration instruction is deliberately compiled outside the coordination gate.
- **Default members do not replace explicit CI selection.** CI and Make targets are split because existing `--workspace` flags bypass the root default-member choice.
- **Release artifacts follow the CLI default manifest.** Experimental crates may remain publishable, but cargo-dist and Homebrew do not opt into their features.
- **Dependency claims require evidence.** Shared crates remain until separate refactors prove they are unnecessary.

## Risks / Trade-offs

- **Broad cfg surface:** Shared runtime modules currently mix local and remote variants. Mitigation: gate cohesive modules and narrow branches, then compile all four feature combinations.
- **Legacy repositories become blocked:** A default binary will refuse ordinary work while legacy worktree config is active. Mitigation: keep diagnostics and `migrate-to-main` available and land `031-01` first.
- **Feature unification can hide missing propagation:** Workspace all-features builds can succeed despite an adapter manifest omission. Mitigation: test packages and feature combinations separately, including backend-only and coordination-only builds.
- **Experimental code may decay outside defaults:** Mitigation: retain a required all-features CI lane and explicit local targets.
- **Release drift:** cargo-dist may silently follow a future default-feature edit. Mitigation: release smoke tests inspect compiled capabilities/dependency evidence.
- **Command discoverability:** Hiding compiled-out experimental commands reduces help noise but can surprise existing users. Mitigation: compatibility parsing returns the typed feature error with exact install/migration guidance.

## Verification Strategy

- Cargo metadata and tree assertions for root default members, CLI defaults, optional dependencies, and absence of `ito-backend` from the standard graph.
- Compile/test matrix for default, backend-only, coordination-only, and all-features builds.
- Runtime tests for legacy backend and coordination config in compiled-out builds, including proof that no fallback mutation occurs.
- Config/schema tests proving legacy DTOs deserialize in the default build and invalid values remain ordinary config errors.
- CLI tests proving Ralph, loop, web, and `migrate-to-main` remain in the standard surface.
- Backend adapter tests proving it explicitly activates core backend support.
- Coordination tests confined to the coordination feature lane.
- Make and GitHub workflow tests/review proving default and experimental commands are named and selected explicitly.
- Cargo-dist/release plan or smoke evidence proving standard artifacts use only default CLI features.
- Before/after dependency evidence that accurately calls out shared dependencies which remain.
- `ito validate 031-03_gate-experimental-backend-coordination --strict` for proposal integrity.

## Migration / Rollback

1. Land the `031-01` feature-neutral migration instruction and its recovery tests.
2. Add feature declarations and explicit propagation while preserving current behavior in all-features builds.
3. Add typed preflight and compatibility dispatch before changing default features.
4. Change coordination configuration defaults for new/main-compatible projects and test legacy config diagnostics.
5. Remove backend and coordination from CLI defaults; add root default members.
6. Split Make, CI, coverage, docs, and release evidence.
7. Verify all feature combinations and standard release artifacts before rollout.

Repositories with active legacy coordination storage use `ito agent instruction migrate-to-main` before ordinary work with the standard binary. No automatic data move occurs in this change.

Rollback is manifest-first: temporarily re-enable the experimental features in the CLI default feature set while preserving the typed error and config parsing work. Source and legacy DTOs remain intact, so rollback does not require a data migration. Do not silently flip legacy storage modes during rollback.

## Open Questions

None. The feature matrix, default CLI surface, error behavior, migration availability, and automation split are approved decisions for this change.
<!-- ITO:END -->
