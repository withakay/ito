## Key points
- The document defines a **DDD discovery workflow** for change `001-34_add-ddd-discovery-workflow`, integrating consensus discovery concepts into the Ito domain discovery process.
- `strategic_ddd_for_coding_agents.md` is explicitly treated as **non-normative reference material**, not a source of new rules.
- The workflow emphasizes an **explicit discovery depth gate** and a clear **capability boundary** between business/domain capability and Ito capability.
- It captures several core DDD concerns: **model ownership** over data/code location, **context relationship patterns**, **consistency requirements**, **optional queries**, and **boundary-smell probes**.
- **Rigorous domain-grill questioning** is not unconditional; it is **gated** and auto-recommended only for **high-impact ambiguity** or when the **user opts in**.
- Additional discovery techniques include **glossary conflict challenges**, **scenario-based boundary probes**, and requiring **repository evidence before asking user questions**.
- The change also introduces lazy capture artifacts such as **CONTEXT.md**, **CONTEXT-MAP.md**, and **ADR** records during discovery.

## Structure / sections summary
- **Metadata**: title, summary, tags, related documents, created/updated timestamps.
- **Reason**: states the purpose as capturing consensus discovery workflow concepts for the DDD discovery change.
- **Raw Concept**:
  - Lists the task, change set, files, workflow flow, timestamp, author, and recurring patterns.
- **Narrative**:
  - **Structure**: positions the workflow within `development/ito_workflow` and clarifies reference material handling.
  - **Dependencies**: ties the workflow to consensus concepts and domain vs. Ito capability distinction.
  - **Highlights**: summarizes the preserved discovery elements and gating behavior.
  - **Rules**: restates the gated recommendation policy for rigorous domain-grill.
  - **Examples**: mentions glossary conflicts and scenario-based boundary checks.
- **Facts**: enumerates the normalized project facts and policy decisions.

## Notable entities, patterns, or decisions
- **Entities / files**
  - `development/ito_workflow/context.md`
  - `development/ito_workflow/ddd_discovery_workflow.md`
  - `strategic_ddd_for_coding_agents.md`
  - `CONTEXT.md`, `CONTEXT-MAP.md`, `ADR`
- **Patterns**
  - Lazy capture of domain artifacts
  - Named-or-provisional context relationships
  - Boundary-smell probing
  - Glossary conflict and scenario-based boundary checks
- **Decisions**
  - Reference material is **non-normative**
  - Discovery is **consensus-based**
  - Domain-grill is **gated**, not default
  - Automatic recommendation applies only under **high-impact ambiguity** or **explicit opt-in**