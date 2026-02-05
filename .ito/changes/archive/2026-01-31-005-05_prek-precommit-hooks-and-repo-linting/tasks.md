# Tasks for: 005-05_prek-precommit-hooks-and-repo-linting

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential (or parallel if tool supports)
- **Created**: 2026-01-31

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Choose hook set + formatter strategy

- **Files**: `.ito/changes/005-05_prek-precommit-hooks-and-repo-linting/design.md`
- **Dependencies**: None
- **Action**:
  Decide which hook sources to use for Markdown/JSON/YAML formatting/validation and document the decision (including rationale and local dependency expectations).
- **Verify**: N/A
- **Done When**: `design.md` reflects a concrete decision (no longer an open question)
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 1.2: Add prek-compatible `.pre-commit-config.yaml`

- **Files**: `.pre-commit-config.yaml`
- **Dependencies**: Task 1.1
- **Action**:
  Add a `.pre-commit-config.yaml` that is runnable by `prek` and includes hooks for whitespace/line endings, JSON/YAML validation, and Rust formatting + clippy.
- **Verify**: `prek run --all-files`
- **Done When**: `prek run --all-files` succeeds on a clean tree
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 1.3: Document prek usage for contributors

- **Files**: `README.md` (and/or `ito-rs/README.md` if more appropriate)
- **Dependencies**: Task 1.2
- **Action**:
  Document how to install/run `prek` (`prek run`, `prek install`, and the "already using pre-commit" migration note).
- **Verify**: N/A
- **Done When**: README clearly describes the intended workflow and commands
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Define and implement clippy lint policy

- **Files**: `Makefile`, `ito-rs/**`, (optional) `clippy.toml`
- **Dependencies**: None
- **Action**:
  Define a curated clippy policy aligned with repo Rust style guidance (enable/select additional lints as appropriate, document escape hatches, and add any necessary configuration).
- **Verify**: `make lint`
- **Done When**: `make lint` enforces the policy and passes on a clean tree
- **Updated At**: 2026-01-31
- **Status**: [x] complete

### Task 2.2: Ensure hook + make targets are consistent

- **Files**: `.pre-commit-config.yaml`, `Makefile`
- **Dependencies**: Task 2.1
- **Action**:
  Ensure the Rust hook commands match the supported repo commands (`make lint`/`cargo fmt`/`cargo clippy`) so contributors and CI get the same results.
- **Verify**: `prek run --all-files` and `make lint`
- **Done When**: Both commands run the same checks and succeed on a clean tree
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add CI parity (optional but recommended)

- **Files**: `.github/workflows/**` (or the repo's CI configuration)
- **Dependencies**: None
- **Action**:
  Add a CI step to run `prek run --all-files` (or equivalent) so CI matches local hooks.
- **Verify**: CI green
- **Done When**: CI runs the prek checks and fails on hook violations
- **Updated At**: 2026-01-31
- **Status**: [x] complete

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Final validation + docs check

- **Files**: N/A
- **Dependencies**: None
- **Action**:
  Run the full local verification set and fix any issues.
- **Verify**: `ito validate 005-05_prek-precommit-hooks-and-repo-linting --strict`
- **Done When**: Ito strict validation succeeds and repo checks pass
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
- **Status**: [x] completed
