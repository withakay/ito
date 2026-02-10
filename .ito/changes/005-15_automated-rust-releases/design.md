## Context

- Current state:
  - Rust workspace root is `Cargo.toml` at the git root, with crates under `ito-rs/crates/`.
  - Release tooling is partially migrated from Release Please to release-plz, but docs and some assumptions are still out of sync.
  - Artifact publishing is handled by a custom `.github/workflows/release.yml` (not cargo-dist).
  - Repository secrets already configured: `CARGO_REGISTRY_TOKEN`, `HOMEBREW_TAP_TOKEN`, `RELEASE_PLZ_TOKEN`, and `RELEASE_PLEASE_TOKEN`.

### Current Pipeline Inventory

- Release PR + publish: `/.github/workflows/release-plz.yml`
  - Trigger: push to `main`
  - Jobs:
    - `release-plz-pr`: runs `release-plz` with `command: release-pr`
    - `release-plz-release`: runs `release-plz` with `command: release`
  - Key wiring:
    - Uses `release-plz/action@v0.5` with `manifest_path: Cargo.toml`
    - Uses `RELEASE_PLZ_TOKEN` as `GITHUB_TOKEN` so tag-triggered workflows run
    - Uses `CARGO_REGISTRY_TOKEN` for crates.io publish

- GitHub release assets: `/.github/workflows/release.yml`
  - Triggers: `release: published` or manual `workflow_dispatch` with `tag`
  - Validates tag version matches `ito-cli` crate version via `cargo metadata --manifest-path Cargo.toml`
  - Builds `ito-cli` binary as `ito` for a target matrix and uploads:
    - `ito-${TAG}-${target}.tar.gz`/`.zip`
    - per-target `.sha256` and aggregated `sha256sums.txt`
  - Notes: hard-codes paths under `target/...`.

- Homebrew updates: `/.github/workflows/update-homebrew.yml`
  - Triggered from `release.yml` via `workflow_call`
  - Downloads release assets by URL and computes sha256, then updates `withakay/homebrew-ito`

- Release note polishing: `/.github/workflows/polish-release-notes.yml`
  - Trigger: `release: published`
  - Uses Claude Code action and edits the GitHub Release body/title
- Desired state:
  - Replicate the "fully automated Rust releases" architecture (release-plz + git-cliff + cargo-dist) while keeping the repo layout and avoiding brittle path assumptions.

## Goals / Non-Goals

**Goals:**

- A single, end-to-end release flow that:
  - creates/updates a release PR (version + changelog)
  - publishes crates to crates.io
  - creates a `vX.Y.Z` tag
  - produces cross-platform binaries + checksums and attaches them to the GitHub Release
- Tooling MUST work when the Cargo workspace is not at repo root.
- Eliminate Release Please references so "how releases work" is unambiguous.

**Non-Goals:**

- Rewriting the product distribution surface (npm binary distribution, installer semantics) unless required by cargo-dist integration.
- Changing versioning strategy beyond what is necessary to make the pipeline deterministic.

## Decisions

- Decision: Use a root `Cargo.toml` virtual workspace that references crates under `ito-rs/`.
  - Rationale: This removes a class of "tool assumes repo-root Cargo.toml" failures and reduces per-tool special casing. It also makes integrations like cargo-dist and dependency tooling more straightforward.
  - Intended shape:
    - Add `Cargo.toml` at the git root with `[workspace]` members pointing at `ito-rs/crates/*`.
    - Move `[workspace.package]` and `[workspace.dependencies]` from `ito-rs/Cargo.toml` to the root workspace.
    - Keep crate directories under `ito-rs/crates/` (no physical moves).
    - Stop treating `ito-rs/Cargo.toml` as a workspace root (either remove it or replace it with a clear, non-authoritative shim).
  - Alternative considered:
    - Keep the workspace rooted at `ito-rs/Cargo.toml` and rely on `manifest_path` everywhere (rejected: too easy to regress).

- Decision: Use per-crate versions for crates.io publishing, while keeping a single canonical "Ito release" tag.
  - Rationale: Per-crate versions reduce coupling for libraries, but the product surface (GitHub release assets, Homebrew, installers, adapter distribution) benefits from a stable repo-wide tag namespace.
  - Tag scheme:
    - Canonical tag: `vX.Y.Z` (represents the end-user Ito release, aligned with the CLI version)
  - Notes:
    - Optional alias tags (e.g., `ito-cli-vX.Y.Z`) are explicitly not required; we only add them if a tool hard-requires them.
    - The release pipeline should avoid building/uploading artifacts twice; only the canonical `vX.Y.Z` tag should trigger artifact workflows.

- Decision: Align release responsibilities with the Orhun flow where practical.
  - release-plz owns: version management, changelog updates (via git-cliff), release PR, crates.io publish, and tag creation.
  - cargo-dist owns: building/packaging release artifacts and publishing them to GitHub Releases.
  - Existing custom workflows MAY remain temporarily if they cover extra steps (e.g., Homebrew) until cargo-dist integration is proven.

## Risks / Trade-offs

- CI coupling and flakiness: release automation spans multiple workflows and secrets; failures are noisy.
  - Mitigation: explicit verification steps, dry-run/branch testing, and a checklist in `RELEASE.md`.
- Changelog scope: root `CHANGELOG.md` currently mixes repo changes; release-plz/git-cliff might need clearer scoping.
  - Mitigation: decide whether the release changelog is for the `ito-cli` crate (recommended) and ensure config paths match.
- Worktree/bare layout edge cases: some tooling resolves repo roots incorrectly when paths are passed relative to subdirectories.
  - Mitigation: ensure release config lives at the git root and use `manifest_path` rather than `config` paths that cause repo-root confusion.

## Migration Plan

1. Decide/implement root `Cargo.toml` workspace strategy (or confirm we can keep `ito-rs/`).
2. Add git-cliff config and validate changelog output.
3. Add cargo-dist config; generate or integrate workflows.
4. Update release workflows to use GitHub-hosted runners and run consistently.
5. Decide whether to keep `update-homebrew.yml` or integrate Homebrew updates into the dist pipeline.
6. Update `RELEASE.md` and remove Release Please references.
7. Validate end-to-end in a branch (release PR created, tag created, artifacts uploaded).

## Integration Plan (Operational)

- Changelog
  - Source config: `cliff.toml` at repo root.
  - Output file: `CHANGELOG.md` at repo root.
  - Release PR responsibility: release-plz updates `CHANGELOG.md` using `cliff.toml`.

- Release PR + crates.io publishing + tags
  - release-plz runs from CI with `manifest_path: Cargo.toml`.
  - release-plz creates the canonical tag `vX.Y.Z`.

- GitHub Release + binary artifacts
  - cargo-dist runs on tag creation (`vX.Y.Z`) and owns:
    - building cross-platform archives + checksums
    - creating/updating the GitHub Release
    - attaching artifacts to the GitHub Release
  - Replace the current custom `/.github/workflows/release.yml` once cargo-dist is proven equivalent.

- Homebrew
  - Keep `/.github/workflows/update-homebrew.yml` initially.
  - Trigger it from the GitHub Release event (published) produced by cargo-dist.

- Runners
  - Use GitHub-hosted runners for all jobs (ubuntu/macos/windows) to match the reference setup and reduce self-hosted variance.

## Open Questions

- Do we want cargo-dist to create the GitHub Release itself, or keep release-plz creating the release and have cargo-dist only upload assets?
- Should we scope changelog generation to `ito-cli` changes only, or keep a repo-wide changelog?
- Do we need to support nested workspaces or multiple Rust workspaces in the future (would push us toward a root workspace)?
