# Tasks for: 005-03_ci-cross-platform-releases

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Created**: 2026-01-31

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Define release target matrix and artifact names

- **Files**: `.github/workflows/`, `ito-rs/Cargo.toml`, `ito-rs/`
- **Dependencies**: None
- **Action**:
  - Define the supported targets (macOS/Linux/Windows, x86_64 + ARM where feasible) and the release asset naming scheme.
  - Decide the tag format (e.g. `vX.Y.Z`) and how the workflow validates versions.
- **Verify**: `make test`
- **Done When**: Target matrix and versioning rules are documented and agreed
- **Updated At**: 2026-01-31
- **Status**: [ ] pending

### Task 1.2: Add GitHub Actions release workflow

- **Files**: `.github/workflows/`
- **Dependencies**: Task 1.1
- **Action**:
  - Add a workflow that triggers on tags and publishes a GitHub Release with artifacts.
  - Ensure it builds `ito-rs` in release mode for each target and uploads assets.
  - Ensure the workflow produces and publishes checksums.
- **Verify**: GitHub Actions run on a test tag (or `workflow_dispatch`) succeeds
- **Done When**: Release workflow creates a draft or published release with all expected assets
- **Updated At**: 2026-01-31
- **Status**: [ ] pending

### Task 1.3: Add macOS/Linux installer script

- **Files**: `scripts/`, docs, release workflow
- **Dependencies**: Task 1.2
- **Action**:
  - Implement an installer that detects OS/arch, downloads the correct asset, verifies checksum, and installs `ito`.
  - Document the one-liner install command.
- **Verify**: Install script succeeds on macOS and Linux runners in CI
- **Done When**: Users can install with a copy/paste command and `ito --version` works
- **Updated At**: 2026-01-31
- **Status**: [ ] pending

______________________________________________________________________

## Checkpoints

### Checkpoint: Review Implementation

- **Type**: checkpoint (requires human approval)
- **Dependencies**: All Wave 1 tasks
- **Action**: Review the implementation before proceeding
- **Done When**: User confirms implementation is correct
- **Updated At**: 2026-01-31
- **Status**: [ ] pending
