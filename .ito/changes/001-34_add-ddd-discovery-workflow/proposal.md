<!-- ITO:START -->
## Why

Ito's current workflow is strong at turning a scoped change into proposal artifacts, specs, tasks, and implementation guidance, but it is still weak at domain discovery. In the default path, ubiquitous language, bounded contexts, and domain events are either implicit or pushed into the specialized `event-driven` schema, which means proposal-first work can lock in the wrong vocabulary and the wrong boundaries before the domain has been modeled.

The workflow module already has two adjacent changes in flight: `001-32_add-planning-workflow` adds a lighter pre-proposal planning lane, and `001-33_enhance-spec-driven-workflow-validation` strengthens spec-driven artifacts and validators. This change builds on that direction by making domain discovery first-class and by using DDD techniques to improve how Ito extracts intent, chooses proposal boundaries, and hands domain knowledge forward into specs and tasks.

## What Changes

- Add a DDD-oriented domain discovery workflow that sits between rough planning and proposal scaffolding.
- Define a lightweight discovery bundle for ubiquitous language, bounded contexts, event storming, and proposal-ready handoff summaries.
- Reuse event-storming concepts outside the `event-driven` schema so `spec-driven` changes can still extract commands, domain events, policies, actors, aggregates, and invariants before drafting specs.
- Make the workflow explicitly distinguish bounded contexts from Ito modules and capabilities, and require cross-context changes to declare the affected contexts and their relationships.
- Add optional validation and review hooks for domain-language consistency and boundary consistency so discovery outputs stay connected to proposals, specs, and tasks.

## Change Shape

- **Type**: feature
- **Risk**: medium
- **Stateful**: no
- **Public Contract**: cli
- **Design Needed**: yes
- **Design Reason**: The change crosses planning prompts, schema/template assets, validation rules, and proposal/review guidance, and it needs a clear model for how DDD concepts fit Ito's existing module/capability/artifact system.

## Capabilities

### New Capabilities

- `domain-discovery-workflow`: Provide a DDD-oriented pre-proposal discovery bundle that extracts ubiquitous language, bounded contexts, domain events, commands, policies, and proposal-ready intent summaries.

### Modified Capabilities

- `workflow-convergence`: Extend the canonical instruction-and-skill workflow so ambiguous or architectural work can pass through a domain discovery lane before proposal scaffolding.
- `ito-schemas`: Support reusable discovery artifacts and proposal handoff conventions that bridge planning outputs into spec-driven and event-driven change creation.
- `cli-validate`: Add opt-in validators for ubiquitous-language consistency and cross-context boundary consistency.

## Impact

- Planning, proposal, and review guidance in `ito-rs/crates/ito-templates/assets/instructions/agent/` and related skills/commands under `ito-rs/crates/ito-templates/assets/skills/` and `assets/commands/`.
- Built-in schema assets under `ito-rs/crates/ito-templates/assets/schemas/`, especially `spec-driven` and `event-driven`.
- Validation parsing and diagnostics in `ito-rs/crates/ito-core/src/validate/` and any supporting template/config types.
- Agent-facing docs and workflow diagrams that explain how discovery outputs flow into proposals, specs, tasks, and review.
- Dependency note: implementation should assume or explicitly coordinate with `001-32_add-planning-workflow` and `001-33_enhance-spec-driven-workflow-validation`.
<!-- ITO:END -->
