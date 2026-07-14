---
createdAt: '2026-04-29T11:07:01.673Z'
keywords: []
related: [development/release_workflow/release_workflow.md, development/release_workflow/build_and_coverage_guardrails.md, development/release_workflow/installer_release_assets.md, development/ito_workflow/context.md]
summary: Release-plz runs from the repository root with dirty publication disabled; Ito's tracked .ito authority must remain versioned and must never be removed from Git to satisfy release hygiene.
tags: []
title: Release-plz Guardrails
updatedAt: '2026-07-14T02:00:00.000Z'
---
## Reason
Record release-plz safety rules after the tracked-main authority cutover.

## Raw Concept
**Task:**
Keep release automation compatible with canonical `.ito` state on `main`.

**Changes:**
- Retired the historical rule that treated all Ito state paths as projected,
  ignored coordination symlinks.
- Established `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`,
  and `.ito/audit` as tracked release inputs in this repository.
- Preserved the existing dirty-tree and package-release constraints.

**Files:**
- `.ito/`
- `.gitignore`
- `release-plz.toml`
- `.github/workflows/release-plz.yml`
- `dist-workspace.toml`

**Flow:**
reviewed Ito change -> tracked `.ito` content integrates on main -> release-plz
checks a clean repository -> cargo-dist plans the standard `ito-cli` artifact.

## Narrative
### Structure
`release-plz.toml` remains at the repository root for repository discovery in
temporary clones. The canonical workflow artifacts consumed by release review
are tracked directly under `.ito`; `docs/ito` is not generated or published.

### Dependencies
Release automation depends on a GitHub App token, full-history checkout,
toolchain/bootstrap steps, Rust caching, `release-plz/action`, and a registry
token for an explicitly authorized publication.

### Highlights
Dirty publishing remains disabled, workspace changelog and dependency updates
remain enabled, and only `ito-cli` creates Git tags. Cargo-dist packages the
standard release feature set (`web` only); backend and coordination-branch are
separately tested experiments and are absent from release artifacts.

### Rules
- Never add the five canonical `.ito` authority roots to `.gitignore` in this
  repository.
- Never use `git rm --cached` to hide tracked Ito authority from release-plz.
  A dirty-tree failure must be resolved at its actual source.
- Do not recreate or publish `docs/ito`.
- Do not set `git_only = true` in `release-plz.toml`.
- Do not tag, push, or publish from verification or migration worktrees.

### Historical note
Before module `031`, a coordination-worktree layout projected Ito paths through
ignored symlinks and release guidance recommended untracking them. That rule is
retired for this repository and must not be used as current remediation.

## Facts
- **ito_authority_paths**: The five `.ito` authority roots are tracked release inputs on `main`. [project]
- **tracked_authority_remediation**: Never untrack canonical `.ito` content to make release-plz clean. [convention]
- **published_mirror_status**: `docs/ito` is retired and has no release publication step. [project]
- **standard_release_features**: Cargo-dist builds `ito-cli` with `default-features = false` and `features = ["web"]`. [project]
- **release_plz_config_location**: `release-plz.toml` remains at the Git repository root. [project]
- **release_plz_allow_dirty**: `allow_dirty` is false. [project]
- **release_plz_publish_allow_dirty**: `publish_allow_dirty` is false. [project]
- **release_plz_changelog_update**: Workspace changelog updates are enabled. [project]
- **release_plz_dependencies_update**: Workspace dependency updates are enabled. [project]
- **ito_cli_git_tags**: `ito-cli` is the only package with Git tags enabled. [project]
