# Releasing Ito

This project uses [release-please](https://github.com/googleapis/release-please) to automate releases.

## How It Works

1. **Commits to `main`** are analyzed by release-please based on [Conventional Commits](https://www.conventionalcommits.org/)
2. **Release-please opens/updates a PR** with version bumps and changelog updates
3. **Merging the release PR** creates a `vX.Y.Z` tag
4. **The tag triggers** the release workflow which:
   - Builds binaries for all platforms (macOS x64/arm64, Linux x64/arm64, Windows x64)
   - Creates a draft GitHub release with artifacts
   - Publishes npm packages
5. **Publishing the release** triggers polish-release-notes (optional AI enhancement)
6. **Publishing the release** also triggers the Homebrew formula update

## Commit Message Format

Use conventional commits to control version bumps:

| Prefix | Version Bump | Example |
|--------|--------------|---------|
| `feat:` | Minor (0.X.0) | `feat: add new command` |
| `fix:` | Patch (0.0.X) | `fix: correct parsing error` |
| `feat!:` or `BREAKING CHANGE:` | Major (X.0.0) | `feat!: redesign API` |

Other prefixes (`docs:`, `chore:`, `refactor:`, `test:`, `ci:`) don't trigger releases but are included in the changelog.

## Manual Release Trigger

The release-please workflow can be triggered manually from the GitHub Actions UI to force a release PR creation.

### Using workflow_dispatch

1. Go to **Actions** → **Release Please** in the GitHub repository
2. Click **Run workflow**
3. Click **Run workflow** (no inputs required)

The workflow will:
1. Create an empty commit with a conventional commit message (`fix: trigger manual release`)
2. Run release-please to analyze all commits and create/update the release PR
3. Release-please will determine the version bump based on conventional commits in the history
4. If no other conventional commits exist, the default is a patch bump

**Editing the version**: Once the release PR is created, you can edit the version numbers in the PR files before merging if you need a different version.

This is useful when you need to:
- Force a release without waiting for the scheduled run
- Create a release on-demand
- Trigger release processing when there may be uncommitted conventional commits

### Emergency Manual Release

If you need to release without any automation:

```bash
# Update version in ito-rs/Cargo.toml
# Update CHANGELOG.md manually
git tag vX.Y.Z
git push origin vX.Y.Z
```

## Files Managed by Release-Please

- `ito-rs/Cargo.toml` - workspace version
- `ito-rs/CHANGELOG.md` - changelog
- `.release-please-manifest.json` - version tracking

## Troubleshooting

### Release PR not created
- Check that commits follow conventional commit format
- Verify the release-please workflow ran successfully

### Version mismatch error in release workflow
- The tag version must match the version in `ito-rs/Cargo.toml`
- Release-please should keep these in sync automatically
