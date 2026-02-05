# Tasks for: 005-04_npm-binary-packages

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Created**: 2026-01-31

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Decide npm package naming and platform split

- **Files**: docs, npm package metadata (to be added)
- **Dependencies**: None
- **Action**:
  - Choose package naming (scoped vs unscoped) and the mapping from targets to package names.
  - Confirm which targets are in-scope for the first iteration.
- **Verify**: N/A
- **Done When**: Naming and target list are documented and agreed
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 1.2: Implement npm packaging pipeline

- **Files**: `.github/workflows/`, package manifests, release tooling config
- **Dependencies**: Task 1.1
- **Action**:
  - Add automation to build or consume release binaries and publish npm packages.
  - Ensure publish is gated to releases and uses a GitHub secret for npm auth.
  - Ensure versions match the Ito release version.
- **Verify**: Publish dry-run (or publish to a test scope) succeeds
- **Done When**: npm packages can be published for at least one platform end-to-end
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 1 tasks
- **Action**: Review the implementation before proceeding
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-01-31
- **Status**: [ ] pending
