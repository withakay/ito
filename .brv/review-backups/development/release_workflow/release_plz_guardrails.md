---
title: Release-plz Guardrails
summary: Release-plz runs from repo root with allow_dirty disabled, workspace changelog updates enabled, and the release workflow must keep .ito coordination paths gitignored while untracking any already tracked ignored files.
tags: []
related: [development/release_workflow/release_workflow.md]
keywords: []
createdAt: '2026-04-29T11:07:01.673Z'
updatedAt: '2026-04-29T11:07:01.673Z'
---
## Reason
Document release-plz configuration and coordination-branch gitignore rules

## Raw Concept
**Task:**
Document release-plz configuration and coordination-branch ignore behavior

**Changes:**
- Kept projected .ito coordination paths in .gitignore
- Recorded the remediation for tracked ignored files via git rm --cached
- Captured release-plz workspace and package settings

**Files:**
- .gitignore
- release-plz.toml
- .github/workflows/release-plz.yml

**Flow:**
release-plz checks repo state -> tracked ignored files under .ito/changes are untracked with git rm --cached -> local files remain on disk -> release-plz runs from repo root with allow_dirty disabled

**Timestamp:** 2026-04-29T11:06:52.649Z

**Author:** withakay

**Patterns:**
- `^.ito/(changes|specs|modules|workflows|audit)$` - Projected Ito coordination paths that must remain gitignored

## Narrative
### Structure
.gitignore explicitly lists the Ito coordination symlinks and local state paths, while release-plz.toml lives at the repository root to support repo discovery in temp clones. The GitHub Actions workflow runs release-plz on main with separate release and release-pr jobs.

### Dependencies
The release workflow depends on a GitHub App token, checkout with fetch-depth 0, build-essential installation, mise toolchain setup, Rust cache, and release-plz/action@v0.5. release-plz also depends on CARGO_REGISTRY_TOKEN for publishing.

### Highlights
The configuration enables changelog and dependency updates, disables dirty publishes, disables semver checks, and enables git tags only for ito-cli. Coordination-branch mode keeps .ito/changes unignored locally but still tracked as gitignored so release-plz dirty checks do not surface projected files.

### Rules
In coordination-branch mode, .ito/changes, .ito/specs, .ito/modules, .ito/workflows, and .ito/audit are projected coordination paths and should remain gitignored. If release-plz reports tracked ignored files under .ito/changes, fix it by removing those projected files from Git tracking with git rm --cached while keeping local files on disk; do not unignore .ito/changes. Do not set git_only = true in release-plz.toml.

### Examples
The release-plz workflow defines a release job for push events on main and a release-pr job with concurrency group release-plz-${{ github.ref }}. Both jobs use the same app token, checkout, apt package cache, toolchain setup, and Rust cache steps.

## Facts
- **ito_coordination_paths**: .ito/changes, .ito/specs, .ito/modules, .ito/workflows, and .ito/audit are projected coordination paths and should remain gitignored. [convention]
- **tracked_ignored_files_remediation**: If release-plz reports tracked ignored files under .ito/changes, remove those files from Git tracking with git rm --cached while keeping local files on disk. [convention]
- **changes_gitignore_rule**: Do not unignore .ito/changes. [convention]
- **release_plz_config_location**: The root release-plz.toml is kept at the git repo root so release-plz can find the .git directory when cloning to a temp dir. [project]
- **release_plz_allow_dirty**: release-plz allow_dirty is set to false. [project]
- **release_plz_publish_allow_dirty**: release-plz publish_allow_dirty is set to false. [project]
- **release_plz_changelog_update**: Workspace changelog updates are enabled in release-plz. [project]
- **release_plz_dependencies_update**: Workspace dependency updates are enabled in release-plz. [project]
- **release_plz_changelog_config**: release-plz uses cliff.toml as the changelog configuration. [project]
- **ito_cli_git_tags**: The ito-cli package is the only package in this configuration with git tags enabled. [project]
