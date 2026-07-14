---
name: ito-review
description: Review proposals, code, specs, tests, and completion evidence against the accepted Ito contract.
---

<!-- ITO:START -->
# Review lifecycle

Determine the change ID and render the authoritative review instruction:

```bash
ito agent instruction review --change "<change-id>"
```

Review proposal/spec compliance before implementation quality. Trace all acceptance criteria to observed code, tests, documentation, or CLI behavior. Run current verification commands; never claim success from stale output, intention, or unchecked task status.

Use independent review passes for non-trivial changes. Native test-runner or specialist agents may be delegated only where the harness exposes a genuine agent surface; they are not required skill fallbacks. Fix critical and important findings, then rerun the affected evidence.

Report findings with `[blocking]`, `[suggestion]`, or `[note]`, followed by `Verdict: approve`, `Verdict: request-changes`, or `Verdict: needs-discussion`. Completion evidence must include the exact checks run, meaningful results, unresolved risks, and acceptance-criteria coverage.
<!-- ITO:END -->
