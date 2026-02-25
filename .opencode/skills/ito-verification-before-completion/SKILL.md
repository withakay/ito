---
name: ito-verification-before-completion
description: Use before claiming work is complete, fixed, or passing — requires running verification commands and confirming output before making success claims
---

# Verification Before Completion

## The Rule

**Never claim success without evidence.** Before stating that something works, passes, or is fixed, you MUST run the relevant command and observe the output yourself.

This applies to:
- Test results ("all tests pass")
- Build status ("builds successfully")
- Bug fixes ("the issue is resolved")
- Task completion ("task X is done")

## Required Process

1. **Run the command** — execute the actual test, build, or verification step
2. **Read the output** — confirm it shows what you expect
3. **Quote the evidence** — include the relevant output in your response
4. **Then claim success** — only after steps 1-3

## Red Flags — Stop and Verify

- You're about to say "should work" or "should pass" — run it instead
- You fixed code but haven't re-run the failing test
- You're about to commit without running the test suite
- You completed a task but didn't verify the acceptance criteria
- You're reasoning about what the output "would be" instead of checking

## Common Traps

| Trap | Fix |
|---|---|
| "The fix is straightforward, tests should pass" | Run the tests |
| "I've seen this pattern work before" | Run it anyway |
| "Only a small change, low risk" | Small changes break things too |
| "Tests passed earlier, this change is safe" | Re-run after every change |
| "I'll verify at the end" | Verify at each step |

## Integration with Ito Workflow

- Before `ito tasks complete`: verify the task's acceptance criteria
- Before claiming a change is ready for review: run the full test suite
- Before `ito archive`: confirm all specs are met with evidence
