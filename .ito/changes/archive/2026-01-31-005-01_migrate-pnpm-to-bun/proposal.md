## Why

Migrating from pnpm to Bun as the package manager will improve developer experience through faster install times, simpler configuration, and better performance. Bun's modern package management capabilities and compatibility with existing workflows make this a low-risk migration that benefits all contributors.

## What Changes

- Replace pnpm with Bun across all package management, build, and test workflows
- Update lockfile from `pnpm-lock.yaml` to `bun.lock`
- Update all scripts, CI workflows, Makefile, and development tooling to use Bun commands
- Update documentation and templates to reflect Bun as the standard package manager
- Maintain Node.js runtime requirement (this is a package manager migration, not a runtime change)

## Capabilities

### New Capabilities

- `bun-package-management`: Core package installation and dependency management using Bun
- `bun-ci-integration`: CI/CD workflows using Bun for reproducible builds
- `bun-dev-workflow`: Developer workflows (build, test, lint) using Bun commands

### Modified Capabilities

<!-- No existing capabilities require spec-level changes - this is a tooling migration -->

## Impact

**Files Modified:**

- `package.json` - Script commands updated to use Bun
- `Makefile` - All pnpm commands replaced with Bun equivalents
- `.github/workflows/ci.yml` - CI setup migrated from pnpm to Bun
- `.github/workflows/release-prepare.yml` - Release workflow migrated to Bun
- `.devcontainer/devcontainer.json` - Development container setup updated
- `test/helpers/run-cli.ts` - Test helper build commands updated
- `README.md` - Contributing instructions updated
- `AGENTS.md` - Development command documentation updated

**Dependencies:**

- No dependency changes - existing packages remain the same
- Bun will auto-migrate `pnpm-lock.yaml` to `bun.lock` during first install

**Developer Experience:**

- Developers must install Bun (`curl -fsSL https://bun.sh/install | bash`)
- Command changes: `pnpm` → `bun`, `pnpm exec` → `bunx`
- Node.js still required (>=20.19.0) for runtime and some tooling
- `make dev-install` workflow requires adjustment for Bun global installs

**CI/CD:**

- GitHub Actions will use `oven-sh/setup-bun@v2` instead of `pnpm/action-setup@v4`
- Install command changes to `bun ci` (frozen lockfile equivalent)
- No expected CI compatibility issues - Bun supports all target platforms (Linux, macOS, Windows)

**Risks:**

- Low: Bun has mature pnpm lockfile migration support
- Dependency lifecycle scripts handled by Bun's trusted dependencies model
- Windows compatibility validated in CI matrix
