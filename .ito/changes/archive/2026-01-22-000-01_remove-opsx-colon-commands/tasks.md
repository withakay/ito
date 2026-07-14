# Tasks for: 000-01_remove-opsx-colon-commands

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Verify**: `node bin/ito.js validate --all` and `make build`

______________________________________________________________________

## Wave 1

### Task 1.1: Replace `/opsx:*` references with `/ito-*`

- **Files**: `src/**`, `docs/**`, `CHANGELOG.md`, `.github/workflows/polish-release-notes.yml`
- **Dependencies**: None
- **Action**:
  - Remove all `/opsx:*` references and standardize the experimental workflow to the hyphenated `/ito-*` commands.
  - Ensure generators/templates output `.claude/commands/ito-*.md` wrappers.
- **Verify**: `rg "/opsx:" src docs dist CHANGELOG.md .github/workflows`
- **Done When**: No `/opsx:*` references remain outside historical archives.
- **Status**: \[x\] complete

### Task 1.2: Validate and build

- **Files**: `.ito/changes/000-01_remove-opsx-colon-commands/**`
- **Dependencies**: Task 1.1
- **Action**:
  - Add change artifacts required by schema validation.
  - Ensure `ito validate --all` passes.
- **Verify**: `node bin/ito.js validate --all`
- **Done When**: All validations pass.
- **Status**: \[x\] complete
