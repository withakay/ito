## Key points
- `release-plz` is configured to run from the repository root with `allow_dirty = false` and `publish_allow_dirty = false`.
- Workspace changelog updates and workspace dependency updates are enabled.
- The release setup keeps projected `.ito` coordination paths gitignored, including `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit`.
- If any of those ignored paths are already tracked, the fix is to remove them from Git tracking with `git rm --cached` while leaving local files on disk.
- `git_only = true` should **not** be set in `release-plz.toml`.
- The GitHub Actions release workflow runs on `main` and includes both release and release-pr jobs.
- `ito-cli` is the only package with git tags enabled.

## Structure / sections summary
- **Reason**: States the purpose of documenting `release-plz` configuration and coordination-branch ignore behavior.
- **Raw Concept**: Summarizes the task, changes made, files involved, workflow flow, timestamp, author, and key path pattern.
- **Narrative**:
  - **Structure**: Explains repository-root placement of `release-plz.toml` and `.gitignore` usage for coordination paths.
  - **Dependencies**: Lists workflow prerequisites such as GitHub App token, checkout depth, build tools, toolchain setup, Rust cache, and `release-plz/action`.
  - **Highlights**: Describes the main configuration choices and coordination-branch behavior.
  - **Rules**: Defines the required ignore/tracking behavior and configuration constraints.
  - **Examples**: Describes the release and release-pr job setup and their shared steps.
- **Facts**: Enumerates the concrete configuration and convention items with their scope labels.

## Notable entities, patterns, or decisions mentioned
- **Projected Ito coordination paths pattern**: `^.ito/(changes|specs|modules|workflows|audit)$`
- **Files involved**:
  - `.gitignore`
  - `release-plz.toml`
  - `.github/workflows/release-plz.yml`
- **Workflow dependencies**:
  - GitHub App token
  - `actions/checkout` with `fetch-depth: 0`
  - `build-essential`
  - `mise` toolchain setup
  - Rust cache
  - `release-plz/action@v0.5`
  - `CARGO_REGISTRY_TOKEN` for publishing
- **Configuration decisions**:
  - Changelog config uses `cliff.toml`
  - Dirty publishing is disabled
  - Semver checks are disabled
  - Git tags are enabled only for `ito-cli`
- **Operational rule**: keep coordination paths ignored without unignoring `.ito/changes`; remediate tracked ignored files via `git rm --cached` rather than changing ignore rules