# Release Process

This repo ships Ito via:

- GitHub Releases (cross-platform archives + installers)
- Homebrew formula updates (in a separate tap repo)

The release pipeline is CI-driven and designed to match the "automated rust releases" flow:

- release-plz: version/changelog PR + crates.io publishing + version tag
- cargo-dist: build/package binaries + create GitHub Release + upload assets

## Workflows Involved

### 1) Release PR + publishing (release-plz)

Workflow: `/.github/workflows/release-plz.yml`

- Trigger: push to `main`
- What it does:
  - Maintains a release PR (version bumps + `CHANGELOG.md` updates)
  - After that PR is merged, publishes crates to crates.io
  - Creates a version tag `vX.Y.Z`

### 2) Build + GitHub Release assets (cargo-dist)

Workflow: `/.github/workflows/release.yml`

- Trigger: tag push (version-like tags such as `vX.Y.Z`)
- What it does:
  - Runs `dist` to build and package artifacts
  - Creates/updates the GitHub Release for the tag
  - Uploads artifacts including:
    - `ito-cli-<target>.tar.xz` / `ito-cli-<target>.zip`
    - `ito-cli-installer.sh` (curl | sh)
    - `ito-cli-installer.ps1` (irm | iex)
    - checksums (`*.sha256`, `sha256.sum`)
- Binary naming:
  - The installed executable is `ito` (or `ito.exe` on Windows)

### 3) Update Homebrew formula

Workflow: `/.github/workflows/homebrew.yml`

- Trigger: GitHub release event `published`
- What it does:
  - Calls the reusable workflow `/.github/workflows/update-homebrew.yml`
  - Downloads cargo-dist artifacts from the release and computes sha256
  - Updates `withakay/homebrew-ito` formula `Formula/ito.rb` and pushes to `main`

### 4) Polish release notes

Workflow: `/.github/workflows/polish-release-notes.yml`

- Trigger: GitHub release event `published`
- What it does:
  - Rewrites cargo-dist/release notes into developer-facing release notes
  - Updates the GitHub Release title/body

## Step-by-Step Release Checklist

### 0) Pre-flight

- Ensure CI is green on `main`.
- Locally (optional but recommended):
  - `make check`
  - `make test`

### 1) Cut and merge a release PR

- Merge normal feature/fix PRs into `main`.
- The release-plz workflow will open/update a release PR.
- Review and merge that PR.

### 2) Wait for automation

After the release PR merge:

- release-plz publishes to crates.io and creates tag `vX.Y.Z`
- cargo-dist builds artifacts and publishes a GitHub Release for `vX.Y.Z`
- Homebrew and release-note polishing run off the published release

### 3) Post-release checks

- GitHub Release contains expected cargo-dist assets and installers
- Homebrew tap `withakay/homebrew-ito` has an updated `Formula/ito.rb`

## Required Secrets / Credentials

Repository secrets used by CI:

- `RELEASE_PLZ_TOKEN`: PAT used by release-plz to create tags/PRs without blocking tag-triggered workflows
- `CARGO_REGISTRY_TOKEN`: crates.io publishing token
- `HOMEBREW_TAP_TOKEN`: pushes formula updates to `withakay/homebrew-ito`
- `CLAUDE_CODE_OAUTH_TOKEN`: optional; polishes release notes

`RELEASE_PLEASE_TOKEN` exists but is unused (Release Please is not part of this pipeline).
