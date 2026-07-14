---
name: ito-proposal
description: Clarify, research, plan, scaffold, review, and integrate an Ito change proposal before implementation. Use when asked to propose, plan, scope, design, or prepare a feature or fix.
---

<!-- ITO:START -->
# Proposal lifecycle

Own intake, feature/fix framing, brainstorming, pre-proposal planning, and proposal/spec/design/task scaffolding.

1. Inspect relevant brownfield specs, code, history, and `.ito/wiki/index.md`. Ask one focused question at a time and use the least-sufficient discovery depth.
2. Confirm the problem, success criteria, scope, exclusions, risks, simpler alternatives, and YAGNI trade-offs. Use `ito-research` when evidence or technology choices remain unresolved.
3. Make module confirmation mandatory. Run `ito list --modules`, then wait for the user to choose an existing module, a new module, or a new sub-module. Never silently use module `000`.
4. Keep durable exploration under `.ito/planning/`. `ito plan init` and `ito plan status` are direct workspace commands, not separate skills.
5. Render `ito agent instruction schemas`, recommend the best-fit schema, and create the change only after module confirmation:

   ```bash
   ito create change "<change-name>" --module <module-id> --schema <schema>
   ```

6. For an existing change, skip creation. Render every authoritative artifact instruction and follow it exactly:

   ```bash
   ito agent instruction proposal --change "<change-id>"
   ito agent instruction specs --change "<change-id>"
   ito agent instruction design --change "<change-id>"
   ito agent instruction tasks --change "<change-id>"
   ```

7. Run `ito validate <change-id> --strict` and review the proposal-only package. Integrate it using `changes.proposal.integration_mode`:

   - `pull_request` (default): push, review, and merge a proposal-only PR.
   - `direct_merge`: after explicit approval, merge the proposal-only commit through the repository's guarded Git workflow.

Do not implement, start tasks, or launch workers from the proposal branch. Verify the handoff with `ito change preflight "<change-id>" --for prepare --refresh`, then hand the integrated proposal to `ito-apply`.
<!-- ITO:END -->
