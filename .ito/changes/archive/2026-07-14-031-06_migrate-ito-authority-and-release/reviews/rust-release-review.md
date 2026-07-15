# Independent Rust, Config, Template, and Release Review

Date: 2026-07-14

Reviewer: independent Rust/config/template/release pass

Comparison base: `813a8d0ac50d1c7b1ee5f592933f59037de60693` (`main`)

## Scope and verdict

I reviewed the complete cutover working tree against the base commit, with emphasis on the Rust feature boundary, default and experimental runtime behavior, migration recovery, main-first readiness, config/schema compatibility, installer and template convergence, the seven-skill contract, tmux retirement, cargo-dist contents, and both verification lanes. State-preservation hashes, mirror normalization, and rollback evidence are intentionally left to the separate Task 6.2 migration review.

The reviewed implementation is ready for that second independent review and the final requirement audit. The standard executable remains a web-enabled spec-driven lifecycle tool with iteration/Ralph/loop support; backend and coordination implementations are independent experiments; migration recovery remains available in the standard binary; and the release plan contains only the approved `ito-cli` binary.

- Blocking findings remaining: none.
- Non-blocking findings remaining: none.

## Findings and resolutions

### RR-01: Shipping lint initially failed on legacy-detector cleanup

Status: resolved.

The default shipping Clippy lane initially rejected an unused `has_non_empty_directory` binding in `legacy_coordination.rs`. The binding was removed after the detector was refined to distinguish authoritative `changes`/`specs` directories from runtime coordination directories. `make lint`, the full shipping check, and strict all-feature Clippy subsequently passed.

### RR-02: Apply/sync readiness tests initially conflated default and experimental behavior

Status: resolved.

The first all-feature run found that a test always expected the default binary's typed `feature_unavailable` response even when `coordination-branch` was compiled. Splitting the case preserved both contracts:

- the standard CLI returns the typed coordination feature error before fetch; and
- the experimental CLI returns a failed main-first readiness report before fetch.

The feature-specific helpers are gated consistently, both targeted test variants pass, and strict test-target Clippy passes. The experimental counterpart continues proving that a proposal absent from authority cannot trigger a coordination fetch first.

### RR-03: A coordination-only rollback test ran in the shipping lane

Status: resolved.

Shipping coverage initially ran `coordination_repair_failure_rolls_back_created_worktree`, even though the coordination repair path is compiled only with `coordination-branch`; the default implementation correctly returned a created worktree instead of the experimental repair error expected by the test. The test is now gated to the coordination feature. Default worktree tests pass without it, and the all-feature lane runs and passes it, preserving rollback coverage in the correct lane.

### RR-04: Workspace feature unification could misreport CLI capabilities

Status: resolved.

The first isolated `cargo test --workspace` exposed a real executable-boundary issue. Building `ito-backend` as another workspace member unified `ito-core/backend`, while `ito-cli/backend` remained disabled. Core-local feature observation therefore allowed the default CLI to progress past the feature preflight even though the executable's backend integration was not selected.

The resolution adds `preflight_config_with_capabilities` and makes `ito-cli::Runtime` report its own local Cargo features explicitly. Core-only callers retain `preflight_config` and its core-local feature set. Documentation now distinguishes these two meanings, and regression coverage supplies explicit disabled capabilities for both backend and coordination-worktree requests. This is a small dependency-injection boundary, not an additional runtime subsystem, and it prevents another workspace member from exposing experimental behavior through the shipping CLI. The exact workspace regression, standard tests, and all-feature tests now pass.

### RR-05: Concurrent feature builds contaminated an early snapshot run

Status: resolved as a verification-process issue; no product or snapshot change was required.

An early default feature-matrix run observed seven help snapshot mismatches because a concurrent all-feature build replaced the shared `target/debug/ito` after the matrix had built its default candidate. A fresh default build matched the committed snapshots; the snapshots were not accepted or changed. The matrix explicitly rebuilds the candidate for each feature case, and the final run was performed with the shared target isolated. It passed without `.snap.new` files or hook normalization.

## Review results by area

### Feature gates and standard shipping boundary

- The root workspace selects `ito-cli` as its default member while retaining experimental crates for explicit workspace builds.
- `ito-cli` defaults to `web`. `backend` and `coordination-branch` are independent additive features, and `experimental` is only their explicit aggregate.
- `ito-core` has no default features. All backend client/repository code and coordination Git/worktree/synchronization code are gated; feature-neutral capability detection and migration recovery remain compiled.
- Optional CLI dependencies and feature propagation match the boundary. Backend-only does not enable coordination, and coordination-only does not enable backend.
- Shipping Make/CI lanes enumerate the approved CLI/support packages and do not rely on all-feature unification. Experimental lanes are named separately and enable the workspace features explicitly.
- `ito-rs/tools/check_release_features.py` proves the manifest, dependency-tree, Homebrew, cargo-dist, and backend-container selections. Shared default dependencies (`rusqlite`, `sha2`, and `hex`) are reported accurately rather than claimed removed.

### Runtime errors and migration recovery

- Explicit commands or config requesting a compiled-out experiment fail through a structured feature error before repository, audit, link, fetch, or fallback mutation.
- Invalid configuration remains a configuration error rather than being disguised as feature unavailability.
- Legacy coordination evidence is inspected by feature-neutral code. Read-only diagnostics warn, ordinary mutations fail closed, and ambiguous evidence is not reconciled automatically.
- `ito agent instruction migrate-to-main` is compiled and renderable without either experiment, including when legacy config is invalid or cannot be decoded.
- The migration prompt is reversible and operator-driven: it snapshots manifests/hashes/link targets, stops on collisions or ambiguity, preserves source bytes, modes, and symlink strings, materializes on a branch from main, updates config/ignore state, validates in a fresh checkout, and leaves the external source untouched.
- Migration is an agent instruction, not an eighth lifecycle command or skill.

### Main-first proposal readiness

- Typed config supports `pull_request` and `direct_merge`, with pull request as the default.
- Prepare readiness resolves the configured authority target, optionally refreshes the exact tracked branch, captures one immutable integration OID, resolves the change from that tree, and validates the authoritative schema and apply-required artifacts from that same tree.
- Execute readiness proves the implementation checkout belongs to the same repository, is a change worktree rather than main/bare control, and descends from the captured integration commit.
- Git reads use literal/path-safe tree access, reject unsafe components and non-regular authoritative blobs, avoid replacement/lazy-fetch behavior, and fail safely for shallow or ambiguous history.
- Apply renders authoritative inputs from the captured OID. Live task/progress overlay is allowed only after execute readiness succeeds.
- Apply, tasks, Ralph/loop, worktree creation/setup, and orchestration consume the readiness gate; worktree creation rolls back branch, worktree, and generated config on later failure.

### Config and schema compatibility

- Project defaults disable backend and coordination and select embedded storage. Proposal integration defaults to pull request.
- Backend and coordination DTOs remain parseable and represented in the schema in standard builds, allowing actionable diagnosis of legacy config.
- The removed `tools.tmux.enabled` key is ignored with a migration warning and is not rewritten as a side effect.
- Canonical schema generation is stable; proposal mode is represented, and removed tmux/published-mirror surfaces are absent from the generated contract.

### Seven-skill contract, generated assets, and tmux retirement

The canonical Ito lifecycle inventory is exactly:

1. `ito`
2. `ito-proposal`
3. `ito-research`
4. `ito-apply`
5. `ito-review`
6. `ito-archive`
7. `ito-loop`

Canonical skills, command wrappers, manifests, and every supported generated harness converge on that inventory and order. Proposal guidance retains the research-handoff path. The migration instruction is reachable through the unified `ito` surface without adding a wrapper.

No current template, manifest, generated harness, init flag, viewer implementation, or default config distributes tmux integration. Upgrade cleanup recognizes historical `ito-tmux` and `tmux` managed assets by reviewed fingerprints/markers, removes only known managed content, does not follow symlink targets, and preserves modified or user-owned content with a manual-migration report.

### Release contents

`dist-workspace.toml` selects only `ito-cli`, disables implicit defaults, explicitly enables `web`, and does not enable all features. `dist plan --output-format=json` succeeded and contained one `ito-cli` release with the source/checksum artifacts, shell and PowerShell installers, Homebrew formula, and approved platform archives. It contained no backend service artifact or experimental feature selection. The separate backend container opts into `backend` explicitly and does not alter standard GitHub Release or Homebrew contents.

## Verification evidence

The final frozen-source matrix completed 8/8 successfully:

1. `make feature-matrix-check`
2. `make check`
3. `make check-experimental`
4. `cargo test --workspace`
5. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
6. `cargo test --workspace --all-features`
7. `make config-schema-check`
8. `make docs-site-check`

Experimental coverage reported 80.52% regions, 75.91% functions, and 80.66% lines. The final matrix produced no source edits, snapshot updates, or hook normalizations.

Additional independent checks passed during this review:

- `cargo fmt --all -- --check`
- `git diff --check 813a8d0ac50d1c7b1ee5f592933f59037de60693`
- `python3 ito-rs/tools/check_release_features.py`
- `dist plan --output-format=json`
- `ito validate 031-06_migrate-ito-authority-and-release --strict`
- focused config, legacy detector, migration instruction, default/all-feature capability, apply readiness, worktree readiness/rollback, init cleanup, tmux-removal, help, and template suites

No tag, push, publication, external coordination mutation, or commit was performed by this review.
