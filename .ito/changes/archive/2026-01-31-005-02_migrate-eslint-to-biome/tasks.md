# Tasks for: 005-02_migrate-eslint-to-biome

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking

______________________________________________________________________

## Wave 1

### Task 1.1: Add Biome and baseline config

- **Files**: `package.json`, `biome.json`
- **Dependencies**: None
- **Action**:
  - Add `@biomejs/biome` as a dev dependency.
  - Create a baseline `biome.json` (root config, VCS integration, JS/TS enabled).
- **Verify**: `bunx biome --version`
- **Done When**: Biome is installed and `biome.json` is present.
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 1.2: Configure restricted imports guardrail

- **Files**: `biome.json`
- **Dependencies**: Task 1.1
- **Action**:
  - Enable `linter.rules.style.noRestrictedImports`.
  - Restrict `@inquirer/*` via `patterns` with an actionable message.
  - Add an override to disable the rule for `src/core/init.ts` (to match current behavior).
- **Verify**: `bunx biome check src/`
- **Done When**: Biome reports restricted imports outside the allowed file.
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 2 (after Wave 1 complete)

### Task 2.1: Switch lint script to Biome

- **Files**: `package.json`
- **Dependencies**: Task 1.2
- **Action**:
  - Replace `lint` script to run Biome (e.g., `biome check src/`).
  - Ensure `bun run lint` remains the canonical lint entrypoint.
- **Verify**: `bun run lint`
- **Done When**: `bun run lint` lints via Biome and fails on violations.
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 2.2: Add formatting scripts

- **Files**: `package.json`
- **Dependencies**: Task 2.1
- **Action**:
  - Add `format` (write) and `format:check` (no-write) scripts using Biome.
  - Decide scope (repo-wide vs `src/`) and encode it in scripts.
- **Verify**: `bun run format:check`
- **Done When**: Formatting can be applied and checked deterministically.
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 3 (after Wave 2 complete)

### Task 3.1: Remove ESLint tooling

- **Files**: `package.json`, `eslint.config.js`
- **Dependencies**: Task 2.1
- **Action**:
  - Remove `eslint` and `typescript-eslint` from dev dependencies.
  - Delete `eslint.config.js`.
  - Ensure no remaining scripts or docs reference ESLint for the lint workflow.
- **Verify**: `bun install && bun run lint`
- **Done When**: ESLint is fully removed and linting still works.
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 3.2: Update CI and docs references

- **Files**: `.github/workflows/ci.yml`, `Makefile`, `README.md`
- **Dependencies**: Task 3.1
- **Action**:
  - Keep CI invoking `bun run lint` (no direct tool coupling), but ensure docs mention Biome where helpful.
  - Keep Makefile targets unchanged unless they reference ESLint directly.
- **Verify**: `bun run lint`
- **Done When**: CI/doc references align with Biome-based linting.
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 4 (Validation)

### Task 4.1: End-to-end verification

- **Files**: (none)
- **Dependencies**: Task 3.2
- **Action**:
  - Run full validation to ensure the migration didn’t regress tooling.
- **Verify**: `bun run lint && bun run format:check && bunx tsc --noEmit && bun run test`
- **Done When**: All verification commands pass locally.
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 5 (Checkpoint)

### Task 5.1: Review parity and developer experience

- **Type**: checkpoint (requires human approval before proceeding)
- **Files**: `biome.json`, `package.json`, `.github/workflows/ci.yml`
- **Dependencies**: Task 4.1
- **Action**:
  - Confirm rule parity is acceptable (especially restricted imports).
  - Confirm `bun run lint` and `bun run format` are the desired UX.
- **Done When**: Reviewer approves the migration approach.
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
