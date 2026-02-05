# Tasks for: 002-06_add-agent-preamble-system

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking

______________________________________________________________________

## Wave 1: Core Preamble Generation

### Task 1.1: Add buildPromptPreamble function

- **Files**: `src/core/ralph/context.ts`
- **Dependencies**: None
- **Action**:
  - Add `buildPromptPreamble()` function that accepts:
    - `iteration: number`
    - `maxIterations: number | undefined`
    - `minIterations: number`
    - `completionPromise: string`
    - `contextContent: string | null`
  - Implement preamble structure with sections:
    - Title: "# Ralph Wiggum Loop - Iteration N"
    - Optional context section (if contextContent exists): "## Additional Context (added by user mid-loop)"
    - Task section placeholder: "## Your Task" (content added by caller)
    - Instructions section: 5 numbered steps
    - Critical rules section: 6 bullet points
    - Autonomy requirements section: 7 bullet points with emphasis
    - Current iteration display: "Current Iteration: N / MAX (min: MIN)" or "N (unlimited) (min: MIN)"
  - Return formatted string with proper spacing and separators
- **Verify**: TypeScript compiles without errors
- **Done When**: Function exists and returns formatted preamble string
- **Status**: \[x\] complete

### Task 1.2: Update buildRalphPrompt signature

- **Files**: `src/core/ralph/context.ts`
- **Dependencies**: Task 1.1
- **Action**:
  - Modify `buildRalphPrompt()` function signature to accept additional parameters:
    - `iteration?: number`
    - `maxIterations?: number`
    - `minIterations?: number`
    - `completionPromise?: string`
    - `contextContent?: string | null`
  - Update function to call `buildPromptPreamble()` when iteration is provided
  - Restructure prompt assembly:
    1. If iteration provided, start with preamble
    1. Add context section from preamble (already embedded)
    1. Add change context if changeId provided
    1. Add module context if moduleId provided
    1. Add user prompt at the end
  - Use `\n\n---\n\n` separator between major sections
  - Remove old context prepending (now handled in preamble)
- **Verify**: TypeScript compiles without errors
- **Done When**: Function signature updated, preamble integrated into prompt structure
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 2: Runner Integration

### Task 2.1: Update runner to pass iteration state

- **Files**: `src/core/ralph/runner.ts`
- **Dependencies**: Task 1.2
- **Action**:
  - At line 99-103, update `buildRalphPrompt()` call to pass:
    - `iteration: i` (loop variable)
    - `maxIterations: maxIterations` (from options)
    - `minIterations: minIterations` (from options)
    - `completionPromise: completionPromise` (from options)
    - `contextContent: contextContent` (already loaded at line 70)
  - Remove line 103 that prepends contextContent (now handled in buildRalphPrompt)
  - Update variable name from `fullPrompt` to `prompt` (no longer combining here)
- **Verify**: TypeScript compiles without errors
- **Done When**: Runner passes all iteration state to buildRalphPrompt, context no longer prepended in runner
- **Status**: \[x\] complete

______________________________________________________________________

## Wave 3: Testing

### Task 3.1: Add unit tests for buildPromptPreamble

- **Files**: `test/core/ralph/context.test.ts` (create if doesn't exist)
- **Dependencies**: Task 1.1
- **Action**:
  - Test preamble generation with various iteration states:
    - Iteration 1 with max iterations
    - Iteration 5 without max iterations (unlimited)
    - With and without context content
    - Completion promise embedded correctly
  - Verify all sections present (title, task, instructions, critical rules, autonomy)
  - Verify context section only appears when contextContent provided
  - Verify iteration display formats correctly
- **Verify**: `bun test test/core/ralph/context.test.ts`
- **Done When**: Tests pass, all scenarios covered
- **Status**: \[x\] complete

### Task 3.2: Add integration test for full prompt structure

- **Files**: `test/core/ralph/context.test.ts`
- **Dependencies**: Task 1.2
- **Action**:
  - Test `buildRalphPrompt()` with iteration parameters:
    - Mock change and module context files
    - Call with iteration, maxIterations, minIterations, completionPromise, contextContent
    - Verify prompt structure: preamble → context → change → module → user prompt
    - Verify separators between sections
    - Test with and without changeId, moduleId, iteration
  - Test backward compatibility (no iteration parameters)
- **Verify**: `bun test test/core/ralph/context.test.ts`
- **Done When**: Tests pass, prompt structure verified
- **Status**: \[x\] complete

### Task 3.3: Manual testing with ralph loop

- **Files**: N/A (testing)
- **Dependencies**: Task 2.1
- **Action**:
  - Create a test change: `ito create change "test-preamble" --module 000`
  - Add simple proposal.md
  - Run: `ito ralph "Add a hello world function" --change test-preamble --min-iterations 1 --max-iterations 1`
  - Observe agent behavior:
    - Does agent update todo list?
    - Does agent avoid asking questions?
    - Does agent output completion promise?
    - Is iteration count visible?
  - Verify preamble appears in agent context (check OpenCode logs or output)
- **Verify**: Manual observation
- **Done When**: Ralph loop runs successfully with preamble, agent behavior improved
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Wave 4: Type Safety (Optional Enhancement)

### Task 4.1: Add type for iteration state

- **Files**: `src/core/ralph/types.ts`
- **Dependencies**: None
- **Action**:
  - Add `RalphIterationState` interface:
    ```typescript
    export interface RalphIterationState {
      iteration: number;
      maxIterations?: number;
      minIterations: number;
      completionPromise: string;
      contextContent?: string | null;
    }
    ```
  - Update `buildPromptPreamble()` and `buildRalphPrompt()` to use this type
  - Update runner to construct and pass `RalphIterationState`
- **Verify**: TypeScript compiles without errors
- **Done When**: Type added, functions updated to use it
- **Status**: [-] discarded (obsolete - TypeScript migration)

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified

## Wave Guidelines

- Waves group related tasks that can be executed in parallel
- Task dependencies must be complete before starting dependent tasks
- "after Wave X complete" indicates wave-level dependencies
- Wave 4 is optional and can be skipped if type safety is not a priority

## Task Structure

Each task should include:

- **ID**: Unique identifier (wave.task)
- **Files**: Which files this task affects
- **Dependencies**: Other tasks that must complete first (or "None")
- **Action**: What to implement or do
- **Verify**: Command to verify completion (optional but recommended)
- **Done When**: Acceptance criteria
- **Status**: Current status (pending/in-progress/complete)
