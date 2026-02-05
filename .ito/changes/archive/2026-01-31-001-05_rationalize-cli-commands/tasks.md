# Tasks for: 001-05_rationalize-cli-commands

## Execution Notes

- **Tool**: OpenCode
- **Mode**: Sequential
- **Note**: Do not implement until proposal is approved

______________________________________________________________________

## Wave 1: Rationalize The Full CLI Surface

### Task 1.0: Codify the CLI surface from the help audit

- **Files**: `.ito/changes/001-05_rationalize-cli-commands/specs/cli-surface/spec.md`
- **Dependencies**: None
- **Action**:
  - Audit the current help surface (`ito --help` + major subcommand help pages)
  - Decide the exact preferred command surface
  - Encode the final decision as requirements (visible commands + hidden deprecated shims)
- **Verify**: `ito --help` matches the spec after Wave 1
- **Done When**: the CLI surface is unambiguous and the spec is the single source of truth
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 1.1: Implement the small stable help surface

- **Files**: `src/cli/index.ts`
- **Dependencies**: None
- **Action**:
  - Ensure `ito --help` only shows the stable commands and visible experimentals:
    - stable: `init`, `update`, `dashboard`, `status`, `ralph`, `create`, `list`, `show`, `validate`, `archive`, `split`, `config`, `completions`
    - experimental: `x-templates`, `x-schemas`
  - Keep legacy commands callable as deprecated shims (warning + hidden)
  - Remove skills from the visible CLI surface
- **Verify**: `ito --help`
- **Done When**: help output is consistent and only shows the preferred surface
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 1.2: Group config and completions (and deprecate old verbs)

- **Files**: `src/cli/index.ts`, `src/commands/config.ts`, `src/commands/completion.ts`
- **Dependencies**: Task 1.1
- **Action**:
  - Add visible grouped commands:
    - `ito config <subcommand>`
    - `ito completions <subcommand>`
  - Keep `ito completion ...` as a hidden deprecated shim pointing to `ito completions ...`
  - Keep legacy config verbs (`get/set/unset/reset/edit/path`) as hidden deprecated shims pointing to `ito config ...`
- **Verify**: `ito config --help`, `ito completions --help`
- **Done When**: new groups work end-to-end; shims still work and warn
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 1.3: Keep experimental commands isolated under `x-*`

- **Files**: `src/commands/artifact-workflow.ts`, `src/commands/research.ts`, `src/commands/ralph.ts`
- **Dependencies**: None
- **Action**:
  - Ensure only `x-templates` and `x-schemas` are visible in help
  - Keep other `x-*` callable but hidden
  - Flip `status` and `ralph` to stable (visible), make `x-status`/`x-ralph` hidden deprecated aliases
- **Verify**: `ito --help` shows only the allowed `x-*`
- **Done When**: experimental UX is consistent and does not pollute the stable help surface
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 1.4: Make `ito update` refresh skills

- **Files**: `src/core/update.ts` (and skills configurator as needed)
- **Dependencies**: None
- **Action**:
  - During `ito update`, install/refresh the core skills (same selection policy as init)
  - Extend update summary output to include skills updates
- **Verify**: `ito update` prints updated skills count/paths
- **Done When**: update refreshes skills without requiring explicit CLI skill commands
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 2: Shell Completion Alignment

### Task 2.1: Update completion registry for the preferred surface

- **Files**: `src/core/completions/command-registry.ts`
- **Dependencies**: Wave 1 complete
- **Action**:
  - Ensure completion matches preferred `ito --help` commands
  - Include only visible experimental commands (`x-templates`, `x-schemas`)
  - Omit hidden deprecated shims
- **Verify**: `ito completions generate zsh > /tmp/ito.zsh` (and other shells as needed)
- **Done When**: completion matches the preferred surface and is in sync with help output
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 3: Docs + QA Script Updates

### Task 3.1: Update docs and QA scripts to use preferred commands

- **Files**: `qa/test-ralph-loop.sh` (and any docs referencing legacy commands)
- **Dependencies**: Wave 1 complete
- **Action**:
  - Replace legacy entrypoints with preferred ones (e.g., `ito ralph`)
  - Avoid documenting deprecated shims except as migration notes
- **Verify**: run scripts locally
- **Done When**: docs/scripts demonstrate the preferred verb-first surface
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 4: Verification

### Task 4.1: Run lint/tests/build

- **Files**: (repo-wide)
- **Dependencies**: Waves 1-3 complete
- **Action**: run `make lint`, `make test`, `make build`
- **Verify**: commands succeed
- **Done When**: all checks pass
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 4.2: Validate change artifacts

- **Files**: `.ito/changes/001-05_rationalize-cli-commands/**`
- **Dependencies**: Waves 1-4 complete
- **Action**: `ito validate "001-05_rationalize-cli-commands" --strict --no-interactive`
- **Verify**: validation succeeds
- **Done When**: validation passes
- **Status**: [-] discarded (obsolete - TypeScript migration)
