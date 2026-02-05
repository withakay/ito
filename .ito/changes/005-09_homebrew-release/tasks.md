# Tasks: Homebrew Release

## Prerequisites

- [x] Verify 005-03_ci-cross-platform-releases produces macOS artifacts (both arm64 and x86_64)
- [x] Ensure GitHub releases include source tarballs (auto-generated at `archive/refs/tags/vX.Y.Z.tar.gz`)

## Tap Repository Setup

- [x] Create `withakay/homebrew-ito` repository on GitHub (must be named `homebrew-ito` for short-form tap)
- [x] Run `brew tap-new withakay/homebrew-ito` locally to generate scaffold
- [x] Push scaffold to GitHub: `cd /opt/homebrew/Library/Taps/withakay/homebrew-ito && git push`
- [x] Add README.md with installation instructions

## Formula Implementation

Create `Formula/ito.rb`:

- [x] Use `brew create <tarball-url> --tap withakay/homebrew-ito --set-name ito` as starting point
- [x] Reference source tarball URL: `https://github.com/withakay/ito/archive/refs/tags/vX.Y.Z.tar.gz`
- [x] Switch stable formula to use GitHub Release archives (not source builds)
- [x] Include per-arch URLs + SHA256 for macOS arm64/x86_64
- [x] Add `livecheck` block for automatic version discovery
- [x] Add test block with HEAD/stable build detection
- [x] Run `brew style withakay/ito` to verify formula syntax
- [x] Test local stable install: `brew tap withakay/ito && brew install ito` works
- [x] Verify `brew test ito` passes

## Release Automation

Create `.github/workflows/update-homebrew.yml` in **main ito repo**:

- [x] Create GitHub PAT with `Content: Write` permission on `withakay/homebrew-ito`
- [x] Add PAT as `HOMEBREW_TAP_TOKEN` secret in ito repository
- [x] Create workflow triggered on release publish / Release workflow completion
- [x] Support manual runs (`workflow_dispatch`)
- [x] Support tag-push trigger (`v*`)
- [x] Workflow steps: checkout tap, download release archives, compute SHA256, rewrite formula, commit to main
- [x] Configure git user for commits (use GitHub Actions bot or custom bot account)
- [ ] Test workflow with a test release

Reference implementation: [searlsco/imsg workflow](https://github.com/searlsco/imsg/blob/main/.github/workflows/update_homebrew_formula.yml)

## Documentation

- [x] Update main README.md with Homebrew installation instructions:
  ```bash
  brew tap withakay/ito
  brew install ito
  ```
- [x] Add Homebrew section to any existing install docs

## Validation

- [x] Test `brew tap withakay/ito && brew install ito` on Apple Silicon
- [ ] Test `brew tap withakay/ito && brew install ito` on Intel Mac (or CI)
- [ ] Test `brew update && brew upgrade ito` after a version bump
- [x] Verify `brew test ito` passes
