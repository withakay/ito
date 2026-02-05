# Tasks for: 001-01_flexible-id-parsing

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking

______________________________________________________________________

## Wave 1: ID Parser Implementation

### Task 1.1: Create ID parser utility module

- **Files**: `src/utils/id-parser.ts`
- **Dependencies**: None
- **Action**:
  Create a new utility module with:
  - `parseModuleId(input: string)` - normalizes module IDs to 3-digit format
  - `parseChangeId(input: string)` - normalizes change IDs to `NNN-NN_name` format
  - Both return structured result with canonical form or error
  - Use regex patterns to handle all input variations
- **Verify**: `pnpm test src/utils/id-parser.test.ts`
- **Done When**: Parser functions exported and handle all documented input formats
- \[x\] Task 1.1 complete

### Task 1.2: Write comprehensive parser tests

- **Files**: `src/utils/id-parser.test.ts`
- **Dependencies**: Task 1.1
- **Action**:
  Create test suite covering:
  - Module IDs: `1`, `01`, `001`, `1_foo`, `001_foo`
  - Change IDs: `1-2_bar`, `001-02_bar`, `1-00003_bar`, `0001-00002_baz`
  - Invalid formats: `abc`, `001-02`, `001_02_bar`
  - Edge cases: empty string, null, excessive padding
- **Verify**: `pnpm test src/utils/id-parser.test.ts --coverage`
- **Done When**: All tests pass, coverage >= 90%
- \[x\] Task 1.2 complete

______________________________________________________________________

## Wave 2: CLI Integration

### Task 2.1: Integrate parser into CLI commands

- **Files**: `src/commands/*.ts` (module and change commands)
- **Dependencies**: Task 1.1, Task 1.2
- **Action**:
  Update CLI commands that accept module/change IDs to use the parser:
  - `ito status --change`
  - `ito instructions --change`
  - `ito validate --changes`
  - `ito archive`
  - `ito module` subcommands
  - `ito new change --module`
- **Verify**: `pnpm test && ito status --change 1-1_flexible-id-parsing`
- **Done When**: All commands accept flexible ID formats
- \[x\] Task 2.1 complete

### Task 2.2: Add last-worked-on module tracking

- **Files**: `src/state/last-module.ts`, `src/commands/change.ts`
- **Dependencies**: Task 1.1
- **Action**:
  Implement tracking of last worked-on module:
  - Store last module ID after change creation/modification
  - Retrieve for module selection prompt
  - Use project-local storage (`.ito/.state` or similar)
- **Verify**: `pnpm test src/state/last-module.test.ts`
- **Done When**: Last module ID persisted and retrievable
- \[x\] Task 2.2 complete

______________________________________________________________________

## Wave 3: Skill Enhancement

### Task 3.1: Update ito-proposal skill with interactive module selection

- **Files**: `.opencode/skill/ito-proposal/SKILL.md`
- **Dependencies**: Task 2.2
- **Action**:
  Update skill to include interactive module selection:
  - Add step for prompting when module not specified
  - Document three options: last worked-on, new module, ungrouped (000)
  - Include example prompts and expected responses
- **Verify**: Manual review of skill file
- **Done When**: Skill documents interactive module selection flow
- \[x\] Task 3.1 complete

______________________________________________________________________

## Wave 4: Documentation

### Task 4.1: Update agent-workflow.md with flexible ID formats

- **Files**: `docs/agent-workflow.md`
- **Dependencies**: Task 2.1
- **Action**:
  Add documentation for:
  - ID Format Examples section showing input → canonical conversions
  - Update CLI Commands Reference to mention flexible input
  - Document interactive module selection in Proposal section
- **Verify**: Manual review of docs
- **Done When**: Documentation covers all new features
- \[x\] Task 4.1 complete

______________________________________________________________________

## Wave 5: Validation

### Task 5.1: Run full test suite and lint

- **Files**: All
- **Dependencies**: Task 1.2, Task 2.1, Task 2.2
- **Action**:
  Run comprehensive validation:
  - `pnpm test` - all tests pass
  - `pnpm lint` - no lint errors
  - `pnpm build` - builds successfully
- **Verify**: `pnpm test && pnpm lint && pnpm build`
- **Done When**: All checks pass
- \[x\] Task 5.1 complete

### Task 5.2: End-to-end verification

- **Files**: None (manual testing)
- **Dependencies**: Task 5.1
- **Action**:
  Manual verification:
  - Test `ito status --change 1-1_flexible-id-parsing`
  - Test `/ito-proposal` without module to trigger prompt
  - Verify documentation renders correctly
- **Verify**: Manual testing
- **Done When**: All scenarios work as documented
- \[x\] Task 5.2 complete

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified

## Wave Guidelines

- Waves group related tasks that can be executed in parallel within the wave
- Task dependencies must be complete before starting dependent tasks
- Wave 5 is validation - run after all implementation complete
