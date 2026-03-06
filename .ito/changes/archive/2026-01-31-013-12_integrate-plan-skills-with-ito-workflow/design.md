## Context

The `executing-plans` skill and `ito-apply` skill both execute tasks from a plan with progress tracking. Having two execution skills creates confusion. The solution is to merge them.

`executing-plans` has valuable patterns that `ito-apply` lacks:
- Batch execution with review checkpoints (3 tasks, report, wait for feedback)
- Critical review before starting
- Explicit stop conditions ("when to stop and ask for help")
- Handoff to finishing-a-development-branch
- Branch safety check (never start on main/master without consent)

`ito-apply` is currently thin - it delegates to CLI output. It should be enhanced with these patterns.

## Goals / Non-Goals

**Goals:**
- Enhance `ito-apply` with valuable execution patterns from `executing-plans`
- Remove `executing-plans` to eliminate duplication
- Update referencing skills (`writing-plans`, `subagent-driven-development`)
- Remove deprecated `superpowers:*` references

**Non-Goals:**
- Changing ito CLI behavior
- Modifying other ito workflow skills beyond `ito-apply`

## Decisions

### 1. Merge direction: executing-plans into ito-apply

**Decision**: Enhance `ito-apply` with executing-plans patterns, then delete executing-plans.

**Rationale**: `ito-apply` is the canonical execution skill in the ito workflow. It should have the best execution patterns.

### 2. Batch size: Default 3 tasks

**Decision**: Keep the "3 tasks per batch" pattern from executing-plans.

**Rationale**: Proven pattern that balances progress with review opportunities.

### 3. ito-apply location

**Decision**: `ito-apply` lives in ito workflow skills (embedded templates), not ito-skills.

**Rationale**: It's a core ito workflow skill, not a general-purpose skill.

### 4. Update location for ito-apply

**Decision**: Update the embedded template at `ito-rs/crates/ito-templates/assets/default/project/.opencode/skills/ito-apply/SKILL.md`

**Rationale**: This is the source of truth for ito workflow skills.

## Risks / Trade-offs

**[Risk] Breaking references** → Skills that reference `executing-plans` will break. Mitigation: Update `writing-plans` and `subagent-driven-development` in same change.

**[Trade-off] ito-apply becomes longer** → More content in one skill. Acceptable for consolidation benefits.

## Migration Plan

1. Enhance `ito-apply` with executing-plans patterns
2. Update `writing-plans` to reference `ito-apply`
3. Update `subagent-driven-development` to remove superpowers references, use `ito-apply`
4. Delete `executing-plans` from ito-skills
5. Remove from embedded templates
6. Update distribution.rs ITO_SKILLS list
