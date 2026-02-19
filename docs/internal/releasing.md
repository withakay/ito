# Releasing Ito

This project uses:

- [release-plz](https://release-plz.dev/) for release PRs, versioning, and crates.io publishing
- [cargo-dist](https://axodotdev.github.io/cargo-dist/) for GitHub Releases and cross-platform binaries/installers

## How It Works

1. **Commits to `main`** are analyzed by release-plz (conventional commits)
2. **release-plz opens/updates a release PR** with version bumps and `CHANGELOG.md` updates (via git-cliff)
3. **Merging the release PR** causes release-plz to publish crates to crates.io and create a `vX.Y.Z` tag
4. **The tag triggers cargo-dist** to:
    - Build cross-platform binaries and installers
    - Create/update the GitHub Release and upload assets
5. **Publishing the GitHub release** triggers:
    - release note polishing (optional)
    - Homebrew formula update
    - GitHub Pages docs deploy (builds `site/` and publishes it)

## GitHub Pages Prerequisite

The repository must have GitHub Pages configured to deploy from GitHub Actions:

- Repo Settings -> Pages -> Source = "GitHub Actions"

## Commit Message Format

Use conventional commits to control version bumps:

| Prefix | Version Bump | Example |
|--------|--------------|---------|
| `feat:` | Minor (0.X.0) | `feat: add new command` |
| `fix:` | Patch (0.0.X) | `fix: correct parsing error` |
| `feat!:` or `BREAKING CHANGE:` | Major (X.0.0) | `feat!: redesign API` |

Other prefixes (`docs:`, `chore:`, `refactor:`, `test:`, `ci:`) don't trigger releases but are included in the changelog.

## Manual Release Trigger

If you need to force (re)generation of the release PR, you can run release-plz locally:

```bash
make release
```

This runs `release-plz release-pr` against the repo.

### Emergency Manual Release

If you need to release without any automation:

```bash
# Update version in Cargo.toml and update CHANGELOG.md
git tag vX.Y.Z
git push origin vX.Y.Z
```

## Files Managed by Release Automation

- `Cargo.toml` - workspace version
- `CHANGELOG.md` - changelog
- `release-plz.toml` - release-plz configuration
- `cliff.toml` - git-cliff configuration
- `dist-workspace.toml` - cargo-dist configuration

## Troubleshooting

### Release PR not created
- Check that commits follow conventional commit format
- Verify the `Release-plz` workflow ran successfully

### Version mismatch error in release workflow
- The tag version must match the version in `Cargo.toml`
- release-plz should keep these in sync automatically
