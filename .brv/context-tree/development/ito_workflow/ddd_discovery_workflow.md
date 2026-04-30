---
createdAt: '2026-04-30T17:07:58.982Z'
keywords: []
related: [development/ito_workflow/context.md, development/ito_workflow/ito_orchestration_consolidation.md, development/source_guides/source_guide_workflow.md, development/ito_workflow/ddd_discovery_workflow.md]
summary: DDD discovery workflow integrates strategic_ddd_for_coding_agents as non-normative reference material and documents the consensus gates, capability boundary, model ownership, context relationship patterns, consistency requirements, optional queries, and boundary-smell probes.
tags: []
title: DDD Discovery Workflow
updatedAt: '2026-04-30T17:47:41.956Z'
---
## Reason
Capture the consensus discovery workflow concepts and reference material for the DDD discovery change

## Raw Concept
**Task:**
Document the DDD discovery workflow integration for change 001-34_add-ddd-discovery-workflow

**Changes:**
- Integrated grill-with-docs ideas into Ito domain discovery
- Added focused domain-grill questioning for ambiguous and cross-context work
- Required repository evidence before asking user questions
- Added glossary conflict challenges and scenario-based boundary probes
- Introduced lazy CONTEXT.md, CONTEXT-MAP.md, and ADR capture
- Added post-approval domain-doc promotion
- Recorded explicit conflicts with Ito minimal-question and proposal-approval guardrails
- Bundled strategic_ddd_for_coding_agents.md as non-normative reference material
- Integrated the consensus discovery workflow concepts
- Captured the explicit discovery depth gate and boundary-smell probes
- Recorded the gated domain-grill recommendation policy

**Files:**
- .brv/context-tree/development/ito_workflow/context.md
- .brv/context-tree/development/ito_workflow/ddd_discovery_workflow.md

**Flow:**
reference material -> consensus concepts -> discovery depth gate -> capability boundary check -> context relationship patterns -> consistency and optional queries -> boundary-smell probes -> gated domain-grill recommendation

**Timestamp:** 2026-04-30

**Author:** ByteRover context curation

**Patterns:**
- `CONTEXT.md` - Lazy domain context capture file
- `CONTEXT-MAP.md` - Lazy domain map capture file
- `ADR` - Architecture decision record capture during discovery

## Narrative
### Structure
This topic captures the DDD discovery workflow as part of development/ito_workflow, with strategic_ddd_for_coding_agents treated as non-normative reference material rather than a source of additional normative rules.

### Dependencies
The workflow depends on the consensus concepts already identified for the discovery change and on the distinction between business/domain capability and Ito capability.

### Highlights
It preserves the discovery depth gate, model ownership over data/code location, named-or-provisional context relationships, consistency requirements, optional queries, and boundary-smell probes. Rigorous domain-grill is not unconditional; it is gated and automatically recommended only for high-impact ambiguity or when the user explicitly opts in.

### Rules
Rigorous domain-grill is gated but auto-recommended for high-impact ambiguity or explicit user opt-in.

### Examples
Example probes include glossary conflict challenges and scenario-based boundary checks for domain separation.

## Facts
- **reference_material**: Change 001-34_add-ddd-discovery-workflow additionally bundles strategic_ddd_for_coding_agents.md as non-normative reference material. [project]
- **workflow_scope**: The workflow integrates only the consensus workflow concepts. [project]
- **discovery_depth_gate**: The workflow includes an explicit discovery depth gate. [project]
- **capability_boundary**: Business/domain capability is distinct from Ito capability. [project]
- **model_ownership**: Model ownership covers data/code location. [project]
- **context_relationship_patterns**: Named-or-provisional context relationship patterns are part of the workflow. [project]
- **consistency_requirements**: Consistency requirements are included in the workflow. [project]
- **optional_queries**: Optional queries are included in the workflow. [project]
- **boundary_smell_probes**: Boundary-smell probes are included in the workflow. [project]
- **domain_grill_policy**: Rigorous domain-grill is gated but auto-recommended for high-impact ambiguity or explicit user opt-in. [project]
