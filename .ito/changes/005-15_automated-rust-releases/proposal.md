## Why

Our release automation keeps stalling because most examples assume a simple repo layout (single Cargo workspace rooted at `./Cargo.toml`, no worktrees, minimal CI). In this repo, the Rust workspace lives in `ito-rs/` (no root `Cargo.toml`), and we use additional tooling/workflows that make "drop-in" configs fail in subtle ways.

We want a release system that is boring, repeatable, and aligned with the proven flow described in https://blog.orhun.dev/automated-rust-releases (release-plz + git-cliff + cargo-dist), but adapted to our repo structure so we stop burning time on trial-and-error.

## What Changes

- Adopt an Orhun-style automated release pipeline, adapted for this repo's layout where Rust crates live under `ito-rs/`.
- Add a root `Cargo.toml` workspace (virtual workspace) that references crates under `ito-rs/` to reduce tool friction.
- Standardize the end-to-end flow:
  - release PR creation/update (version + changelog)
  - tag creation (`vX.Y.Z`) and crates.io publishing
  - artifact build + GitHub Release asset upload (and downstream Homebrew update)
- Add/align supporting release config (git-cliff config and cargo-dist config) and wire them into CI.
- Remove remaining Release Please references (docs/targets) so the repo reflects the actual release tooling.

## Capabilities

### New Capabilities

- `release-automation`: Define the required CI behavior for release PRs, tag-driven publishing, and artifact generation in a monorepo where the Rust workspace is not at the git root.

### Modified Capabilities

- `release-artifacts`: Clarify/extend the release pipeline expectations to ensure the automated flow produces the required cross-platform assets and checksums.

## Impact

- CI/workflows: `.github/workflows/release-plz.yml`, `.github/workflows/release.yml`, `.github/workflows/update-homebrew.yml`, and potentially adding cargo-dist-generated workflows.
- Release config/docs: `release-plz.toml`, new `cliff.toml`, updates to `RELEASE.md`.
- Rust workspace metadata: `ito-rs/Cargo.toml` (cargo-dist configuration; possibly minor metadata changes).
- Repo layout: may introduce a root `Cargo.toml` workspace to simplify tooling; may update developer commands/docs accordingly.
- Secrets: repository already has `CARGO_REGISTRY_TOKEN`, `HOMEBREW_TAP_TOKEN`, `RELEASE_PLZ_TOKEN` configured; `RELEASE_PLEASE_TOKEN` also exists but should become unused as Release Please references are removed.
