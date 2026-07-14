## Context

The repository already has a tag-triggered release pipeline that validates `vX.Y.Z` tags against the `ito-cli` crate version, builds multi-platform release artifacts, drafts GitHub releases, and publishes npm packages.

This change introduces `release-please` as the preferred mechanism to:

- decide the next release version
- update version/changelog files via a release PR
- create the `vX.Y.Z` tag and GitHub Release on merge

## Goals / Non-Goals

**Goals:**

- A maintainer can cut a release by merging a `release-please` PR.
- The `vX.Y.Z` tag created by `release-please` triggers the existing artifact build/publish workflow.
- Release version and tag stay consistent (no manual tagging).
- Release documentation clearly describes the new flow.

**Non-Goals:**

- Replacing the existing artifact build/publish pipeline.
- Changing supported target matrices or packaging formats.
- Fully redesigning how changelog entries are authored (only integrating with the chosen `release-please` strategy).

## Decisions

- **Keep tag-triggered build/publish**: `release-please` is responsible for versioning, tagging, and release note generation; the existing tag-triggered workflow remains responsible for building and publishing artifacts.
- **Release version format**: release PRs must set the published version to plain `X.Y.Z` so the existing tag validation regex (`vX.Y.Z`) continues to work.
- **Release notes polishing**: keep the existing post-publish release-notes “polish” workflow, but align triggers so it runs consistently with how `release-please` creates/releases GitHub Releases.

## Risks / Trade-offs

- **Risk**: `release-please` release-type/config does not correctly update Rust workspace versions. → **Mitigation**: validate config against the repo structure (workspace + `ito-cli`) and add a CI check that release PRs keep tag/version expectations.
- **Risk**: GitHub Release event triggers change (draft vs published). → **Mitigation**: explicitly decide whether releases are created as draft or published and update the polish workflow trigger accordingly.
