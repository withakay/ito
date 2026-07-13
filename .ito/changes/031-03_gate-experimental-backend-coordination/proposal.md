<!-- ITO:START -->
## Why

Ito's default Rust build currently includes the backend adapter and always compiles coordination-branch/worktree behavior even though the core spec-driven workflow does not require either subsystem. This increases the shipped surface, build coupling, and runtime ambiguity: a binary can silently contain experimental persistence and synchronization paths while the primary proposal, apply, iteration, and archive workflow only needs filesystem-backed state.

Backend persistence and coordination-branch storage still represent useful experiments, so this change establishes an explicit build boundary instead of deleting them. The default binary remains a complete spec-driven design and iteration tool, while experimental builds can opt into either subsystem independently.

## What Changes

- Add independent Cargo features for backend support and coordination-branch support; neither feature is enabled in the default build.
- Keep the default CLI's web feature enabled and preserve the Ralph, loop, and iteration commands in the default binary.
- Set the root Cargo workspace's default members to the primary CLI while retaining experimental crates as workspace members for explicit builds and checks.
- Propagate backend enablement explicitly from `ito-cli` and `ito-backend` into `ito-core`; do not rely on transitive feature unification.
- Keep backend and coordination configuration DTOs available in all builds so legacy configuration remains parseable.
- Return typed, actionable feature-unavailable errors when configuration or a command requests support that was not compiled in; do not silently fall back to filesystem or embedded storage.
- Keep the `migrate-to-main` agent instruction and its template compiled into the default binary so repositories can leave coordination storage without first installing an experimental build.
- Split build, test, lint, coverage, and release evidence between the default feature set and an explicit all-features lane.
- Preserve release support for opt-in experimental crates/features without including them in cargo-dist or Homebrew's default `ito` binary.
- Avoid claiming that gating removes shared dependencies such as `rusqlite`, `sha2`, or `hex`; these remain where default functionality still uses them.

## Change Shape

- **Type**: refactor
- **Risk**: high
- **Stateful**: yes
- **Public Contract**: cli, config
- **Design Needed**: yes
- **Design Reason**: Cargo feature propagation, legacy configuration, migration availability, CI coverage, and release contents must agree across several crates and workflows.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `rust-workspace`: Define the default workspace member and independent Cargo feature boundary while preserving the default iteration surface.
- `backend-client-runtime`: Reject backend requests with a typed error when backend support is not compiled in.
- `change-coordination-branch`: Make coordination behavior an independent opt-in feature and reject compiled-out requests without fallback.
- `cascading-config`: Continue parsing legacy backend and coordination configuration in every build.
- `release-automation`: Prove the shipped default feature set separately from the experimental all-features build.

## Impact

- Affected manifests: root `Cargo.toml`, `ito-cli`, `ito-core`, `ito-backend`, and dependent adapter manifests.
- Affected runtime composition: backend repositories, coordination synchronization/worktree wiring, configuration preflight, and feature-unavailable errors.
- Affected CLI surface: default features, experimental-only commands, Ralph/loop availability, and migration instruction rendering.
- Affected automation: Make targets, GitHub CI, documentation/coverage lanes, cargo-dist planning, release smoke checks, and crates.io feature propagation.
- Migration impact: repositories with legacy backend or coordination configuration must receive an actionable error and retain access to the migration-to-main instruction; no automatic fallback may redirect their writes.
<!-- ITO:END -->
