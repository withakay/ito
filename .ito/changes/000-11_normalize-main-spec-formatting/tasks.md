# Tasks for: 000-11_normalize-main-spec-formatting

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Prefer the tasks CLI to drive status updates and pick work

```bash
ito tasks status 000-11_normalize-main-spec-formatting
ito tasks next 000-11_normalize-main-spec-formatting
ito tasks start 000-11_normalize-main-spec-formatting 1.1
ito tasks complete 000-11_normalize-main-spec-formatting 1.1
ito tasks shelve 000-11_normalize-main-spec-formatting 1.1
ito tasks unshelve 000-11_normalize-main-spec-formatting 1.1
ito tasks show 000-11_normalize-main-spec-formatting
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add the `spec-formatting` capability spec

- **Files**: `.ito/changes/000-11_normalize-main-spec-formatting/specs/spec-formatting/spec.md`
- **Dependencies**: None
- **Action**: Define the canonical main spec structure and normalization rules as a new capability.
- **Verify**: `ito validate 000-11_normalize-main-spec-formatting --strict`
- **Done When**: The change validates and the requirements clearly define the desired end state.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 1.2: Inventory main specs that need normalization

- **Files**: `.ito/specs/**/spec.md`
- **Dependencies**: None
- **Action**:
  - Identify all main specs that start with delta operation sections (`## ADDED|MODIFIED|REMOVED|RENAMED Requirements`).
  - Identify all specs missing `#` title and/or `## Purpose`.
  - Identify all specs with placeholder `TBD` purpose text.
- **Verify**: `rg -n "^## (ADDED|MODIFIED|REMOVED|RENAMED) Requirements" .ito/specs`
- **Done When**: A deterministic list of files and required edits exists (prefer checked-in notes under the change if needed).
- **Updated At**: 2026-02-25
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Normalize `.ito/specs/**/spec.md` to canonical structure

- **Files**: `.ito/specs/**/spec.md`
- **Dependencies**: None
- **Action**:
  - Convert delta-style main specs to use `## Requirements`.
  - Standardize titles and add missing `## Purpose` sections.
  - Replace `TBD` purpose placeholders with meaningful text.
  - Preserve all `### Requirement:` and `#### Scenario:` blocks (no semantic changes).
- **Verify**: `ito validate --strict`
- **Done When**: `ito validate --strict` passes and the spec tree has a consistent human-readable structure.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

### Task 2.2: Regression check: truth vs delta remains unambiguous

- **Files**: `.ito/specs/**`, `.ito/changes/**/specs/**`
- **Dependencies**: Task 2.1
- **Action**: Ensure main specs do not contain delta operation headings and change deltas still do.
- **Verify**:
  - `rg -n "^## (ADDED|MODIFIED|REMOVED|RENAMED) Requirements" .ito/specs` (should be empty)
  - `rg -n "^## (ADDED|MODIFIED|REMOVED|RENAMED) Requirements" .ito/changes` (should have matches)
- **Done When**: Searches confirm the intended separation.
- **Updated At**: 2026-02-25
- **Status**: [x] complete

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add validation for delta-style formatting in `.ito/specs/`

- **Files**: `ito-rs/**` (validator implementation + tests), `.ito/specs/spec-formatting/spec.md` (if needed)
- **Dependencies**: None
- **Action**: Add a validator that flags delta operation headings in main specs (warning by default, strict error).
- **Verify**: `make check`
- **Done When**: Validation catches regressions where delta-only structure is reintroduced into `.ito/specs/`.
- **Updated At**: 2026-02-25
- **Status**: [-] shelved

______________________________________________________________________

## Checkpoints

### Checkpoint: Proposal Review

- **Type**: checkpoint (requires human approval)
- **Dependencies**: None
- **Action**: Review the proposal and the `spec-formatting` delta spec.
- **Done When**: User approves the proposal.
- **Updated At**: 2026-02-25
- **Status**: [x] complete
