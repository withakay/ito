# Tasks for: 005-01_migrate-pnpm-to-bun

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (phases must complete in order)
- **Template**: Enhanced task format with phases, verification, and status tracking

______________________________________________________________________

## Phase 1: Local Lockfile Migration and Validation

### Task 1.1: Install Bun locally

- **Files**: None (system installation)
- **Dependencies**: None
- **Action**:
  - Install Bun using official installer: `curl -fsSL https://bun.sh/install | bash`
  - Verify installation: `bun --version`
- **Verify**: `bun --version` shows installed version
- **Done When**: Bun is available in PATH and version is displayed
- **Status**: \[x\] complete

### Task 1.2: Clean working tree and generate bun.lock

- **Files**: `bun.lock` (new), `pnpm-lock.yaml` (preserved)
- **Dependencies**: Task 1.1
- **Action**:
  - Ensure clean git working tree (commit or stash changes)
  - Remove `node_modules/` directory
  - Run `bun install` to auto-migrate from `pnpm-lock.yaml`
  - Verify `bun.lock` is created and `pnpm-lock.yaml` is unchanged
- **Verify**: `ls bun.lock` shows the file exists
- **Done When**: `bun.lock` exists and dependencies are installed in `node_modules/`
- **Status**: \[x\] complete

### Task 1.3: Validate build workflow with Bun

- **Files**: `dist/` (build output)
- **Dependencies**: Task 1.2
- **Action**:
  - Run `bun run build`
  - Compare build artifacts in `dist/` with previous pnpm build output
  - Verify no differences in generated code
- **Verify**: `bun run build` completes successfully
- **Done When**: Build succeeds and artifacts match pnpm baseline
- **Status**: \[x\] complete

### Task 1.4: Validate test workflow with Bun

- **Files**: None (test execution)
- **Dependencies**: Task 1.3
- **Action**:
  - Run `bun run test` to execute all tests
  - Verify all tests pass
  - Run `bun run test:coverage` to generate coverage
- **Verify**: `bun run test` shows all tests passing
- **Done When**: All tests pass with same results as pnpm
- **Status**: \[x\] complete

### Task 1.5: Validate lint workflow with Bun

- **Files**: None (linting execution)
- **Dependencies**: Task 1.3
- **Action**:
  - Run `bun run lint`
  - Verify no new linting errors
- **Verify**: `bun run lint` completes without errors
- **Done When**: Linting passes with same results as pnpm
- **Status**: \[x\] complete

### Task 1.6: Commit bun.lock

- **Files**: `bun.lock`
- **Dependencies**: Task 1.3, Task 1.4, Task 1.5
- **Action**:
  - Git add `bun.lock`
  - Commit with message: "chore: add bun.lock (migrated from pnpm)"
- **Verify**: `git log -1` shows the commit
- **Done When**: `bun.lock` is committed to git
- **Status**: \[x\] complete

______________________________________________________________________

## Phase 2: Update Scripts and Tooling

### Task 2.1: Update package.json scripts

- **Files**: `package.json`
- **Dependencies**: Task 1.6
- **Action**:
  - Line 45: `dev:cli`: Change `pnpm build` → `bun run build`
  - Line 51: `prepare`: Change `pnpm run build` → `bun run build`
  - Line 52: `prepublishOnly`: Change `pnpm run build` → `bun run build`
  - Line 55: `release`: Change `pnpm run release:ci` → `bun run release:ci`
  - Line 56: `release:ci`: Change `pnpm run check:pack-version && pnpm exec changeset publish` → `bun run check:pack-version && bunx changeset publish`
- **Verify**: `grep -n "pnpm" package.json` shows no pnpm references in scripts
- **Done When**: All pnpm commands in package.json scripts replaced with Bun equivalents
- **Status**: \[x\] complete

### Task 2.2: Update Makefile targets

- **Files**: `Makefile`
- **Dependencies**: Task 1.6
- **Action**:
  - Line 9-18: Replace `pnpm test` → `bun run test`, `pnpm lint` → `bun run lint`
  - Update test-watch and test-coverage targets similarly
  - Refactor dev-install target (lines 20-35) to use Bun global install mechanism
    - Research `bun add -g .` or `bun link` workflow
    - Update commands to use Bun instead of pnpm
- **Verify**: `make test`, `make lint`, `make build` all work correctly
- **Done When**: All Makefile targets use Bun commands and execute successfully
- **Status**: \[x\] complete

### Task 2.3: Update test helper

- **Files**: `test/helpers/run-cli.ts`
- **Dependencies**: Task 1.6
- **Action**:
  - Line 62: Replace `runCommand('pnpm', ['run', 'build'])` → `runCommand('bun', ['run', 'build'])`
- **Verify**: `grep -n "pnpm" test/helpers/run-cli.ts` shows no pnpm references
- **Done When**: Test helper uses Bun for build command
- **Status**: \[x\] complete

### Task 2.4: Run tests to validate changes

- **Files**: None (test execution)
- **Dependencies**: Task 2.1, Task 2.2, Task 2.3
- **Action**:
  - Run `bun run test` to ensure all tests still pass
  - Run `make test` to verify Makefile integration
- **Verify**: `bun run test` and `make test` both pass
- **Done When**: All tests pass after script updates
- **Status**: \[x\] complete

______________________________________________________________________

## Phase 3: Update CI Workflows

### Task 3.1: Update ci.yml workflow

- **Files**: `.github/workflows/ci.yml`
- **Dependencies**: Task 2.4
- **Action**:
  - Replace `pnpm/action-setup@v4` with `oven-sh/setup-bun@v2`
  - Keep `actions/setup-node@v4` (still needed for Node runtime)
  - Replace `pnpm install --frozen-lockfile` with `bun ci`
  - Replace `pnpm run build` with `bun run build`
  - Replace `pnpm test` with `bun run test`
  - Replace `pnpm exec tsc --noEmit` with `bunx tsc --noEmit`
  - Update cache key from `pnpm` to appropriate Bun cache path if needed
- **Verify**: Push to branch and observe GitHub Actions run
- **Done When**: CI workflow uses Bun and passes on all platforms (Linux, macOS, Windows)
- **Status**: \[x\] complete

### Task 3.2: Update release-prepare.yml workflow

- **Files**: `.github/workflows/release-prepare.yml`
- **Dependencies**: Task 2.4
- **Action**:
  - Replace `pnpm/action-setup@v4` with `oven-sh/setup-bun@v2`
  - Keep Node 24 for npm OIDC requirements
  - Update `changesets/action@v1` publish command from `pnpm run release:ci` to `bun run release:ci`
- **Verify**: Inspect workflow file for correctness (release testing requires actual release)
- **Done When**: Release workflow updated to use Bun
- **Status**: \[x\] complete

### Task 3.3: Validate CI on all platforms

- **Files**: None (CI validation)
- **Dependencies**: Task 3.1, Task 3.2
- **Action**:
  - Push changes to a PR branch
  - Verify GitHub Actions runs successfully on Linux, macOS, and Windows
  - Check that `bun ci`, `bun run build`, `bun run test`, `bunx tsc --noEmit` all succeed
- **Verify**: GitHub Actions shows green checkmarks on all matrix entries
- **Done When**: CI passes on all platforms with Bun
- **Status**: \[x\] complete

______________________________________________________________________

## Phase 4: Update Development Environment

### Task 4.1: Update devcontainer configuration

- **Files**: `.devcontainer/devcontainer.json`
- **Dependencies**: Task 2.4
- **Action**:
  - Replace pnpm/corepack installation with Bun installation
  - Update postCreateCommand from `pnpm install` to `bun install`
  - Example: Use `curl -fsSL https://bun.sh/install | bash && bun install`
- **Verify**: Rebuild devcontainer and run `bun --version`
- **Done When**: Devcontainer installs Bun and runs `bun install` successfully
- **Status**: \[x\] complete

______________________________________________________________________

## Phase 5: Update Documentation

### Task 5.1: Update README.md

- **Files**: `README.md`
- **Dependencies**: Task 3.3
- **Action**:
  - Update contributing section to use Bun instead of pnpm
  - Change install instructions from `pnpm install` to `bun install`
  - Update example commands to use `bun run` and `bunx`
- **Verify**: `grep -n "pnpm" README.md` shows minimal or no references (some historical refs acceptable)
- **Done When**: README.md reflects Bun as the standard package manager
- **Status**: \[x\] complete

### Task 5.2: Update AGENTS.md

- **Files**: `AGENTS.md`
- **Dependencies**: Task 3.3
- **Action**:
  - Update "Development Commands" section
  - Change Makefile note from "uses pnpm internally" to "uses Bun internally"
- **Verify**: `grep -n "pnpm" AGENTS.md` shows updated references
- **Done When**: AGENTS.md reflects Bun usage
- **Status**: \[x\] complete

### Task 5.3: Update other documentation and templates

- **Files**: `docs/schema-customization.md`, `schemas/spec-driven/templates/tasks.md`, other docs
- **Dependencies**: Task 3.3
- **Action**:
  - Search for pnpm references in docs/ and schemas/
  - Update user-facing examples to use Bun
  - Leave historical `.ito/changes/archive/` references unchanged (optional cleanup)
- **Verify**: `grep -r "pnpm" docs/ schemas/` shows minimal active references
- **Done When**: Active documentation reflects Bun usage
- **Status**: \[x\] complete

______________________________________________________________________

## Phase 6: Final Validation and Cleanup

### Task 6.1: Remove pnpm-lock.yaml

- **Files**: `pnpm-lock.yaml` (removed)
- **Dependencies**: Task 3.3, Task 5.3
- **Action**:
  - Verify CI is green and all workflows are passing
  - Remove `pnpm-lock.yaml` (or rename to `pnpm-lock.yaml.bak` temporarily)
  - Commit removal: `git rm pnpm-lock.yaml`
- **Verify**: `ls pnpm-lock.yaml` shows file not found
- **Done When**: `pnpm-lock.yaml` removed from repository
- **Status**: [-] discarded (obsolete - TypeScript migration)
- **Status**: \[x\] complete

### Task 6.2: Validate dev-install workflow

- **Files**: None (developer workflow testing)
- **Dependencies**: Task 2.2, Task 6.1
- **Action**:
  - Run `make dev-install` in a clean environment
  - Verify the `ito` command is globally available
  - Test executing `ito --version` to confirm it's the local development version
- **Verify**: `ito --version` shows expected version
- **Done When**: `make dev-install` successfully installs the package globally for testing
- **Status**: [-] discarded (obsolete - TypeScript migration)
- **Status**: \[x\] complete

### Task 6.3: Final full test suite

- **Files**: None (comprehensive testing)
- **Dependencies**: Task 6.1, Task 6.2
- **Action**:
  - Run `bun install` in a clean checkout
  - Run `bun run build`
  - Run `bun run test`
  - Run `bun run lint`
  - Run `make test`, `make build`, `make lint`
  - Verify all commands succeed
- **Verify**: All test, build, and lint commands pass
- **Done When**: Complete test suite passes with Bun
- **Status**: [-] discarded (obsolete - TypeScript migration)
- **Status**: \[x\] complete

### Task 6.4: Verify release workflow (dry-run)

- **Files**: None (release testing)
- **Dependencies**: Task 6.3
- **Action**:
  - Test changesets workflow: `bunx changeset version` (dry-run)
  - Verify `bunx changeset publish` command structure (no actual publish)
  - Ensure release workflow can execute without errors
- **Verify**: Changesets commands execute without errors
- **Done When**: Release workflow validated (no actual publish required)
- **Status**: [-] discarded (obsolete - TypeScript migration)
- **Status**: \[x\] complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified

## Phase Guidelines

- Phases must complete in order (1 → 2 → 3 → 4 → 5 → 6)
- Tasks within a phase can be executed sequentially or in parallel (check dependencies)
- Each task includes verification criteria for quality assurance
- CI validation (Phase 3) is critical before proceeding to documentation updates

## Verification Summary

After all tasks complete, the following should be true:

- `bun.lock` exists and is used for all installs
- `pnpm-lock.yaml` is removed
- All scripts use `bun run` or `bunx` instead of pnpm
- CI workflows use `oven-sh/setup-bun@v2` and `bun ci`
- All CI platforms (Linux, macOS, Windows) pass
- Documentation reflects Bun as standard package manager
- Build artifacts are identical to pnpm baseline
- Release workflow tested (changesets commands work)
