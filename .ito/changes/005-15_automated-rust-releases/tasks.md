# Tasks for: 005-15_automated-rust-releases

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 005-15_automated-rust-releases
ito tasks next 005-15_automated-rust-releases
ito tasks start 005-15_automated-rust-releases 1.1
ito tasks complete 005-15_automated-rust-releases 1.1
ito tasks shelve 005-15_automated-rust-releases 1.1
ito tasks unshelve 005-15_automated-rust-releases 1.1
ito tasks show 005-15_automated-rust-releases
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Inventory current release pipeline and existing Ito changes

- **Files**: `RELEASE.md`, `.github/workflows/release-plz.yml`, `.github/workflows/release.yml`, `release-plz.toml`, `.ito/changes/005-14_enable-crates-io-publishing/proposal.md`
- **Dependencies**: None
- **Action**:
  - Document the current source of truth for: versioning, changelog, tag creation, crates.io publish, artifact building, Homebrew updates.
  - Identify which parts already match the Orhun flow and where assumptions break due to the `ito-rs/` workspace layout.
- **Verify**: `ito show 005-14_enable-crates-io-publishing`
- **Done When**: A short written inventory exists in the design/proposal (or linked notes) and open gaps are listed.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 1.2: Implement root `Cargo.toml` virtual workspace

- **Files**: `ito-rs/Cargo.toml`, `Cargo.toml` (repo root)
- **Dependencies**: Task 1.1
- **Action**:
  - Create root `Cargo.toml` with a virtual workspace that references `ito-rs/crates/*` members.
  - Move shared workspace settings (`workspace.package`, `workspace.dependencies`, resolver) from `ito-rs/Cargo.toml` into the root workspace.
  - Decide what to do with `ito-rs/Cargo.toml` to avoid two sources of truth (remove it or replace it with an explicit shim).
- **Verify**: `cargo metadata` succeeds from repo root and workspace builds/tests can be invoked with root as the canonical workspace.
- **Done When**: Root `Cargo.toml` is the single authoritative workspace entrypoint.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 1.3: Define cargo-dist + git-cliff integration plan

- **Files**: `ito-rs/Cargo.toml`, (new) `cliff.toml`, `.github/workflows/release.yml`
- **Dependencies**: Task 1.2
- **Action**:
  - Specify where changelog config and changelog output live and how release-plz will update them.
  - Specify whether cargo-dist replaces `/.github/workflows/release.yml` or is integrated into it.
- **Verify**: Plan is captured in `/.ito/changes/005-15_automated-rust-releases/design.md` with explicit workflow triggers.
- **Done When**: The intended workflow graph is unambiguous.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Add git-cliff configuration and wire it into release-plz

- **Files**: (new) `cliff.toml`, `release-plz.toml`
- **Dependencies**: None
- **Action**:
  - Add `cliff.toml` and configure release-plz to use it for changelog generation/updates.
  - Ensure paths work when the workspace root is `ito-rs/` and changelog is at repo root.
- **Verify**: `release-plz update --manifest-path ito-rs/Cargo.toml` (local dry-run) and/or CI run.
- **Done When**: release-plz produces deterministic changelog edits.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 2.2: Ensure ito-cli installs as `ito`

- **Files**: `ito-rs/crates/ito-cli/Cargo.toml`, `ito-rs/crates/ito-cli/src/main.rs`, `.github/workflows/release.yml` (or cargo-dist workflow)
- **Dependencies**: None
- **Action**:
  - Ensure the `ito-cli` crate produces a binary named `ito`.
  - Ensure packaging/installation (cargo-dist, Homebrew, shell installer) installs the executable as `ito` (or `ito.exe` on Windows).
- **Verify**: `cargo build -p ito-cli --bin ito --release` and `cargo dist plan` (once integrated).
- **Done When**: All supported installation paths yield an executable named `ito`.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 2.3: Integrate cargo-dist for packaging and GitHub release assets

- **Files**: `ito-rs/Cargo.toml`, `.github/workflows/release.yml` (or new cargo-dist workflows)
- **Dependencies**: Task 2.1
- **Action**:
  - Add cargo-dist metadata to the Rust workspace.
  - Generate or hand-integrate workflows so tag `vX.Y.Z` builds and uploads artifacts + checksums.
  - Keep Homebrew update working (either by preserving `update-homebrew.yml` or integrating equivalent outputs).
- **Verify**: `cargo dist plan` and a CI run on a test tag.
- **Done When**: Artifacts are produced for the supported target matrix and attached to a release.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

### Task 2.4: Update release documentation and remove Release Please references

- **Files**: `RELEASE.md`, `Makefile`
- **Dependencies**: Task 2.3
- **Action**:
  - Update `RELEASE.md` to describe the release-plz + cargo-dist flow.
  - Ensure `make release` and referenced workflows match reality.
- **Verify**: Manual doc review.
- **Done When**: A maintainer can follow `RELEASE.md` end-to-end without encountering missing files/tools.
- **Updated At**: 2026-02-10
- **Status**: [x] complete

______________________________________________________________________

## Wave 3 (Checkpoint)

- **Depends On**: Wave 2

### Task 3.1: Validate end-to-end release flow in a branch

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**: `.github/workflows/release-plz.yml`, `.github/workflows/release.yml`, `release-plz.toml`, `ito-rs/Cargo.toml`, `RELEASE.md`
- **Dependencies**: None
- **Action**:
  - Confirm the pipeline works in practice: release PR created, merge produces tags, canonical tag triggers artifact upload, and Homebrew update runs.
- **Done When**: Maintainer confirms the end-to-end run is acceptable.
- **Updated At**: 2026-02-10
- **Status**: [ ] in-progress

______________________________________________________________________
