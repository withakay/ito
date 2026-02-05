## Context

The `finishing-a-development-branch` skill presents 4 options after implementation:
1. Merge to main
2. Create PR
3. Keep working
4. Discard

It references `executing-plans` which is being removed, and doesn't include ito-archive for completing ito changes.

## Goals / Non-Goals

**Goals:**
- Update reference from `executing-plans` to `ito-apply-change-proposal`
- Add option 5: Archive ito change
- Detect ito changes and highlight archive option when relevant

**Non-Goals:**
- Changing the other 4 options
- Making ito-archive mandatory

## Decisions

### 1. Add as option 5

**Decision**: Add ito-archive as a fifth option, not a replacement.

**Rationale**: The original 4 options are still valid. Archive is additive for ito projects.

### 2. Conditional highlighting

**Decision**: When a ito change is detected, highlight option 5 as relevant.

**Rationale**: Helps users in ito projects discover the archive workflow.

## Migration Plan

1. Replace `executing-plans` with `ito-apply-change-proposal`
2. Add option 5 for ito-archive
3. Add ito change detection logic
4. Update embedded template
