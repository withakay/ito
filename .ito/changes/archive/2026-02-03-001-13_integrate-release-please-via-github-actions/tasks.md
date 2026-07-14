# Tasks for: 001-13_integrate-release-please-via-github-actions

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-13_integrate-release-please-via-github-actions
ito tasks next 001-13_integrate-release-please-via-github-actions
ito tasks start 001-13_integrate-release-please-via-github-actions 1.1
ito tasks complete 001-13_integrate-release-please-via-github-actions 1.1
ito tasks shelve 001-13_integrate-release-please-via-github-actions 1.1
ito tasks unshelve 001-13_integrate-release-please-via-github-actions 1.1
ito tasks show 001-13_integrate-release-please-via-github-actions
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Confirm current release pipeline expectations

- **Files**: `.github/workflows/release.yml`, `ito-rs/Cargo.toml`
- **Dependencies**: None
- **Action**:
  - Document what the tag-triggered release workflow expects (tag format, version source, release creation behavior).
  - Identify which Cargo package(s) must be updated by release-please (workspace version vs specific crate).
- **Verify**: Manual inspection (no runtime verification)
- **Done When**: Requirements in `specs/release-please-releases/spec.md` match the observed pipeline constraints
- **Updated At**: 2026-02-02
- **Status**: [x] completed

### Task 1.2: Add release-please workflow and config

- **Files**: `.github/workflows/release-please.yml`, `release-please-config.json`, `.release-please-manifest.json`
- **Dependencies**: Task 1.1
- **Action**:
  - Add a GitHub Actions workflow that runs `googleapis/release-please-action` on `main`.
  - Configure release-please for Rust version/changelog updates consistent with the existing tag-triggered pipeline.
  - Ensure workflow permissions and concurrency are appropriate.
- **Verify**: YAML validates; workflow is syntactically correct
- **Done When**: Repo contains a release-please workflow + config files and they align with spec requirements
- **Updated At**: 2026-02-02
- **Status**: [x] completed

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Align existing release-related workflows with release-please

- **Files**: `.github/workflows/release.yml`, `.github/workflows/polish-release-notes.yml`
- **Dependencies**: Task 1.2
- **Action**:
  - Ensure the release pipeline remains triggered correctly by release-please-created tags/releases.
  - Decide draft vs published behavior and adjust the polish workflow trigger if needed.
- **Verify**: Manual inspection (release testing requires real tags/releases)
- **Done When**: Workflows have consistent triggers and no obvious double-release behavior
- **Updated At**: 2026-02-02
- **Status**: [x] completed

### Task 2.2: Update maintainer documentation

- **Files**: `README.md`, `docs/**` (as appropriate)
- **Dependencies**: Task 2.1
- **Action**:
  - Document the preferred release flow via release-please.
  - Document fallback/manual steps and how to recover from a failed release run.
- **Verify**: Manual doc review
- **Done When**: Docs reflect release-please-first releases and remove/soften manual tagging guidance
- **Updated At**: 2026-02-02
- **Status**: [x] completed
