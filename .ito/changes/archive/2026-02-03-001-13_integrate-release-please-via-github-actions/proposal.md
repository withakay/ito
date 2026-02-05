## Why

Ito releases currently require maintainers to coordinate version bumps, tags, and release notes across multiple steps, which is easy to get wrong and hard to standardize. Integrating `release-please` via GitHub Actions makes the release flow repeatable and lowers the operational overhead for cutting new Ito versions.

## What Changes

- Add a `release-please` GitHub Actions workflow that opens/updates a release PR against `main`.
- Add `release-please` configuration for this repo so release PRs:
  - bump the Rust workspace/crate versions used for releases
  - update the changelog/release notes source of truth
- On merge of the release PR, `release-please` creates the `vX.Y.Z` tag and GitHub Release, which triggers the existing tag-based release pipeline to build and publish artifacts.
- Update maintainer-facing docs to make `release-please` the preferred release mechanism.

## Capabilities

### New Capabilities

- `release-please-releases`: Automate Ito releases via `release-please` (release PR, tag creation, GitHub Release creation) while keeping the existing artifact build/publish pipeline.

### Modified Capabilities

<!-- None. -->

## Impact

- **CI/Workflows**: Adds a new workflow under `.github/workflows/` and may adjust existing release-related workflows to align triggers/permissions.
- **Versioning**: Release commits will use clean `X.Y.Z` versions (no local suffix) to match `vX.Y.Z` tags expected by the current release pipeline.
- **Docs**: Maintainer release instructions will shift from manual steps to “merge the release PR”.
