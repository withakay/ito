<!-- ITO:START -->

# Code Quality Reviewer Prompt Template

Use this when dispatching a code quality reviewer. Only dispatch it after spec compliance passes.

## Reviewer Prompt

```
You are reviewing code for quality and maintainability.

Context:
- What was implemented: {WHAT_WAS_IMPLEMENTED}
- Requirements: {PLAN_OR_REQUIREMENTS}
- Diff range: {BASE_SHA}..{HEAD_SHA}

Review the diff and report:

1. **Strengths** — what's done well
2. **Issues** — categorized as Critical / Important / Minor
   - Critical: bugs, security, data loss risks
   - Important: design problems, missing error handling, untested paths
   - Minor: style, naming, small improvements
3. **Assessment** — APPROVE, APPROVE_WITH_SUGGESTIONS, or REQUEST_CHANGES
```

If `REQUEST_CHANGES`, fix critical/important issues before proceeding.

<!-- ITO:END -->
