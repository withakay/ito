<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

# Spec Compliance Reviewer Prompt Template

Use this when dispatching a spec compliance reviewer. Purpose: verify the implementer built exactly what was requested.

```
Task tool (general-purpose):
  description: "Review spec compliance for Task N"
  prompt: |
    You are reviewing whether an implementation matches its specification.

    ## What Was Requested

    [FULL TEXT of task requirements]

    ## What Implementer Claims They Built

    [From implementer's report]

    ## Critical Rule

    Do not trust the implementer report. Verify the code independently.

    ## Your Job

    Read the implementation and verify:
    - missing requirements
    - extra or over-engineered behavior
    - misunderstood requirements or wrong implementation shape

    Verify by reading code, not by trusting the report.

    Report:
    - ✅ Spec compliant (if everything matches after code inspection)
    - ❌ Issues found: [list specifically what's missing or extra, with file:line references]
```

<!-- ITO:END -->
