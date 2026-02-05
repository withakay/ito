## Context

This project currently uses pnpm as its package manager across all workflows: local development (Makefile), CI/CD (GitHub Actions), development containers (.devcontainer), and test helpers. The migration to Bun aims to improve developer experience through faster installs and simpler configuration while maintaining full compatibility with existing Node.js-based tooling.

**Current State:**

- Package manager: pnpm with `pnpm-lock.yaml`
- Build/test/lint: Executed via pnpm scripts
- CI: GitHub Actions with `pnpm/action-setup@v4`
- Dev environment: devcontainer with corepack + pnpm
- Runtime: Node.js >= 20.19.0 (unchanged)

**Constraints:**

- Must maintain Node.js runtime requirement (this is a package manager migration, not a runtime change)
- Must support all current platforms: Linux, macOS, Windows
- Must maintain compatibility with existing tooling: vitest, tsc, eslint, changesets
- Zero functional changes to build output or runtime behavior

## Goals / Non-Goals

**Goals:**

- Replace pnpm with Bun for all package management operations
- Migrate `pnpm-lock.yaml` to `bun.lock` successfully
- Update all scripts, CI workflows, Makefile, and documentation
- Validate cross-platform compatibility (Linux, macOS, Windows)
- Maintain reproducible builds and frozen lockfile enforcement in CI

**Non-Goals:**

- Migrating runtime from Node.js to Bun (future consideration)
- Replacing vitest/tsc/eslint with Bun alternatives (out of scope)
- Changing package.json dependencies (lockfile migration only)
- Modifying test assertions or implementation logic

## Decisions

### Decision 1: Package Manager Migration Only (Not Runtime)

**Choice:** Migrate package manager to Bun while keeping Node.js as the runtime.

**Rationale:**

- Minimizes risk by isolating the change to package management
- Existing tooling (vitest, tsc, eslint) expects Node.js runtime
- Package.json `engines` field requires Node >= 20.19.0
- Bun's package manager is mature and production-ready
- Runtime migration can be evaluated separately in the future

**Alternatives Considered:**

- Full migration to Bun runtime: Rejected due to compatibility concerns with existing tooling and higher risk
- Stay with pnpm: Rejected because Bun offers better performance and simpler configuration

### Decision 2: Lockfile Migration Strategy

**Choice:** Use Bun's automatic `pnpm-lock.yaml` migration on first `bun install`.

**Rationale:**

- Bun natively supports reading `pnpm-lock.yaml` and generating equivalent `bun.lock`
- Preserves original `pnpm-lock.yaml` for verification
- Low-risk: Can validate identical dependency resolution before committing
- Official Bun documentation recommends this approach

**Alternatives Considered:**

- Manual lockfile regeneration: Rejected because automatic migration is safer and preserves exact versions
- Gradual migration: Rejected because it adds complexity without reducing risk

### Decision 3: CI Setup Action

**Choice:** Use `oven-sh/setup-bun@v2` instead of `pnpm/action-setup@v4`.

**Rationale:**

- Official Bun setup action maintained by the Bun team
- Handles Bun installation and PATH configuration
- Supports version pinning for reproducibility
- Well-documented CI integration pattern

**Alternatives Considered:**

- Manual Bun installation via curl: Rejected due to additional complexity and lack of version pinning
- Keep pnpm/action-setup and install Bun manually: Rejected because it's redundant

### Decision 4: Frozen Lockfile Command

**Choice:** Use `bun ci` in CI instead of `bun install --frozen-lockfile`.

**Rationale:**

- `bun ci` is the official CI-optimized command (equivalent to `bun install --frozen-lockfile`)
- Clearer intent and shorter command
- Matches industry convention (npm ci, pnpm install --frozen-lockfile)

**Alternatives Considered:**

- `bun install --frozen-lockfile`: Functionally identical but less idiomatic

### Decision 5: dev-install Makefile Target

**Choice:** Refactor `make dev-install` to use Bun's global install mechanism.

**Rationale:**

- Current implementation relies on `pnpm root -g` and `pnpm -g add .`
- Bun supports global installs via `bun add -g <path>`
- Must verify Bun's local directory global install behavior
- May require `bun link` workflow instead

**Alternatives Considered:**

- Keep using npm for global installs: Acceptable fallback if Bun's global install from local path is insufficient
- Remove dev-install target: Rejected because it's a useful developer workflow

### Decision 6: Script Command Syntax

**Choice:** Use `bun run <script>` for package.json scripts and `bunx <bin>` for executables.

**Rationale:**

- `bun run` is explicit and matches pnpm/npm convention
- `bunx` is Bun's equivalent to `npx`/`pnpm exec`
- Clear distinction between scripts and binaries

**Alternatives Considered:**

- Use `bun <script>` shorthand: Less explicit, harder to distinguish from Bun's built-in commands

## Risks / Trade-offs

### Risk: Dependency Lifecycle Scripts

**Risk:** Some dependencies may have lifecycle scripts that don't execute if not on Bun's trusted list.

**Mitigation:**

- Bun has internal default trusted list for popular packages (esbuild, etc.)
- Test full build/test workflow after migration to catch missing scripts
- Add `trustedDependencies` to package.json if needed
- Reference: https://bun.com/docs/pm/lifecycle

### Risk: Windows Compatibility

**Risk:** Bun's Windows support may have edge cases not caught in development.

**Mitigation:**

- CI matrix already includes Windows (pwsh)
- Validate `bun ci`, `bun run build`, `bun run test` on Windows in CI
- Bun supports Windows 10 1809+ (within project's support matrix)

### Risk: Global Install Workflow

**Risk:** `make dev-install` may not work identically with Bun's global install mechanism.

**Mitigation:**

- Test Bun's `bun add -g .` and `bun link` workflows
- Document any workflow changes for developers
- Fallback: Use npm for global install if Bun's approach is insufficient

### Risk: CI Caching Differences

**Risk:** Bun's cache strategy differs from pnpm, potentially affecting CI performance.

**Mitigation:**

- `setup-bun` action handles caching automatically
- Monitor CI execution times post-migration
- Explicitly cache `~/.bun/install/cache` if needed

### Trade-off: Node.js Still Required

**Trade-off:** This migration does not eliminate the Node.js dependency.

**Rationale:**

- Tooling (vitest, tsc, eslint) still requires Node.js runtime
- Package.json `engines` field enforces Node >= 20.19.0
- Future work could evaluate replacing these tools with Bun-native alternatives

## Implementation Strategy

### Phase 1: Local Lockfile Migration and Validation

1. Clean working tree and remove `node_modules/`
1. Run `bun install` to generate `bun.lock` from `pnpm-lock.yaml`
1. Validate build/test/lint workflows:
   ```bash
   bun install
   bun run build
   bun run test
   bun run lint
   ```
1. Compare build artifacts and test results with pnpm baseline
1. Commit `bun.lock` when validated

### Phase 2: Update Scripts and Tooling

1. Update `package.json` scripts:
   - Replace `pnpm run` → `bun run`
   - Replace `pnpm exec` → `bunx`
1. Update `Makefile`:
   - Replace all pnpm commands with Bun equivalents
   - Refactor `dev-install` target for Bun global installs
1. Update `test/helpers/run-cli.ts`:
   - Replace `pnpm run build` → `bun run build`

### Phase 3: Update CI Workflows

1. Update `.github/workflows/ci.yml`:
   - Replace `pnpm/action-setup@v4` with `oven-sh/setup-bun@v2`
   - Replace `pnpm install --frozen-lockfile` with `bun ci`
   - Replace `pnpm run build` with `bun run build`
   - Replace `pnpm test` with `bun run test`
   - Replace `pnpm exec tsc --noEmit` with `bunx tsc --noEmit`
1. Update `.github/workflows/release-prepare.yml`:
   - Same setup-bun swap
   - Replace `pnpm run release:ci` with `bun run release:ci`

### Phase 4: Update Development Environment

1. Update `.devcontainer/devcontainer.json`:
   - Replace pnpm/corepack setup with Bun installation
   - Run `bun install` in postCreateCommand

### Phase 5: Update Documentation

1. Update `README.md` contributing section
1. Update `AGENTS.md` Makefile documentation
1. Update any templates or examples in `docs/` and `schemas/`
1. Remove or archive `pnpm-lock.yaml`

### Phase 6: Validation and Verification

1. Run full CI matrix on all platforms (Linux, macOS, Windows)
1. Verify release workflow (changesets version + publish)
1. Test dev-install workflow
1. Validate devcontainer setup

## Migration Plan

**Pre-migration Checklist:**

- [ ] Bun installed locally for testing
- [ ] Clean working tree (no uncommitted changes)
- [ ] CI passing on main branch

**Migration Steps:**

1. Execute Phase 1 locally (lockfile migration)
1. Execute Phase 2 (scripts and tooling)
1. Execute Phase 3 (CI workflows)
1. Execute Phase 4 (devcontainer)
1. Execute Phase 5 (documentation)
1. Execute Phase 6 (validation)

**Rollback Strategy:**

- If migration fails, revert all changes and restore `pnpm-lock.yaml`
- Keep `pnpm-lock.yaml.bak` until migration is validated
- CI matrix provides safety net for platform-specific issues

**Success Criteria:**

- `bun.lock` committed and used across all environments
- All CI workflows green on Linux, macOS, Windows
- Build artifacts identical to pnpm baseline
- Release workflow tested (dry-run or staging publish)
- Developer documentation updated

## Open Questions

None - migration strategy is well-defined with clear implementation phases.
