# Change: Homebrew Release

## Why

Users on macOS expect to install CLI tools via Homebrew (`brew install ito`). Currently Ito can only be installed via GitHub Releases or building from source. Adding Homebrew support provides a familiar, auto-updating installation experience for the largest segment of macOS developers.

The golden path for CLI distribution via Homebrew is a personal/org tap (not homebrew-core), with automated formula updates on each release. This approach:
- Avoids homebrew-core's strict acceptance criteria for "niche or self-submitted" CLIs
- Provides full control over the formula without PR approval cycles
- Enables automatic updates within seconds of `git push --tags`

## What Changes

### 1. Create Homebrew Tap Repository

Create `withakay/homebrew-ito` tap repository following Homebrew conventions:
- Repository name MUST be `homebrew-ito` (the `homebrew-` prefix enables short-form tap syntax)
- Users will tap with: `brew tap withakay/ito` (the `homebrew-` prefix is implicit)
- Scaffold using `brew tap-new withakay/homebrew-ito` for correct structure

### 2. Create Ito Formula

Formula at `Formula/ito.rb` referencing GitHub release tarballs:
- GitHub auto-generates tarballs at predictable URLs: `github.com/withakay/ito/archive/refs/tags/vX.Y.Z.tar.gz`
- Formula specifies URL + SHA256 checksum for reproducibility
- Add `livecheck` block for version discovery
- Add minimal test (e.g., `assert_match version.to_s, shell_output("#{bin}/ito --version")`)
- Target Ruby 3.x via `depends_on "ruby@3"` (NOT `uses_from_macos "ruby"` which is ancient 2.6.x)

### 3. Automate Formula Updates

GitHub Actions workflow in the **main ito repo** (not the tap) to update formula on release:
- Triggered on tag push or GitHub release creation
- Downloads tarball, computes SHA256, updates formula
- Commits directly to tap repo's main branch (no PR ceremony for self-owned tap)
- Requires PAT with `Content: Write` permission on tap repo

### 4. Update Documentation

- Add Homebrew installation to README
- Document two-command install process:
  ```bash
  brew tap withakay/ito
  brew install ito
  ```

## Capabilities

### New Capabilities

- `homebrew-formula`: Homebrew formula definition for ito with:
  - macOS x86_64 and arm64 architecture support
  - Automatic version/SHA256 updates on release
  - livecheck for version discovery
  - Installation test

### Modified Capabilities

<!-- None - this is a new distribution channel, not a change to existing spec behavior -->

## Impact

- **New repository**: Requires creating `withakay/homebrew-ito` tap repository
- **CI/CD**: Adds workflow to ito repo that updates tap on release
- **Secrets**: Requires `HOMEBREW_TAP_TOKEN` PAT secret in ito repo
- **Dependencies**: Relies on `005-03_ci-cross-platform-releases` for release artifacts
- **Documentation**: README and install docs need Homebrew instructions

## References

- [How to distribute scripts via Homebrew](https://justin.searls.co/posts/how-to-distribute-your-own-scripts-via-homebrew/) - Justin Searls' guide (main reference)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Homebrew Tap Guide](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)
- Example workflow: [searlsco/imsg update workflow](https://github.com/searlsco/imsg/blob/main/.github/workflows/update_homebrew_formula.yml)
