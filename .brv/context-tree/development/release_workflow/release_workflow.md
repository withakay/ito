---
createdAt: '2026-04-25T20:49:39.073Z'
keywords: []
related: [development/release_workflow/release_workflow.abstract.md, development/release_workflow/release_workflow.overview.md, development/release_workflow/build_and_coverage_guardrails.md, development/release_workflow/manifesto_instruction_implementation_notes.md]
summary: Ito release process covering release-plz PRs, cargo-dist tag-triggered GitHub releases, Homebrew tap updates, local install notes, and release configuration files.
tags: []
title: Release Workflow
updatedAt: '2026-04-25T20:49:39.073Z'
---
## Reason
Document the Ito release process and CI release pipeline

## Raw Concept
**Task:**
Document the Ito release workflow and release automation pipeline

**Changes:**
- Release-plz merges a release PR, publishes crates.io releases, and creates version tags
- cargo-dist builds and publishes GitHub Releases from version tags
- Homebrew formula updates are pushed to withakay/homebrew-ito

**Files:**
- .github/workflows/release-plz.yml
- .github/workflows/v-release.yml
- .github/workflows/polish-release-notes.yml
- dist-workspace.toml
- release-plz.toml

**Flow:**
merge release PR -> release-plz publishes crates and tags vX.Y.Z -> cargo-dist builds and creates GitHub Release -> Homebrew formula is updated -> release notes are polished

**Timestamp:** 2026-04-25

## Narrative
### Structure
The release pipeline is split between release-plz for versioning/publishing and cargo-dist for artifact builds, GitHub Releases, and Homebrew publishing. The repo root release-plz.toml and dist-workspace.toml coordinate the automation.

### Dependencies
Requires GitHub Actions, release-plz, cargo-dist, crates.io token, Homebrew tap token, and optionally Claude Code OAuth for release note polishing.

### Highlights
The workflow supports GitHub Releases, cross-platform installer artifacts, Homebrew formula updates, and local Homebrew installation via the withakay/ito tap.

### Rules
Do not set git_only = true in release-plz.toml; it can cause release-plz to miscalculate repository paths during diff/worktree operations. The publish-homebrew-formula job errors if the generated formula already contains a service do block.

### Examples
Local install examples: brew install withakay/ito/ito; brew upgrade ito; brew unlink ito-cli; brew link ito; verify with /opt/homebrew/bin/ito --version.
