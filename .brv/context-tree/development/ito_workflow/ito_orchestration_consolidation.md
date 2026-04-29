---
title: Ito Orchestration Consolidation
summary: Ito orchestration consolidation is folded into change 028-02, introducing agent-surface-taxonomy and making ito agent instruction orchestrate the authoritative source for overlapping orchestration and multi-agent skills/prompts.
tags: []
related: []
keywords: []
createdAt: '2026-04-29T11:58:17.438Z'
updatedAt: '2026-04-29T11:58:17.438Z'
---
## Reason
Document consolidation of orchestration proposal into existing change 028-02 and taxonomy for agent surfaces

## Raw Concept
**Task:**
Document the consolidation of the Ito orchestration proposal into existing change 028-02_centralize-instruction-source-of-truth.

**Changes:**
- Folded the orchestration consolidation proposal into change 028-02 instead of creating a duplicate.
- Added agent-surface-taxonomy to distinguish direct entrypoint agents from delegated role sub-agents.
- Scoped consolidation of overlapping orchestration and multi-agent skills/prompts behind ito agent instruction orchestrate as the authoritative source.

**Files:**
- .claude/agents/ito-general.md
- .claude/agents/ito-orchestrator.md
- .claude/skills/ito-orchestrate/
- .claude/skills/ito-test-with-subagent/

**Flow:**
proposal -> fold into existing change -> add agent-surface-taxonomy -> designate authoritative orchestration source

**Timestamp:** 2026-04-29

**Author:** ByteRover context engineering

**Patterns:**
- `^028-02_centralize-instruction-source-of-truth$` - Existing change identifier used as the consolidation target

## Narrative
### Structure
This note captures a consolidation decision within the Ito workflow domain. It distinguishes entrypoint agents such as ito-general and ito-orchestrator from delegated sub-agents like planner, researcher, worker, reviewer, and test-runner.

### Dependencies
Depends on the existing 028-02 change and the overlap between orchestration-related skills and prompts that were being consolidated.

### Highlights
The authoritative source for orchestration is the ito agent instruction orchestrate path, preventing duplication across related agent instructions and skills.
