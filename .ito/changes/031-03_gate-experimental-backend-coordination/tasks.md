<!-- ITO:START -->
# Tasks for: 031-03_gate-experimental-backend-coordination

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates; keep exactly one task in progress.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`
- **Testing**: Use RED/GREEN/REFACTOR and the repository's test-with-subagent workflow for Rust builds, tests, and checks.
- **Sequencing**: Do not disable coordination in the default build until the feature-neutral `migrate-to-main` instruction from `031-01_migrate-coordination-state-to-main` is present and passing its default-build tests.
- **Scope**: Preserve Ralph, loop, and the general iteration workflow in every default-build checkpoint.

```bash
ito tasks status 031-03_gate-experimental-backend-coordination
ito tasks next 031-03_gate-experimental-backend-coordination
ito tasks start 031-03_gate-experimental-backend-coordination 1.1
ito tasks complete 031-03_gate-experimental-backend-coordination 1.1
```

______________________________________________________________________

## Wave 1: Cargo Boundary and Typed Capability Contract

- **Depends On**: None

### Task 1.1: Define the workspace feature matrix

- **Files**: `Cargo.toml`, `ito-rs/crates/ito-cli/Cargo.toml`, `ito-rs/crates/ito-core/Cargo.toml`, `ito-rs/crates/ito-backend/Cargo.toml`, `ito-rs/crates/ito-web/Cargo.toml`, `Cargo.lock`
- **Dependencies**: None
- **Action**: Add root `default-members`; make core defaults empty; define independent `backend` and `coordination-branch` features; change CLI defaults to web only; propagate features explicitly with `default-features = false`; keep experimental crates as workspace members. Make only truly feature-exclusive dependencies optional, leaving `rusqlite`, `sha2`, and `hex` where default consumers still require them.
- **Verify**: `cargo metadata --no-deps --format-version 1` and focused manifest tests/assertions for default members and feature propagation.
- **Done When**: Cargo metadata expresses four valid combinations (default, backend-only, coordination-only, all-features), `ito-backend` explicitly activates core backend support, and neither experimental feature implies the other.
- **Requirements**: rust-workspace:independent-experimental-features, rust-workspace:primary-default-member, backend-client-runtime:explicit-feature-propagation, change-coordination-branch:independent-feature-gate
- **Status**: [x] complete
- **Updated At**: 2026-07-13

### Task 1.2: Introduce the compiled-capability preflight and typed error

- **Files**: `ito-rs/crates/ito-core/src/errors.rs`, `ito-rs/crates/ito-core/src/compiled_capabilities.rs`, `ito-rs/crates/ito-core/src/repository_runtime.rs`, sibling `*_tests.rs` files, `ito-rs/crates/ito-cli/src/runtime.rs`, `ito-rs/crates/ito-cli/src/app/run.rs`
- **Dependencies**: Task 1.1
- **Action**: Add a feature-neutral capability descriptor and typed feature-unavailable error carrying the missing feature, requester, and recovery. Run preflight after cascading config is parsed but before repository, audit, link, or write initialization. Classify recovery-safe commands so help, config/init/update migration paths, and migration instruction rendering remain usable.
- **Verify**: Focused core and CLI tests built without experimental features, including assertions on the structured error fields and proof that fallback repositories/mutations are never constructed.
- **Done When**: Compiled-out requests fail through one typed contract, invalid configuration remains distinguishable, and stateful commands cannot silently select filesystem or embedded fallback.
- **Requirements**: backend-client-runtime:compiled-out-error, change-coordination-branch:compiled-out-error, cascading-config:parse-compiled-out-features
- **Status**: [x] complete
- **Updated At**: 2026-07-13

### Task 1.3: Gate backend implementation code

- **Files**: `ito-rs/crates/ito-core/src/lib.rs`, `ito-rs/crates/ito-core/src/backend_*.rs`, `ito-rs/crates/ito-core/src/event_forwarder.rs`, `ito-rs/crates/ito-core/src/fs_project_store.rs`, `ito-rs/crates/ito-core/src/remote_task_repository.rs`, `ito-rs/crates/ito-core/src/token.rs`, `ito-rs/crates/ito-core/src/artifact_mutations.rs`, `ito-rs/crates/ito-core/src/task_mutations.rs`, `ito-rs/crates/ito-core/src/repository_runtime.rs`, `ito-rs/crates/ito-core/src/audit/store.rs`, corresponding sibling tests
- **Dependencies**: Task 1.2
- **Action**: Apply backend feature gates to cohesive backend modules and to remote-only branches in shared modules. Preserve filesystem/default branches unconditionally. Gate tests with the implementation and remove no shared dependency without evidence.
- **Verify**: `cargo check -p ito-cli` and `cargo check -p ito-cli --no-default-features --features backend`; focused backend tests in the backend-enabled lane.
- **Done When**: The default CLI does not compile backend implementation modules or depend normally on `ito-backend`, while a backend-only build compiles and exercises the full backend path without coordination support.
- **Requirements**: rust-workspace:independent-experimental-features, backend-client-runtime:explicit-feature-propagation, backend-client-runtime:compiled-out-error
- **Status**: [x] complete
- **Updated At**: 2026-07-13

### Task 1.4: Gate coordination implementation code

- **Files**: `ito-rs/crates/ito-core/src/lib.rs`, `ito-rs/crates/ito-core/src/coordination.rs`, `ito-rs/crates/ito-core/src/coordination_worktree.rs`, `ito-rs/crates/ito-core/src/git.rs`, `ito-rs/crates/ito-core/src/git_remote.rs`, `ito-rs/crates/ito-core/src/repo_paths.rs`, `ito-rs/crates/ito-core/src/create/mod.rs`, `ito-rs/crates/ito-core/src/worktree_ensure.rs`, `ito-rs/crates/ito-core/src/validate_repo/`, corresponding sibling tests, coordination callers under `ito-rs/crates/ito-cli/src/`
- **Dependencies**: Task 1.2
- **Action**: Gate reservation/fetch/push, worktree lifecycle, link wiring/repair, and coordination-only validation. Route shared callers through the compiled-capability facade so the default implementation remains main-compatible and worktree configuration cannot be silently reinterpreted.
- **Verify**: `cargo check -p ito-cli` and `cargo check -p ito-cli --no-default-features --features coordination-branch`; focused coordination tests in the coordination-enabled lane.
- **Done When**: The default CLI omits coordination implementation code, a coordination-only build works without backend, and compiled-out coordination requests reach the typed error before artifact access.
- **Requirements**: rust-workspace:independent-experimental-features, change-coordination-branch:independent-feature-gate, change-coordination-branch:compiled-out-error
- **Status**: [x] complete
- **Updated At**: 2026-07-13

______________________________________________________________________

## Wave 2: Default CLI, Configuration, and Recovery Surface

- **Depends On**: Wave 1

### Task 2.1: Preserve the standard CLI and compatibility dispatch

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`, `ito-rs/crates/ito-cli/src/app/run.rs`, `ito-rs/crates/ito-cli/src/commands/mod.rs`, backend/coordination command modules, CLI help snapshots and integration tests
- **Dependencies**: None
- **Action**: Keep proposal lifecycle, Ralph, loop, and iteration commands in default help and dispatch. Retain lightweight compatibility parsing for compiled-out experimental commands, hide those commands from standard discovery where appropriate, and return the typed feature error when invoked.
- **Verify**: Default-build CLI help/snapshot tests plus command tests for Ralph, loop, backend compatibility, and coordination compatibility.
- **Done When**: Standard workflows are unchanged, experimental implementations do not leak into the default help surface, and known compiled-out requests produce actionable structured errors instead of unknown-command or fallback behavior.
- **Requirements**: rust-workspace:default-iteration-surface, backend-client-runtime:compiled-out-error, change-coordination-branch:compiled-out-error
- **Status**: [x] complete
- **Updated At**: 2026-07-13

### Task 2.2: Keep migrate-to-main feature-neutral

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/migrate-to-main.md.j2`, instruction asset registry, `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-cli/src/cli/agent.rs`, instruction rendering and distribution tests
- **Dependencies**: Task 2.1
- **Action**: Integrate the migration instruction outside all backend and coordination cfg gates. Ensure its renderer uses feature-neutral config DTOs and read-only context, retains JSON output, and is allowed through legacy-config preflight.
- **Verify**: `cargo test -p ito-cli --no-default-features migrate_to_main` (or the focused final test target selected during implementation) and a CLI invocation of `ito agent instruction migrate-to-main` from a binary without experimental features.
- **Done When**: The standard binary renders the recovery instruction in text and JSON modes even when legacy worktree config is active, without compiling or invoking coordination synchronization.
- **Requirements**: rust-workspace:default-iteration-surface, change-coordination-branch:compiled-out-error
- **Status**: [x] complete
- **Updated At**: 2026-07-13

### Task 2.3: Preserve legacy config parsing and adopt main-compatible defaults

- **Files**: `ito-rs/crates/ito-config/src/config/types.rs`, `ito-rs/crates/ito-config/src/config/coordination_storage_tests.rs`, config schema snapshots/artifacts, config validation tests, compiled-capability tests
- **Dependencies**: Task 2.1
- **Action**: Keep backend and coordination DTOs/schema fields unconditional. Change new/default coordination configuration to disabled and embedded/main-compatible storage while keeping backend disabled. Add tests for legacy recognized fields, invalid values, feature-unavailable classification, and recovery command exemptions.
- **Verify**: Focused `ito-config` and CLI/core preflight tests plus the repository's config-schema freshness check.
- **Done When**: Old config parses in the default binary, new config does not request compiled-out coordination, invalid values remain config errors, and explicit legacy feature requests never fall back.
- **Requirements**: cascading-config:parse-compiled-out-features, backend-client-runtime:compiled-out-error, change-coordination-branch:compiled-out-error
- **Status**: [x] complete
- **Updated At**: 2026-07-13

______________________________________________________________________

## Wave 3: Build, CI, and Release Evidence

- **Depends On**: Wave 2

### Task 3.1: Split Make targets by shipping and experimental feature sets

- **Files**: `Makefile`, any invoked scripts under `scripts/` or `ito-rs/tools/`
- **Dependencies**: None
- **Action**: Make default build/test/clippy/docs/coverage/check targets exercise the shipping CLI feature set. Add explicitly named experimental/all-features targets for workspace backend and coordination coverage. Remove ambiguous `--workspace` usage where it defeats the default-member boundary.
- **Verify**: Dry-run or execute the focused default and experimental Make targets with the test-with-subagent workflow.
- **Done When**: A contributor can tell which feature set each target verifies, default targets match distributed behavior, and experimental targets still cover opt-in code.
- **Requirements**: release-automation:split-feature-verification, rust-workspace:primary-default-member
- **Status**: [>] in-progress
- **Updated At**: 2026-07-13

### Task 3.2: Split GitHub CI into default and all-features lanes

- **Files**: `.github/workflows/ci.yml`, reusable local actions or scripts used by CI
- **Dependencies**: Task 3.1
- **Action**: Add required default/shipping and experimental/all-features test and lint lanes, with docs/coverage selection made explicit. Include a default dependency-tree assertion and standard CLI surface tests; retain backend and coordination tests in the all-features lane.
- **Verify**: Workflow syntax validation and execution of the exact Cargo/Make commands locally where possible.
- **Done When**: CI cannot pass solely because feature unification compiled experimental code, and both shipped and opt-in combinations have required evidence.
- **Requirements**: release-automation:split-feature-verification, rust-workspace:primary-default-member
- **Status**: [ ] pending
- **Updated At**: 2026-07-13

### Task 3.3: Pin and prove standard release contents

- **Files**: `dist-workspace.toml`, `.github/workflows/v-release.yml` if regenerated, `release-plz.toml`, package metadata, release smoke tests/scripts, documentation describing experimental installation
- **Dependencies**: Task 3.2
- **Action**: Ensure cargo-dist, GitHub Release, installers, and Homebrew package `ito-cli` with web-only defaults and no experimental implementation. Keep `ito-backend` version-aligned/publishable if `cargo install ito-cli --features backend` remains supported. Add reproducible feature/dependency evidence without asserting removal of shared crates.
- **Verify**: `dist plan` or the repository's cargo-dist check, release smoke tests, and `cargo tree -p ito-cli` assertions for default and experimental selections.
- **Done When**: Standard artifacts prove their feature set, opt-in backend resolution remains viable, and evidence accurately reports why `rusqlite`, `sha2`, or `hex` may remain.
- **Requirements**: release-automation:default-artifact-features, release-automation:accurate-dependency-evidence, backend-client-runtime:explicit-feature-propagation
- **Status**: [ ] pending
- **Updated At**: 2026-07-13

______________________________________________________________________

## Wave 4: Feature Matrix and Final Quality Gate

- **Depends On**: Wave 3

### Task 4.1: Exercise all four feature combinations

- **Files**: Feature-matrix test script or Make targets, focused Rust integration tests and snapshots updated by earlier tasks
- **Dependencies**: None
- **Action**: Run default, backend-only, coordination-only, and all-features build/test/lint combinations. Confirm backend-only and coordination-only do not activate each other, and default tests cover typed legacy-config failures plus standard iteration/recovery commands.
- **Verify**: The exact feature-matrix commands documented in Make/CI, executed through the test-with-subagent workflow.
- **Done When**: Every supported combination compiles, its expected tests pass, and Cargo metadata/tree evidence matches the intended feature graph.
- **Requirements**: rust-workspace:independent-experimental-features, rust-workspace:default-iteration-surface, backend-client-runtime:explicit-feature-propagation, change-coordination-branch:independent-feature-gate, release-automation:split-feature-verification
- **Status**: [ ] pending
- **Updated At**: 2026-07-13

### Task 4.2: Capture reproducible dependency and artifact evidence

- **Files**: `.ito/changes/031-03_gate-experimental-backend-coordination/demos/` or the repository's approved Showboat evidence location within this change
- **Dependencies**: Task 4.1
- **Action**: Produce Showboat evidence for root default-member selection, default and all-features Cargo trees, typed compiled-out errors, Ralph/loop availability, migration instruction rendering, and cargo-dist/release selection. Call out shared dependencies that remain and why.
- **Verify**: Replay the Showboat document commands successfully against the implementation worktree.
- **Done When**: Reviewers can reproduce the build boundary and release claims from captured command output without relying on narrative assertions.
- **Requirements**: release-automation:default-artifact-features, release-automation:accurate-dependency-evidence, rust-workspace:primary-default-member, rust-workspace:default-iteration-surface
- **Status**: [ ] pending
- **Updated At**: 2026-07-13

### Task 4.3: Complete independent review and repository checks

- **Files**: All files changed by this implementation; proposal artifacts only for any required clarification
- **Dependencies**: Task 4.2
- **Action**: Run the required Rust-focused and general diff review passes, resolve findings, then run default and experimental repository checks. Validate traceability and the Ito change package.
- **Verify**: `ito trace 031-03_gate-experimental-backend-coordination`, `ito validate 031-03_gate-experimental-backend-coordination --strict`, the default `make check`, and the experimental/all-features check target.
- **Done When**: Reviews have no unresolved correctness findings, every requirement is covered by completed tasks/tests, both build lanes pass, and strict Ito validation succeeds.
- **Requirements**: cascading-config:parse-compiled-out-features, backend-client-runtime:compiled-out-error, change-coordination-branch:compiled-out-error, release-automation:split-feature-verification
- **Status**: [ ] pending
- **Updated At**: 2026-07-13

______________________________________________________________________

## Wave Guidelines

- Waves group tasks that can run in parallel within the wave after their explicit dependencies are satisfied.
- Wave N depends on all prior waves completing.
- Use Ito task commands for every status transition; do not edit status markers manually during implementation.
- Stop before changing default features if `031-01` migration recovery is not integrated and verified.
<!-- ITO:END -->
