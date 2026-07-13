<!-- ITO:START -->
# Tasks for: 031-06_migrate-ito-authority-and-release

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates; keep exactly one task in progress.
- **Safety**: Freeze Ito mutations before Task 1.2 and never run sync, commit, reset, push, delete, or cleanup commands in the external coordination checkout.
- **Stop conditions**: Any missing dependency evidence, dirty/changing source, unexpected link, collision, hash mismatch, unexplained mirror difference, failed gate, or blocking review finding stops the current wave.
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 031-06_migrate-ito-authority-and-release
ito tasks next 031-06_migrate-ito-authority-and-release
ito tasks start 031-06_migrate-ito-authority-and-release 1.1
ito tasks complete 031-06_migrate-ito-authority-and-release 1.1
```

______________________________________________________________________

## Wave 1: Readiness and immutable evidence

- **Depends On**: None

### Task 1.1: Prove the core-reset dependencies are integrated

- **Files**: `.ito/changes/031-06_migrate-ito-authority-and-release/evidence/dependencies.md`, Git history, CI evidence for `031-01` through `031-05`
- **Dependencies**: None
- **Action**: Confirm each preceding module change is approved, implemented, merged to `main`, strictly valid, and green on its required checks. Record each integrated commit and create the cutover branch from the resulting `main`; stop if any dependency is only proposed, task-complete, or unmerged.
- **Verify**: `ito validate 031-01_migrate-coordination-state-to-main --strict && ito validate 031-02_enforce-main-first-implementation --strict && ito validate 031-03_gate-experimental-backend-coordination --strict && ito validate 031-04_remove-tmux-integration --strict && ito validate 031-05_consolidate-seven-lifecycle-skills --strict`
- **Done When**: The evidence names five merged commits, their required green checks, and the exact `main` base commit used by `031-06`.
- **Requirements**: ito-authority-cutover:dependency-gated-cutover
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 1.2: Snapshot and independently hash external authority

- **Files**: `.ito/changes/031-06_migrate-ito-authority-and-release/evidence/source-manifest.md`, `.ito/changes/031-06_migrate-ito-authority-and-release/evidence/source-files.sha256`, `.ito/changes/031-06_migrate-ito-authority-and-release/evidence/source-links.txt`
- **Dependencies**: Task 1.1
- **Action**: Freeze Ito mutations; inspect the five managed links without following them implicitly; record external path, branch, commit, status, link targets, sorted relative inventories, file types/modes, and SHA-256 values. Generate the inventory and hashes twice by independent runs and stop on dirty, broken, unexpected, missing, or changing source state.
- **Verify**: `sha256sum --check .ito/changes/031-06_migrate-ito-authority-and-release/evidence/source-files.sha256`
- **Done When**: Repeated manifests are identical, every managed path is covered, the source commit/status is recorded, and no source mutation has occurred.
- **Requirements**: ito-authority-cutover:external-state-preservation
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2: Materialize tracked authority

- **Depends On**: Wave 1

### Task 2.1: Stage and materialize the five managed directories

- **Files**: `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, `.ito/audit`, cutover staging area, source/destination parity evidence
- **Dependencies**: None
- **Action**: Copy the approved external snapshot to an isolated staging directory, verify it, reject all destination collisions, then replace only the repository's five link entries with real directories. Preserve file bytes and executable modes; do not move or modify source content.
- **Verify**: `test ! -L .ito/changes && test ! -L .ito/specs && test ! -L .ito/modules && test ! -L .ito/workflows && test ! -L .ito/audit && sha256sum --check .ito/changes/031-06_migrate-ito-authority-and-release/evidence/source-files.sha256`
- **Done When**: All five paths are real directories, their normalized inventories and hashes match the source manifest, and the external source still reproduces its original hashes.
- **Requirements**: ito-authority-cutover:external-state-preservation, ito-authority-cutover:tracked-main-authority
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 2.2: Configure embedded main authority and remove local legacy wiring

- **Files**: `.ito/config.json`, `.gitignore`, project tmux configuration/assets, coordination wiring referenced by project setup
- **Dependencies**: Task 2.1
- **Action**: Set coordination to disabled embedded storage, keep backend disabled, remove the tmux tool setting and all project-generated tmux assets, remove only the managed ignore/link wiring made obsolete by real tracked directories, and retain optional implementation worktree settings without allowing them to own Ito state.
- **Verify**: `ito config get changes.coordination_branch && ito config get backend.enabled && test -z "$(rg -l 'tools[.]tmux|ito-tmux|/tmux/' .ito/config.json .claude .codex .github .opencode .pi 2>/dev/null)"`
- **Done When**: Resolved config is embedded/disabled with backend false, tmux is absent, the five directories are trackable, and external-state hashes remain unchanged.
- **Requirements**: ito-authority-cutover:tracked-main-authority
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 2.3: Prove the prepared branch is main-compatible

- **Files**: `.ito/changes/031-06_migrate-ito-authority-and-release/evidence/materialized-parity.md`, Git index/diff
- **Dependencies**: Task 2.2
- **Action**: Run the integrated legacy-state detector and Ito validation, inspect the Git diff for tracked real directories, reproduce source-to-destination parity, and record that authority changes only when the reviewed branch merges to `main`.
- **Verify**: `ito validate 031-06_migrate-ito-authority-and-release --strict && git diff --check`
- **Done When**: The branch is reported main-compatible, every materialized file is accounted for in Git, no managed link remains, and source-before/source-after hashes agree.
- **Requirements**: ito-authority-cutover:external-state-preservation, ito-authority-cutover:tracked-main-authority
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3: Prove parity and retire the published mirror

- **Depends On**: Wave 2

### Task 3.1: Audit `docs/ito` against materialized authority

- **Files**: `docs/ito`, `.ito/changes`, `.ito/specs`, `.ito/changes/031-06_migrate-ito-authority-and-release/evidence/mirror-parity.md`
- **Dependencies**: None
- **Action**: Define and review the active/archive/spec path-normalization map; compare sorted inventories and content hashes; record the disposition of every difference. Preserve and investigate mirror-only content rather than dropping it.
- **Verify**: `ito validate --strict && git diff --check`
- **Done When**: Every active change, archived change, and current spec in `docs/ito` maps to byte-equivalent authoritative content or a documented approved normalization, with no unexplained mirror-only data.
- **Requirements**: ito-authority-cutover:mirror-parity-before-retirement
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 3.2: Remove the mirror contract and publication surfaces

- **Files**: `docs/ito`, published-mirror config DTOs/defaults, mirror generation/publication code and tests, `schemas/ito-config.schema.json`, mirror documentation/workflows
- **Dependencies**: Task 3.1
- **Action**: After parity evidence is approved, remove `docs/ito`, mirror path configuration, generation/publication paths, tests, workflows, and current source-of-truth claims. Retain only clearly historical migration evidence where useful.
- **Verify**: `test ! -e docs/ito && test -z "$(rg -l 'published_mirror|published mirror path|generate.*docs/ito' ito-rs schemas .github docs .ito/wiki 2>/dev/null)"`
- **Done When**: The mirror and its configurable/publication contract are absent, tracked `.ito` remains complete, and configuration/schema tests cover obsolete values according to policy.
- **Requirements**: ito-authority-cutover:mirror-parity-before-retirement, published-ito-mirror:plain-checkout-visibility, published-ito-mirror:default-and-configurable-path, published-ito-mirror:generated-read-only-output, published-ito-mirror:main-publication-workflow, ito-config-crate:published-mirror-path
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

______________________________________________________________________

## Wave 4: Converge canonical sources and generated assets

- **Depends On**: Wave 3

### Task 4.1: Update specs, wiki, and project source guidance

- **Files**: `.ito/specs`, `.ito/wiki`, `.ito/project.md`, `.ito/user-prompts`, `AGENTS.md`, `README.md`, `docs/`
- **Dependencies**: None
- **Action**: Merge the accepted core-reset deltas into current truth as appropriate; make wiki config/index/status/log/topics source tracked `.ito`; update project and user guidance plus user-facing docs to the proposal-on-main, implementation-from-main, archive-on-main lifecycle; label historical/experimental surfaces explicitly.
- **Verify**: `ito validate --strict && make docs-site-check`
- **Done When**: Raw specs and current guidance agree on main authority, the wiki no longer indexes `docs/ito`, and no current workflow tells users to author in coordination storage or the mirror.
- **Requirements**: ito-authority-cutover:guidance-and-asset-convergence
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 4.2: Update canonical templates and the reduced install profile

- **Files**: `ito-rs/crates/ito-templates/assets/default`, `ito-rs/crates/ito-templates/assets/instructions`, canonical skill/command/agent manifests and install profiles
- **Dependencies**: Task 4.1
- **Action**: Align canonical templates with main authority and the seven lifecycle skills defined by `031-05`; ensure gated experimental surfaces follow `031-03`; ensure removed tmux and obsolete coordination/mirror workflows cannot be emitted by default.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Canonical sources contain one consistent lifecycle, the default profile contains exactly the approved seven skills, and negative asset tests cover tmux and retired outputs.
- **Requirements**: ito-authority-cutover:guidance-and-asset-convergence
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 4.3: Regenerate schema and every managed harness surface

- **Files**: `schemas/ito-config.schema.json`, `.claude`, `.codex`, `.github`, `.opencode`, `.pi`, other Ito-managed generated assets
- **Dependencies**: Task 4.2
- **Action**: Run canonical schema generation and supported-harness regeneration, remove orphaned managed outputs, then run both generators a second time. Do not hand-edit generated files to conceal source drift.
- **Verify**: `make config-schema && ito init --upgrade --tools all && make config-schema-check && git diff --check`
- **Done When**: Generated outputs reflect canonical sources, the second pass adds no diff, obsolete assets are absent, and external coordination hashes still match the original snapshot.
- **Requirements**: ito-authority-cutover:guidance-and-asset-convergence
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

______________________________________________________________________

## Wave 5: Verify default, experimental, docs, and release lanes

- **Depends On**: Wave 4

### Task 5.1: Run the standard default-feature quality lane

- **Files**: Rust workspace, default `ito` binary, standard CI logs
- **Dependencies**: None
- **Action**: From a clean build state, run the integrated standard-feature formatting, lint, build, test, coverage, migration-instruction, template, and repository checks without enabling backend or coordination runtime features.
- **Verify**: `make check && cargo test --workspace`
- **Done When**: The standard lane is green and the built/default release surface excludes experimental backend/coordination runtime and all removed assets while retaining required migration recovery.
- **Requirements**: ito-authority-cutover:dual-lane-release-verification
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 5.2: Run the explicit all-features quality lane independently

- **Files**: Rust workspace, all-features CI logs
- **Dependencies**: Task 5.1
- **Action**: Run formatting, lint, build, and tests with all Cargo features enabled in a separate clean lane; prove experimental backend and coordination code remains buildable without altering standard defaults or project authority.
- **Verify**: `cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace --all-features`
- **Done When**: The all-features lane is green, experimental behavior is opt-in, and standard/default artifact evidence is unchanged.
- **Requirements**: ito-authority-cutover:dual-lane-release-verification
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 5.3: Verify generated artifacts, documentation, and release planning

- **Files**: schema/assets/docs check logs, cargo-dist/release-plz plan output, installer/checksum smoke evidence
- **Dependencies**: Task 5.2
- **Action**: Re-run schema, template, docs-site, installer/checksum, and non-publishing release-plan checks. Confirm the standard release plan packages only the approved default binary and no command pushes, tags, publishes, or mutates the external coordination checkout.
- **Verify**: `make config-schema-check && make docs-site-check && dist plan --output-format=json`
- **Done When**: All generated-artifact and docs checks pass, release planning succeeds without publication, default artifact contents are correct, and source-after hashes still match source-before hashes.
- **Requirements**: ito-authority-cutover:external-state-preservation, ito-authority-cutover:dual-lane-release-verification
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

______________________________________________________________________

## Wave 6: Independent reviews and requirement audit

- **Depends On**: Wave 5

### Task 6.1: Complete an independent Rust, config, template, and release review

- **Files**: Full diff, `.ito/changes/031-06_migrate-ito-authority-and-release/reviews/rust-release-review.md`
- **Dependencies**: None
- **Action**: Have a reviewer independent of implementation assess Rust/config behavior, feature boundaries, schema/template generation, tests, CI, and release contents. Record every finding and resolution.
- **Verify**: `test -s .ito/changes/031-06_migrate-ito-authority-and-release/reviews/rust-release-review.md && test -z "$(rg -n 'blocking.*unresolved|unresolved.*blocking' .ito/changes/031-06_migrate-ito-authority-and-release/reviews/rust-release-review.md)"`
- **Done When**: The first review is recorded and has no unresolved blocking finding.
- **Requirements**: ito-authority-cutover:dual-lane-release-verification
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 6.2: Complete an independent migration, parity, and documentation review

- **Files**: Full diff and evidence manifests, `.ito/changes/031-06_migrate-ito-authority-and-release/reviews/migration-requirements-review.md`
- **Dependencies**: Task 6.1
- **Action**: Have a second independent reviewer assess source preservation, collision handling, hashes, mirror parity, tracked-main authority, wiki/docs correctness, rollback, and separation from `031-01` through `031-05`. Record every finding and resolution.
- **Verify**: `test -s .ito/changes/031-06_migrate-ito-authority-and-release/reviews/migration-requirements-review.md && test -z "$(rg -n 'blocking.*unresolved|unresolved.*blocking' .ito/changes/031-06_migrate-ito-authority-and-release/reviews/migration-requirements-review.md)"`
- **Done When**: The second review is independently recorded and has no unresolved blocking finding.
- **Requirements**: ito-authority-cutover:external-state-preservation, ito-authority-cutover:mirror-parity-before-retirement, ito-authority-cutover:dual-lane-release-verification
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

### Task 6.3: Audit every requirement and produce final readiness evidence

- **Files**: `.ito/changes/031-06_migrate-ito-authority-and-release/requirement-audit.md`, all verification evidence, final Git diff
- **Dependencies**: Task 6.2
- **Action**: Map every requirement ID and scenario to implementation evidence and passing commands; rerun strict validation, traceability, repo checks, clean-generation checks, external source hash verification, and diff review. Do not tag, publish, push, archive, or delete the retained source as part of this task.
- **Verify**: `ito validate 031-06_migrate-ito-authority-and-release --strict && ito trace 031-06_migrate-ito-authority-and-release && make check && git diff --check && sha256sum --check .ito/changes/031-06_migrate-ito-authority-and-release/evidence/source-files.sha256`
- **Done When**: Every requirement has passing evidence, both reviews are resolved, both feature lanes and release planning are green, generated outputs are idempotent, the external source is unchanged, and the change is ready for reviewed main integration rather than locally released.
- **Requirements**: ito-authority-cutover:dependency-gated-cutover, ito-authority-cutover:external-state-preservation, ito-authority-cutover:tracked-main-authority, ito-authority-cutover:mirror-parity-before-retirement, ito-authority-cutover:guidance-and-asset-convergence, ito-authority-cutover:dual-lane-release-verification, published-ito-mirror:plain-checkout-visibility, published-ito-mirror:default-and-configurable-path, published-ito-mirror:generated-read-only-output, published-ito-mirror:main-publication-workflow, ito-config-crate:published-mirror-path
- **Updated At**: 2026-07-13
- **Status**: [ ] pending

______________________________________________________________________

## Wave Guidelines

- Finish and verify each wave before starting the next.
- Keep exactly one task in progress; use `ito tasks start` and `ito tasks complete`.
- Any stop condition leaves the external source and `docs/ito` intact until the discrepancy is resolved.
- The final handoff is a reviewed cutover PR; release/tag/archive and external-store cleanup are separate, later actions.
<!-- ITO:END -->
