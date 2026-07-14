<!-- ITO:START -->
## Why

Changes `031-01` through `031-05` make a smaller, main-first Ito possible, but Ito's own repository still keeps authoritative state behind external coordination symlinks and publishes a second generated view under `docs/ito`. The reset is not complete until the repository performs a lossless, reviewable cutover to tracked `.ito` state on `main`, removes the obsolete mirror and tmux surfaces, and proves that both the standard and experimental release lanes remain healthy.

## What Changes

- Make authority mutation in this change depend on the reviewed and verified implementation of `031-01_migrate-coordination-state-to-main`, `031-02_enforce-main-first-implementation`, `031-03_gate-experimental-backend-coordination`, `031-04_remove-tmux-integration`, and `031-05_consolidate-seven-lifecycle-skills`, integrated first as ancestors of one main-bound cutover branch.
- Snapshot the external coordination checkout's Git identity, managed-path inventory, symlink metadata, and deterministic content hashes before copying `.ito/{changes,specs,modules,workflows,audit}` into real tracked directories.
- Stop on missing, conflicting, or non-equivalent content; never delete, rewrite, commit in, or otherwise mutate the old external coordination worktree or branch.
- Change this repository's configuration to disabled embedded coordination storage, keep `backend.enabled=false`, remove tmux configuration and generated tmux assets, and establish committed `.ito` artifacts on `main` as Ito's sole writable authority.
- Verify the materialized `.ito` state against `docs/ito`, record any intentional layout normalization, and only then retire the generated published mirror, its configuration, generation paths, and source-of-truth claims.
- Update canonical specs, wiki metadata and topic pages, project/source guidance, instruction templates, user documentation, and generated harness assets so every supported surface describes the main-authoritative lifecycle and the reduced default skill set.
- Regenerate the configuration schema and all Ito-managed assets from their canonical sources, with stale coordination, mirror, tmux, and removed-skill outputs absent from default installations.
- Gate release readiness on strict Ito validation, requirement traceability, default-feature and all-features CI lanes, non-publishing release-plan checks, two independent review passes, and a final requirement audit.
- **BREAKING**: Remove `docs/ito` as a supported generated mirror and remove its published-mirror path configuration; consumers must read the tracked `.ito` artifacts on `main`.

## Change Shape

- **Type**: migration
- **Risk**: high
- **Stateful**: yes
- **Public Contract**: config
- **Design Needed**: yes
- **Design Reason**: The cutover changes the repository's authoritative state, removes a published contract, crosses Git and filesystem boundaries, and must remain reversible until independent parity checks complete.

## Capabilities

### New Capabilities

- `ito-authority-cutover`: Define the dependency-gated, lossless migration that makes tracked `.ito` state on `main` authoritative and proves the reset is ready to release.

### Modified Capabilities

- `published-ito-mirror`: Remove the generated `docs/ito` mirror after its content is proven equivalent to the materialized tracked state.
- `ito-config-crate`: Remove the obsolete published-mirror path configuration once `.ito` on `main` is directly readable.

## Impact

- Repository state: `.ito/{changes,specs,modules,workflows,audit}`, `.ito/config.json`, `.gitignore`, the retained external coordination checkout, and the `main` integration sequence.
- Canonical sources: `.ito/specs`, `.ito/wiki`, `.ito/user-prompts`, root agent guidance, `ito-rs/crates/ito-templates/assets`, and configuration definitions.
- Generated/distributed surfaces: `schemas/ito-config.schema.json`, supported harness commands, skills and agents, documentation, and `docs/ito` retirement.
- Verification and release: default-feature and all-features Rust lanes, template/schema determinism, docs checks, release planning, independent review evidence, and Ito requirement traceability.
<!-- ITO:END -->
