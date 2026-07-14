---
name: ito-proposal
description: Clarify, research, plan, and scaffold an Ito change proposal before implementation begins.
---

<!-- ITO:START -->
# Proposal lifecycle

Own intake, feature/fix framing, brainstorming, pre-proposal planning, and proposal/spec/design/task scaffolding.

1. Start with intake. Inspect relevant brownfield specs, code, history, and `.ito/wiki/index.md` before asking questions. Ask one focused question at a time and use the least-sufficient discovery depth.
2. Confirm the problem, success criteria, scope, exclusions, risks, and simpler alternatives. Use bounded-context/DDD discovery for cross-context work; reserve a rigorous domain grill for high-impact ambiguity or explicit opt-in.
3. Make module confirmation a mandatory gate. Run `ito list --modules`, then wait for the user to choose an existing module, a new module, or a new sub-module. Never silently use module `000`.
4. Use a research handoff to `ito-research` when evidence or technology trade-offs are unresolved. Bring the cited synthesis back into the proposal.
5. For durable exploration, keep topic-specific artifacts under `.ito/planning/`. `ito plan init` and `ito plan status` remain direct workspace CLI commands; they are not separate skill activations.
6. Render `ito agent instruction schemas`, recommend the best-fit schema, and create the change only after module confirmation:

```bash
ito create change "<change-name>" --module <module-id> --schema <schema>
```

For an existing change ID, skip creation. Generate each authoritative artifact instruction and follow it exactly:

```bash
ito agent instruction proposal --change "<change-id>"
ito agent instruction specs --change "<change-id>"
ito agent instruction design --change "<change-id>"
ito agent instruction tasks --change "<change-id>"
```

Present alternatives and YAGNI trade-offs explicitly. A proposal is ready for review, not implementation; follow the repository's main-first integration policy before handing off to `ito-apply`.
<!-- ITO:END -->
