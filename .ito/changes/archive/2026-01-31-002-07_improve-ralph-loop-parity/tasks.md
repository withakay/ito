# Tasks for: 002-07_improve-ralph-loop-parity

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential

______________________________________________________________________

## Wave 1: Context + Prompt Parity

### Task 1.1: Reload context every iteration

- **Files**: `src/core/ralph/runner.ts`, `src/core/ralph/state.ts`
- **Dependencies**: None
- **Action**:
  - Move context loading into the per-iteration loop so `.ito/.state/ralph/<change-id>/context.md` is read at the start of every iteration
  - Ensure `--add-context` updates are visible on the next iteration without restarting
- **Verify**: Manual run with `ito ralph` + `--add-context` between iterations
- **Done When**: Context changes appear in the next iteration prompt
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 1.2: Add structured preamble + labeled context section

- **Files**: `src/core/ralph/context.ts`
- **Dependencies**: Task 1.1
- **Action**:
  - Add a preamble builder for per-iteration prompts
  - Ensure context (if present) is rendered under `## Additional Context (added by user mid-loop)`
  - Keep existing change/module context injection behavior
- **Verify**: Unit tests for prompt assembly
- **Done When**: Prompts consistently include preamble + labeled context section
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 2: Loop Robustness

### Task 2.1: Make completion promise detection whitespace-tolerant

- **Files**: `src/core/ralph/runner.ts`
- **Dependencies**: None
- **Action**:
  - Replace exact string matching for `<promise>...</promise>` with a tolerant matcher that ignores surrounding whitespace/newlines
  - Maintain `--min-iterations` semantics
- **Verify**: Unit tests covering `<promise>\nCOMPLETE\n</promise>` formats
- **Done When**: Completion detection works across common formatting variations
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 2.2: Continue loop on non-zero harness exit

- **Files**: `src/core/ralph/harnesses/opencode.ts`, `src/core/ralph/runner.ts`, `src/core/ralph/types.ts`
- **Dependencies**: Task 2.1
- **Action**:
  - Change harness execution to return an exit code/result object instead of throwing on non-zero exits
  - Update runner to record failures in history and proceed to next iteration
  - Add a fail-fast option (CLI flag) that preserves stop-on-error behavior
- **Verify**: Manual run where harness intentionally exits non-zero; ensure loop continues and status records exit code
- **Done When**: Loop continues on failure unless fail-fast is enabled
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 3: History + Status Improvements

### Task 3.1: Expand history record schema

- **Files**: `src/core/ralph/types.ts`, `src/core/ralph/state.ts`, `src/core/ralph/runner.ts`
- **Dependencies**: Task 2.2
- **Action**:
  - Extend iteration history to include: exitCode, completionPromiseFound, duration, changedFilesCount
  - Optionally record commit hash when auto-commit is enabled
- **Verify**: `ito ralph --status` shows new fields
- **Done When**: History file persists richer telemetry per iteration
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 3.2: Improve status output formatting

- **Files**: `src/core/ralph/runner.ts` (status path)
- **Dependencies**: Task 3.1
- **Action**:
  - Update `--status` output to include exit code and a brief change summary per recent iteration
- **Verify**: `ito ralph --status --change <id>`
- **Done When**: Status output surfaces useful debug info at a glance
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 4: CLI Parity Flags

### Task 4.1: Add --prompt-file option

- **Files**: `src/commands/ralph.ts`
- **Dependencies**: None
- **Action**:
  - Add `--prompt-file <path>` flag to load the prompt from a file
  - Define behavior if both a positional prompt and `--prompt-file` are provided (prefer file or error)
- **Verify**: `ito ralph --prompt-file /path/to/prompt.txt --change <id>`
- **Done When**: Prompt can be loaded from a file
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 4.2: Add --no-stream option

- **Files**: `src/commands/ralph.ts`, `src/core/ralph/harnesses/opencode.ts`, `src/core/ralph/types.ts`
- **Dependencies**: Task 2.2
- **Action**:
  - Add `--no-stream` flag to disable live harness output streaming
  - Ensure output is still captured for completion detection and history
- **Verify**: Manual run with `--no-stream` and completion detection
- **Done When**: Loop runs without streaming, still functions correctly
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 5: Tests + Validation

### Task 5.1: Add unit tests for completion detection and prompt assembly

- **Files**: `test/core/ralph/*.test.ts`
- **Dependencies**: Task 1.2, Task 2.1
- **Action**:
  - Add tests for prompt structure (preamble + context + change/module)
  - Add tests for whitespace-tolerant completion detection
- **Verify**: `make test`
- **Done When**: Tests pass
- **Status**: [-] discarded (obsolete - TypeScript migration)

### Task 5.2: Validate change artifacts

- **Files**: N/A
- **Dependencies**: All above
- **Action**:
  - Run `ito validate 002-07_improve-ralph-loop-parity --strict` and fix any issues
- **Verify**: Validation passes
- **Done When**: `ito validate --strict` is clean
- **Status**: [-] discarded (obsolete - TypeScript migration)
