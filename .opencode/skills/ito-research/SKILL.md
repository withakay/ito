---
name: ito-research
description: "Conduct structured research for feature development, technology evaluation, or problem investigation. Use when the user needs to explore options, analyze trade-offs, or investigate technical approaches."
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->


# Ito Research

Use this skill for technology evaluation, feature research, proposal review, and recommendation synthesis.

## Template Map

| Goal | Template |
|---|---|
| Stack / library choice | @research-stack.md |
| Feature landscape / competitor scan | @research-features.md |
| Architecture / pattern design | @research-architecture.md |
| Pitfalls / anti-patterns | @research-pitfalls.md |
| Final recommendation | @research-synthesize.md |
| Security review | @review-security.md |
| Scale / performance review | @review-scale.md |
| Edge-case review | @review-edge.md |

## Workflow

- For new features/tech: stack → features → architecture → pitfalls → synthesis.
- For proposal review: security and/or scale and/or edge-case review, depending on risk.

## Output Location

Save research under the Ito directory. For absolute paths:

```bash
ITO_ROOT="$(ito path ito-root)"
```

Then save to:

- `$ITO_ROOT/research/{{topic}}/` for feature/technology research
- `$ITO_ROOT/changes/{{change_id}}/reviews/` for change reviews

## Integration with Ito Workflow

Research outputs can feed proposals and designs: run the relevant templates, save the results, reference them in `proposal.md` / `design.md`, and use them to shape `tasks.md`.

<!-- ITO:END -->
