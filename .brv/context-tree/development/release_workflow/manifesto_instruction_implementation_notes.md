---
createdAt: '2026-04-28T05:19:48.096Z'
keywords: []
related: [development/release_workflow/build_and_coverage_guardrails.md, development/release_workflow/release_workflow.md]
summary: Manifesto instruction rendering only reports synced_at_generation on Synchronized syncs; RateLimited is not fresh success; full --operation requires --change; unconfigured operations render null.
tags: []
title: Manifesto Instruction Implementation Notes
updatedAt: '2026-04-28T05:19:48.096Z'
---
## Reason
Document constraints for manifesto instruction rendering and sync status reporting

## Raw Concept
**Task:**
Document manifesto instruction implementation notes for sync reporting and operation rendering

**Changes:**
- Clarified that synced_at_generation only records successful synchronization
- Defined RateLimited as an absence of observed sync during generation
- Added requirement that full --operation must be paired with --change
- Captured null rendering behavior for unconfigured operations

**Flow:**
generation -> coordination sync -> if Synchronized set synced_at_generation; if RateLimited do not report fresh success; full --operation only when --change is resolved

**Timestamp:** 2026-04-28

## Narrative
### Structure
These notes describe how manifesto generation should represent sync outcomes and operation instruction visibility. They also capture the scoping rule that embedded operation instructions belong to the resolved change state.

### Dependencies
Depends on coordination sync results and the presence of a resolved change when using manifesto full --operation.

### Highlights
Only Synchronized should produce synced_at_generation. RateLimited must not be treated as fresh success. Unconfigured operations remain null in rendered output.

## Facts
- **synced_at_generation_population**: synced_at_generation should only be populated when coordination sync returns Synchronized [project]
- **rate_limited_sync_status**: RateLimited means no sync was observed during generation and must not be reported as fresh success [project]
- **full_operation_requires_change**: Manifesto full --operation requires --change [project]
- **embedded_operation_instruction_scope**: Embedded operation instructions are scoped to resolved change state [project]
- **memory_instruction_rendering**: Memory instruction rendering exposes configured operation instructions in manifesto output while unconfigured operations remain null [project]
