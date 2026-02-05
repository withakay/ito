## Context

The ralph loop currently passes minimal structured context to agents at each iteration. The prompt consists of the change proposal, module context, and user prompt, but lacks iteration-specific guidance about agent behavior, task management, and completion signals. This results in suboptimal agent performance where agents may:

- Ask questions breaking the autonomous loop
- Forget to update todo lists
- Provide false completion signals
- Lack awareness of iteration progress

The existing context integration system (`--add-context` flag) allows users to inject additional context mid-loop, which is stored in `.ito/.state/ralph/{changeId}/context.txt` and loaded via `loadRalphContext()`. However, this context is simply prepended to the prompt without structure or clear delineation.

## Goals / Non-Goals

**Goals:**

- Provide clear, structured iteration context to agents at the start of each loop
- Embed autonomy requirements to prevent question-asking behavior
- Integrate user-added context as a distinct, labeled section
- Display iteration progress (current/max/min) to help agents understand loop state
- Include explicit instructions for todo list management and completion promises
- Maintain backward compatibility with existing ralph loop behavior

**Non-Goals:**

- Modify the harness layer (OpenCodeHarness, future harnesses)
- Change how context is added via `--add-context` flag
- Alter the state management system or file structure
- Add new CLI flags or options
- Modify the completion promise detection logic

## Decisions

### Decision 1: Create dedicated `buildPromptPreamble()` function

**Chosen:** Separate preamble generation into its own function in `context.ts`

**Rationale:**

- Separation of concerns: preamble logic is distinct from change/module context loading
- Easier to test and maintain
- Allows future customization of preamble without touching context loading
- Clear function signature makes dependencies explicit

**Alternatives Considered:**

- Inline preamble generation in `buildRalphPrompt()`: Would make the function too large and mix concerns
- Generate preamble in runner.ts: Would duplicate logic if multiple harnesses need preambles

### Decision 2: Pass iteration state from runner to prompt builder

**Chosen:** Extend `buildRalphPrompt()` to accept iteration state parameters

**Rationale:**

- Preamble needs iteration count, max iterations, min iterations, and completion promise
- Runner already has this information in the loop (iteration variable `i`, options)
- Avoids loading state file again (performance)
- Makes dependencies explicit in function signature

**Interface:**

```typescript
buildRalphPrompt(
  userPrompt: string,
  options: {
    changeId?: string;
    moduleId?: string;
    iteration?: number;
    maxIterations?: number;
    minIterations?: number;
    completionPromise?: string;
    contextContent?: string | null;
  }
): Promise<string>
```

**Alternatives Considered:**

- Load state file in context.ts: Unnecessary I/O, runner already has the data
- Create separate `RalphIterationState` type: Over-engineering for a simple parameter set

### Decision 3: Structure prompt as: Preamble → Context → Change/Module → User Prompt

**Chosen:** Place preamble first, followed by optional context section, then change/module context, then user prompt

**Rationale:**

- Preamble sets the stage and establishes rules before any task-specific content
- Context section (user-added) appears early but after rules, clearly labeled as "Additional Context"
- Change/module context provides task-specific background
- User prompt comes last as the specific instruction for this iteration
- Separator `---` between major sections for clarity

**Structure:**

```
# Ralph Wiggum Loop - Iteration N
<autonomy rules, instructions, critical rules, iteration progress>

---

## Additional Context (added by user mid-loop)
<user context if present>

---

## Change Proposal (changeId)
<change proposal content>

---

## Module (moduleId)
<module content>

---

<user prompt>
```

**Alternatives Considered:**

- Context after task: Less discoverable, agents might miss it
- Preamble after change/module: Rules would come too late, agent might not read them
- No separators: Harder for agents to parse distinct sections

### Decision 4: Use exact preamble text from user's example

**Chosen:** Implement the preamble structure as shown in the user's example with minor adjustments

**Rationale:**

- User has clearly thought through the requirements and wording
- The example emphasizes autonomy strongly (critical for loop success)
- Specific instructions about todo lists align with OpenCode's capabilities
- Completion promise format is well-defined

**Minor Adjustments:**

- Accept parameters for dynamic values (iteration, max, min, completion promise)
- Extract context section into conditional block (only if context exists)
- Ensure user prompt is passed through, not embedded in preamble

### Decision 5: Move context loading to runner, pass as parameter

**Chosen:** Load context in runner.ts and pass to `buildRalphPrompt()` as parameter

**Rationale:**

- Runner already loads context at line 70 (`loadRalphContext()`)
- Avoids duplicate I/O operations
- Makes `buildRalphPrompt()` more pure (deterministic output from inputs)
- Simplifies testing

**Alternatives Considered:**

- Load context in context.ts: Would require passing changeId, duplicate I/O
- Keep current approach: Context is prepended to full prompt, loses structure

## Risks / Trade-offs

### Risk: Increased token usage

**Impact:** Preamble adds ~300-500 tokens to every iteration

**Mitigation:**

- Preamble is concise and focused
- Token cost is justified by improved agent performance (fewer failed iterations)
- Users can customize via future configuration if needed

### Risk: Preamble conflicts with harness-specific instructions

**Impact:** OpenCode or future harnesses may have their own system prompts that conflict with preamble

**Mitigation:**

- Preamble is user-facing task context, not system-level instructions
- Harnesses control their own system prompts; preamble is part of user prompt
- If conflicts arise, harness can be configured to suppress preamble (future work)

### Risk: Hardcoded text becomes stale

**Impact:** As Ito evolves, preamble wording may become outdated

**Mitigation:**

- Preamble text is in code, easy to find and update
- Future enhancement: move to configuration or template system
- For now, benefits outweigh configuration complexity

### Trade-off: Reduced flexibility for custom prompts

**Impact:** Users can't easily customize preamble text without code changes

**Trade-off Justification:**

- Consistency across iterations is more valuable than customization
- Future enhancement: add `--preamble-template` flag if needed
- Most users benefit from opinionated, well-tested preamble

## Migration Plan

**Deployment:**

1. Update `src/core/ralph/context.ts`:
   - Add `buildPromptPreamble()` function
   - Modify `buildRalphPrompt()` to accept iteration state parameters
   - Call `buildPromptPreamble()` and integrate into prompt structure
1. Update `src/core/ralph/runner.ts`:
   - Pass iteration state to `buildRalphPrompt()` call (line 99)
   - Remove context prepending logic (line 103), now handled in context.ts
1. Update `src/core/ralph/types.ts` if needed (add types for clarity)
1. Write tests for `buildPromptPreamble()` and updated `buildRalphPrompt()`

**Rollback Strategy:**

- Changes are purely additive to prompt content
- If issues arise, revert commits to restore previous prompt structure
- No state migration needed (state files unchanged)

**Testing:**

- Unit tests for preamble generation with various iteration states
- Integration test: run ralph loop, capture prompt, verify structure
- Manual testing: `ito ralph` on existing change, observe agent behavior

## Open Questions

None - design is ready for implementation.
