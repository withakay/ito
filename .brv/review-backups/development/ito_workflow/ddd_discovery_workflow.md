---
title: DDD Discovery Workflow
summary: 'DDD discovery workflow for Ito: focused domain-grill questioning, repository evidence first, glossary and scenario probes, lazy context promotion, and explicit guardrail conflicts.'
tags: []
related: [development/ito_workflow/context.md, development/ito_workflow/ito_orchestration_consolidation.md, development/source_guides/source_guide_workflow.md]
keywords: []
createdAt: '2026-04-30T17:07:58.982Z'
updatedAt: '2026-04-30T17:07:58.982Z'
---
## Reason
Document the new Ito domain discovery workflow that integrates grill-with-docs ideas and clarifies guardrail conflicts.

## Raw Concept
**Task:**
Document the DDD discovery workflow for Ito domain discovery

**Changes:**
- Integrated grill-with-docs ideas into Ito domain discovery
- Added focused domain-grill questioning for ambiguous and cross-context work
- Required repository evidence before asking user questions
- Added glossary conflict challenges and scenario-based boundary probes
- Introduced lazy CONTEXT.md, CONTEXT-MAP.md, and ADR capture
- Added post-approval domain-doc promotion
- Recorded explicit conflicts with Ito minimal-question and proposal-approval guardrails

**Flow:**
ambiguous or cross-context work -> domain-grill questions -> repository evidence review -> conflict checks -> boundary probes -> lazy context capture -> approval -> domain-doc promotion

**Timestamp:** 2026-04-30

**Patterns:**
- `CONTEXT.md` - Lazy domain context capture file
- `CONTEXT-MAP.md` - Lazy domain map capture file
- `ADR` - Architecture decision record capture during discovery

## Narrative
### Structure
This workflow sits inside Ito domain discovery and adds a DDD-oriented questioning phase before context capture and promotion.

### Dependencies
Depends on repository evidence, glossary conflict checks, scenario probes, and post-approval promotion into domain documentation.

### Highlights
The workflow is designed for ambiguous or cross-context work and makes guardrail conflicts explicit so discovery can proceed without over-questioning.

### Rules
Use repository evidence before user questions. Capture context lazily. Promote domain docs only after approval. Respect Ito minimal-question and proposal-approval guardrails while documenting their conflicts with the discovery workflow.

### Examples
Example probes include glossary conflict challenges and scenario-based boundary checks for domain separation.
