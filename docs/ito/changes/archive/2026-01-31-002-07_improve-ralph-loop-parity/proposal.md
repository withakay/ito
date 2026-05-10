## Why

Ito's `ralph` loop is functional but missing several "operator ergonomics" and "agent shepherding" behaviors that exist in the standalone `opencode-ralph-wiggum/ralph.ts`. The gap reduces agent effectiveness (less structured guidance and context), makes the loop brittle (non-zero harness exits halt progress), and limits debuggability (minimal history/telemetry). Closing these gaps will make `ito ralph` more reliable for long-running autonomous work on change proposals.

## What Changes

- Add richer prompt structure for loop iterations (preamble + clearly labeled context), bringing `ito ralph` closer to the standalone loop behavior
- Reload and render user-provided context on every iteration so mid-loop context updates take effect without restarting the loop
- Improve completion promise detection to be whitespace-tolerant (and more robust than exact string match)
- Make the loop resilient to harness non-zero exits by recording failures and continuing (with a fail-fast option)
- Expand per-iteration history to record exit codes, git changes/commit info, and other useful telemetry
- Add CLI parity flags to improve operator control (prompt file input, streaming control, and verbosity options)

## Capabilities

### New Capabilities

<!-- None. This change extends the existing ralph loop behavior. -->

### Modified Capabilities

- `cli-ralph`: extend loop prompting, context handling, error resilience, history/telemetry, and CLI options

## Impact

**Affected code:**

- `src/commands/ralph.ts`
- `src/core/ralph/runner.ts`
- `src/core/ralph/context.ts`
- `src/core/ralph/state.ts`
- `src/core/ralph/types.ts`
- `src/core/ralph/harnesses/opencode.ts`

**Behavioral impact:**

- The loop becomes more robust by continuing on harness failures by default (with an opt-in fail-fast mode)
- Prompt content becomes more structured and consistent per iteration
- Status/history output becomes more informative for debugging and progress tracking

**Risks:**

- Larger prompts increase token usage per iteration
- Continuing on error may hide problems if operators expect fail-fast behavior (mitigated by explicit flags and improved reporting)
