# Tasks for: 001-06_promote-x-instructions-to-stable

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking

______________________________________________________________________

## Wave 1: Create agent command infrastructure

### Task 1.1: Create src/commands/agent.ts

- **Files**: `src/commands/agent.ts`
- **Dependencies**: None
- **Action**:
  Create new file `src/commands/agent.ts` with:
  - Import Commander and required dependencies
  - Export `registerAgentCommands(program: Command)` function
  - Create `agent` command group with description: "Commands that generate machine-readable output for AI agents"
  - Add `instruction [artifact]` subcommand with same options as x-instructions (--change, --schema, --json)
  - Delegate to existing `instructionsCommand` function from artifact-workflow.ts
- **Verify**: `bun run build`
- **Done When**: Build passes, new file exists with proper exports
- **Status**: [x] completed

### Task 1.2: Export instructionsCommand from artifact-workflow.ts

- **Files**: `src/commands/artifact-workflow.ts`
- **Dependencies**: None
- **Action**:
  - Export the `instructionsCommand` function (currently private)
  - Ensure function signature is suitable for reuse
- **Verify**: `bun run build`
- **Done When**: Function is exported and can be imported from agent.ts
- **Status**: [x] completed

### Task 1.3: Register agent commands in CLI

- **Files**: `src/cli.ts`
- **Dependencies**: Task 1.1, Task 1.2
- **Action**:
  - Import `registerAgentCommands` from `./commands/agent`
  - Call `registerAgentCommands(program)` to register the agent command group
- **Verify**: `bun run ito agent --help`
- **Done When**: `ito agent` shows help with instruction subcommand
- **Status**: [x] completed

______________________________________________________________________

## Wave 2: Add deprecation and verification

### Task 2.1: Add deprecation warning to x-instructions

- **Files**: `src/commands/artifact-workflow.ts`
- **Dependencies**: Task 1.3
- **Action**:
  - Modify x-instructions command to emit deprecation warning to stderr before executing
  - Warning text: "Warning: ito x-instructions is deprecated, use ito agent instruction"
  - Use `console.error()` so it doesn't interfere with stdout JSON output
- **Verify**: `bun run ito x-instructions proposal --change "001-06_promote-x-instructions-to-stable" 2>&1 | head -1`
- **Done When**: Deprecation warning appears on first line of stderr
- **Status**: [x] completed

### Task 2.2: Verify JSON output not affected by deprecation warning

- **Files**: None (verification only)
- **Dependencies**: Task 2.1
- **Action**:
  - Run x-instructions with --json flag
  - Verify stdout is valid JSON
  - Verify deprecation warning only appears on stderr
- **Verify**: `bun run ito x-instructions proposal --change "001-06_promote-x-instructions-to-stable" --json | jq .`
- **Done When**: JSON parses successfully, warning is only on stderr
- **Status**: [x] completed

### Task 2.3: Verify new command works identically

- **Files**: None (verification only)
- **Dependencies**: Task 1.3
- **Action**:
  - Run `ito agent instruction proposal` and compare output to x-instructions
  - Verify all options work (--change, --schema, --json)
- **Verify**: `diff <(bun run ito agent instruction proposal --change "001-06" 2>/dev/null) <(bun run ito x-instructions proposal --change "001-06" 2>/dev/null)`
- **Done When**: Outputs are identical
- **Status**: [x] completed

______________________________________________________________________

## Wave 3: Update Ito skills

### Task 3.1: Update ito-proposal skill

- **Files**: `src/core/templates/skill-templates.ts` (or wherever ito-proposal template lives)
- **Dependencies**: Task 2.3
- **Action**:
  - Find all references to `ito x-instructions` in the ito-proposal skill template
  - Replace with `ito agent instruction`
- **Verify**: `grep -r "x-instructions" src/core/templates/`
- **Done When**: No references to x-instructions in skill templates
- **Status**: [x] completed

### Task 3.2: Update ito-apply skill

- **Files**: Skill template files
- **Dependencies**: Task 2.3
- **Action**:
  - Find all references to `ito x-instructions` in ito-apply skill
  - Replace with `ito agent instruction`
- **Verify**: `grep -r "x-instructions" .opencode/skill/`
- **Done When**: No references to x-instructions in OpenCode skills
- **Status**: [x] completed

### Task 3.3: Update any documentation references

- **Files**: `docs/`, `README.md`, `.ito/AGENTS.md`
- **Dependencies**: Task 2.3
- **Action**:
  - Search for any documentation referencing x-instructions
  - Update to reference `ito agent instruction`
  - Add note about new agent command group
- **Verify**: `grep -r "x-instructions" docs/ README.md .ito/`
- **Done When**: No outdated references in documentation
- **Status**: [x] completed

______________________________________________________________________

## Wave 4: Tests and validation

### Task 4.1: Add tests for agent command group

- **Files**: `src/commands/__tests__/agent.test.ts`
- **Dependencies**: Task 1.3
- **Action**:
  - Create test file for agent commands
  - Test that `ito agent` shows help
  - Test that `ito agent instruction` generates valid output
  - Test that --json flag produces valid JSON
- **Verify**: `bun test agent`
- **Done When**: All tests pass
- **Status**: [x] completed

### Task 4.2: Run full test suite

- **Files**: None
- **Dependencies**: Task 3.1, Task 3.2, Task 3.3, Task 4.1
- **Action**:
  - Run full test suite to ensure no regressions
  - Fix any failing tests
- **Verify**: `make test`
- **Done When**: All tests pass
- **Status**: [x] completed

### Task 4.3: Manual verification of workflow

- **Files**: None
- **Dependencies**: Task 4.2
- **Action**:
  - Create a test change with `ito create change test-agent --module 000`
  - Run `/ito-proposal` workflow using updated skills
  - Verify instruction generation works correctly
- **Verify**: Manual testing
- **Done When**: Full proposal workflow completes successfully with new command
- **Status**: [x] completed

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified

## Wave Guidelines

- Waves group related tasks that can be executed in parallel
- Task dependencies must be complete before starting dependent tasks
- "after Wave X complete" indicates wave-level dependencies
