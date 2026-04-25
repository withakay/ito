## Key points
- The Ito release process is split between **release-plz** for versioning/publishing and **cargo-dist** for building release artifacts and GitHub Releases.
- A typical flow is: **merge release PR → release-plz publishes crates and tags `vX.Y.Z` → cargo-dist builds and creates GitHub Release → Homebrew formula is updated → release notes are polished**.
- Release automation relies on several GitHub Actions workflows and root configuration files: `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, and `release-plz.toml`.
- The pipeline supports **crates.io publishing**, **GitHub Releases**, **cross-platform installer artifacts**, and **Homebrew tap updates** to `withakay/homebrew-ito`.
- Important operational dependencies include **GitHub Actions**, **release-plz**, **cargo-dist**, a **crates.io token**, a **Homebrew tap token**, and optionally **Claude Code OAuth** for polishing release notes.
- A key configuration rule is **not to set `git_only = true` in `release-plz.toml`**, because it can cause repository path miscalculation during diff/worktree operations.
- The Homebrew publishing job has a specific failure mode: it errors if the generated formula already contains a **service do** block.

## Structure / sections summary
- **Reason**: States the purpose is to document the Ito release process and CI release pipeline.
- **Raw Concept**: Summarizes the intended task, the main changes in the workflow, the affected files, the end-to-end release flow, and a timestamp.
- **Narrative**
  - **Structure**: Explains the split of responsibilities between release-plz and cargo-dist, and notes that `release-plz.toml` and `dist-workspace.toml` coordinate automation.
  - **Dependencies**: Lists the tooling, tokens, and optional OAuth needed to run the pipeline.
  - **Highlights**: Notes supported outputs like GitHub Releases, installer artifacts, and Homebrew publishing/local install support.
  - **Rules**: Captures configuration constraints and a formula-generation edge case.
  - **Examples**: Provides local Homebrew usage commands and a version-check path.

## Notable entities, patterns, or decisions
- **release-plz**: Handles release PR merging, crates.io publishing, and version tagging.
- **cargo-dist**: Builds artifacts and publishes GitHub Releases based on version tags.
- **Homebrew tap**: Publishes via `withakay/homebrew-ito` and supports local installation through `withakay/ito`.
- **Workflow files**: `release-plz.yml`, `v-release.yml`, and `polish-release-notes.yml` imply a staged automation pipeline.
- **Configuration files**: `release-plz.toml` and `dist-workspace.toml` are the central repo-root controls for the release setup.
- **Decision/rule**: Avoid `git_only = true` in release-plz configuration due to path/diff issues.
- **Local install pattern**: Commands include `brew install withakay/ito/ito`, `brew upgrade ito`, `brew unlink ito-cli`, `brew link ito`, and verification via `/opt/homebrew/bin/ito --version`.