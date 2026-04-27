---
name: ito-research
description: "Conduct structured research for feature development, technology evaluation, or problem investigation. Use when the user needs to explore options, analyze trade-offs, or investigate technical approaches."
---

<!-- ITO:START -->


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

Save research outputs under the Ito directory.

Research source artifacts and wiki synthesis have different jobs:

- `$ITO_ROOT/research/{{topic}}/` and `$ITO_ROOT/changes/{{change_id}}/reviews/` store the original investigation or review output.
- `$ITO_ROOT/wiki/` stores durable synthesis, topic links, query artifacts, and freshness notes that help future proposal, research, and archive sessions.
- Do not replace the source research file with a wiki page; cite the source artifact from the wiki page instead.

If you need absolute paths at runtime:

```bash
ITO_ROOT="$(ito path ito-root)"
```

Then save to:

- `$ITO_ROOT/research/{{topic}}/` for feature/technology research
- `$ITO_ROOT/changes/{{change_id}}/reviews/` for change reviews

If `$ITO_ROOT/wiki/index.md` exists, read it before starting to find prior topic pages and known gaps. If coverage is stale, missing, or contradictory, call that out and continue from source research or raw Ito artifacts.

After completing research, update the wiki only when the finding is durable enough to help future sessions:

- Add lasting recommendations, decisions, and cross-cutting findings to relevant topic pages.
- Add one-off investigations or prompt-specific answers under `$ITO_ROOT/wiki/queries/` when they are useful but not topic-page material.
- Update `index.md`, `log.md`, and `_meta/status.md` when wiki coverage changes meaningfully.

## Example Usage

### Technology Research

```
User: Research options for implementing real-time notifications

Agent: I'll use the ito-research skill to evaluate options.

1. First, I'll use @research-stack.md to compare:
   - WebSockets vs SSE vs Polling
   - Library options (socket.io, ws, etc.)

2. Then @research-architecture.md for:
   - Pub/sub patterns
   - Scaling considerations

3. Finally @research-synthesize.md to recommend an approach.
```

### Change Review

```
User: Review the auth refactor change for security issues

Agent: I'll use @review-security.md to audit the change:
- Map attack surface
- Check for auth bypasses
- Verify input validation
- Review cryptographic usage
```

## Integration with Ito Workflow

Research outputs can feed into change proposals:

1. Complete research using templates above
2. Save findings to `$ITO_ROOT/research/{{topic}}/`
3. Reference research in `proposal.md` or `design.md`
4. Update relevant `.ito/wiki/` topic pages or query artifacts when findings have durable reuse value
5. Use research to inform `tasks.md` prioritization

<!-- ITO:END -->
