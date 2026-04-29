<!-- ITO:START -->

# Implementer Subagent Prompt Template

Use this when dispatching an implementer subagent.

```
Task tool (general-purpose):
  description: "Implement Task N: [task name]"
  prompt: |
    You are implementing Task N: [task name]

    ## Task Description

    [FULL TEXT of task from plan - paste it here, don't make subagent read file]

    ## Context

    [Scene-setting: where this fits, dependencies, architectural context]

    ## Before You Begin

    If requirements, approach, dependencies, or assumptions are unclear, ask before starting.

    ## Your Job

    Once requirements are clear:
    1. Implement exactly what the task specifies
    2. Write tests (use TDD when required)
    3. Verify the implementation
    4. Commit your work
    5. Self-review
    6. Report back

    Work from: [directory]

    If something unexpected or unclear appears, ask instead of guessing.

    ## Before Reporting Back: Self-Review

    Review your work with fresh eyes:
    - completeness: all requirements met, no obvious edge-case gaps
    - quality: clear names, maintainable code, consistent patterns
    - discipline: no overbuilding, no unrequested behavior
    - testing: behavior verified, TDD followed when required

    If you find issues during self-review, fix them now before reporting.

    ## Report Format

    When done, report:
    - What you implemented
    - What you tested and test results
    - Files changed
    - Self-review findings (if any)
    - Any issues or concerns
```

<!-- ITO:END -->
