## Why

The ralph loop currently passes minimal context to agents, resulting in suboptimal agent behavior. Agents lack clear instructions about their autonomous nature, iteration context, completion signals, and task management expectations. This leads to confused agents that may ask questions (breaking the autonomous loop), forget to update todos, or provide false completion signals. A structured preamble system will provide consistent, clear context to agents at each iteration.

## What Changes

- Add a `buildPromptPreamble()` function that generates a structured prompt header for each iteration
- Include iteration count (current/max/min) in the preamble
- Embed autonomy requirements (no questions, no user interaction, self-sufficient decision-making)
- Integrate user-added context (from `--add-context`) into a dedicated section
- Add explicit instructions for todo list management and completion promise usage
- Insert the preamble before the change proposal and user prompt in `buildRalphPrompt()`

## Capabilities

### New Capabilities

- `preamble-generation`: Generate structured iteration context with task instructions, autonomy rules, and completion signals
- `context-integration`: Merge user-added context (from `--add-context`) into the preamble as a distinct section

### Modified Capabilities

<!-- No existing capabilities have changing requirements -->

## Impact

**Affected Code:**

- `src/core/ralph/context.ts` - Add `buildPromptPreamble()` function and modify `buildRalphPrompt()` to include preamble
- `src/core/ralph/runner.ts` - Pass iteration state to prompt builder
- `src/core/ralph/types.ts` - Potentially add types for preamble configuration

**Benefits:**

- Improved agent behavior with clear autonomy expectations
- Better task management through explicit todo list instructions
- Reduced loop failures from agents asking questions or providing false completions
- Consistent agent experience across all iterations

**Risks:**

- Longer prompts may consume more tokens
- Need to ensure preamble doesn't conflict with existing agent instructions
