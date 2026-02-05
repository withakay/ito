# Release Process

This repo ships Ito via:

- GitHub Releases (binary archives + checksums)
- Homebrew formula updates (in a separate tap repo)

The process is largely driven by GitHub Actions workflows under `.github/workflows/`.

## Workflows Involved

### 1) `Release Please` (`.github/workflows/release-please.yml`)

Trigger:

- Successful completion of the `CI` workflow on `main` (`workflow_run`)
- Manual run (`workflow_dispatch`)

What it does:

- Runs `googleapis/release-please-action@v4` using `release-please-config.json` and `.release-please-manifest.json`.
- Maintains `CHANGELOG.md` and version bumps for the `ito-rs/` package (release type: Rust).

Notes:

- This workflow manages the *version/changelog PR* lifecycle.
- The current manifest version is stored in `.release-please-manifest.json` (example: `0.20.8`).

### 2) `Release` (`.github/workflows/release.yml`)

Triggers:

- GitHub release event: `published`
- Manual run (`workflow_dispatch`) with an explicit `tag` input

What it does:

1. Resolves the tag (`vX.Y.Z`) from the release event (or input).
2. Validates the git tag matches the Rust crate version:
   - Compares `vX.Y.Z` (tag) to the `ito-cli` crate version from `cargo metadata` in `ito-rs/Cargo.toml`.
3. Builds release binaries for:
   - macOS x86_64 (`x86_64-apple-darwin`)
   - macOS arm64 (`aarch64-apple-darwin`)
   - Linux x86_64 (`x86_64-unknown-linux-gnu`)
   - Linux arm64 (`aarch64-unknown-linux-gnu`, built via `cross`)
   - Windows x86_64 (`x86_64-pc-windows-msvc`)
4. Packages artifacts:
   - `ito-vX.Y.Z-<target>.tar.gz` (macOS/Linux)
   - `ito-vX.Y.Z-<target>.zip` (Windows)
   - `ito-vX.Y.Z-<target>.sha256` (per-asset checksums)
   - `sha256sums.txt` (concatenation of the per-asset `.sha256` files)
5. Uploads the packaged artifacts to the existing GitHub release for the tag.

Notes:

- The GitHub release itself is created by `Release Please` when the release PR is merged.

### 3) `Polish Release Notes` (`.github/workflows/polish-release-notes.yml`)

Trigger:

- GitHub release event: `published`

What it does:

- Fetches the current release body via `gh release view`.
- Uses `anthropics/claude-code-action@v1` to rewrite the changelog-style notes into structured release notes.
- Updates the release title and notes via `gh release edit`.

Notes:

- Only runs on the upstream repo (`withakay/ito`), not forks.
- Requires `CLAUDE_CODE_OAUTH_TOKEN`.

### 4) `Update Homebrew Formula` (`.github/workflows/update-homebrew.yml`)

Triggers:

- GitHub release event: `published`
- Tag push event: `v*`
- Manual run (`workflow_dispatch`) with an explicit `tag` input

What it does:

- Computes `sha256` checksums for the platform release archives (macOS + Linux):
  - `https://github.com/withakay/ito/releases/download/<TAG>/ito-<TAG>-x86_64-apple-darwin.tar.gz`
  - `https://github.com/withakay/ito/releases/download/<TAG>/ito-<TAG>-aarch64-apple-darwin.tar.gz`
  - `https://github.com/withakay/ito/releases/download/<TAG>/ito-<TAG>-x86_64-unknown-linux-gnu.tar.gz`
  - `https://github.com/withakay/ito/releases/download/<TAG>/ito-<TAG>-aarch64-unknown-linux-gnu.tar.gz`
- Checks out the tap repo `withakay/homebrew-ito`.
- Rewrites `Formula/ito.rb` with the new `version` + per-arch `url`/`sha256` blocks.
- Commits and pushes to the tap repo’s `main`.

Notes:

- Requires `HOMEBREW_TAP_TOKEN`.

## Step-by-Step Release Checklist

### 0) Pre-flight

- Make sure CI is green on `main`.
- Locally, run:
  - `make check` (runs `prek run --all-files`)
  - `make test`

### 1) Cut the version/changelog PR (Release Please)

- Merge normal feature/fix PRs into `main`.
- After `CI` completes successfully on `main`, the `Release Please` workflow will open or update a release PR.
- To manually (re)trigger it from your machine: `make release`.
- Review that PR (version bump + `CHANGELOG.md` updates) and merge it.

### 2) Merge the Release Please PR

Merging the release PR causes `Release Please` to:

- Create the version tag `vX.Y.Z`.
- Create/publish the corresponding GitHub release.

### 3) Wait for automation to finish

Publishing the GitHub release triggers:

- `Release` (builds + uploads archives/checksums)
- `Update Homebrew Formula` (bumps `withakay/homebrew-ito`)
- `Polish Release Notes` (if enabled/credentialed)

If `Release` fails at “Validate tag matches crate version”, fix the version mismatch and rerun.

### 4) Post-release checks

- Verify Homebrew tap:
  - `withakay/homebrew-ito` has the updated `Formula/ito.rb`
- Verify assets on GitHub release:
  - Archives + `.sha256` files + `sha256sums.txt` are present

## Required Secrets / Credentials

GitHub Actions needs these repository secrets for a full release:

- `CLAUDE_CODE_OAUTH_TOKEN`: polish release notes after publishing
- `HOMEBREW_TAP_TOKEN`: push updates to `withakay/homebrew-ito`

The workflows also use `secrets.GITHUB_TOKEN` for creating/editing GitHub releases.

## Versioning Notes

- Tags are `vX.Y.Z`.
- The release workflow enforces that the tag matches the `ito-cli` crate version.
