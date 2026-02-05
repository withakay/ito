## Context

The `writing-plans` skill and `ito-write-change-proposal` skill both create structured task lists for implementation. Having two planning skills creates confusion. The solution is to merge them.

`writing-plans` has valuable patterns that `ito-write-change-proposal` lacks:
- Bite-sized task granularity (2-5 min steps)
- TDD flow per task (failing test → run → implement → run → commit)
- Task structure guidance (exact file paths, complete code, exact commands)
- Plan header template (goal, architecture, tech stack)

`ito-write-change-proposal` is currently thin - it delegates to CLI output. It should be enhanced with these patterns.

## Goals / Non-Goals

**Goals:**
- Enhance `ito-write-change-proposal` with valuable task authoring patterns from `writing-plans`
- Remove `writing-plans` to eliminate duplication
- Update referencing skills (`subagent-driven-development`)

**Non-Goals:**
- Changing ito CLI behavior or task format
- Modifying other ito workflow skills beyond `ito-write-change-proposal`

## Decisions

### 1. Merge direction: writing-plans into ito-write-change-proposal

**Decision**: Enhance `ito-write-change-proposal` with writing-plans patterns, then delete writing-plans.

**Rationale**: `ito-write-change-proposal` is the canonical planning skill in the ito workflow. It should have the best task authoring guidance.

### 2. Task granularity: 2-5 minute steps

**Decision**: Keep the "2-5 minute" task size guidance from writing-plans.

**Rationale**: Proven pattern that enables steady progress and easy verification.

### 3. TDD flow: Include in task guidance

**Decision**: Add TDD flow guidance to ito-write-change-proposal task creation.

**Rationale**: TDD ensures verifiable tasks and prevents untested code.

### 4. ito-write-change-proposal location

**Decision**: `ito-write-change-proposal` lives in ito workflow skills (embedded templates), not ito-skills.

**Rationale**: It's a core ito workflow skill.

## Risks / Trade-offs

**[Risk] Breaking references** → Skills that reference `writing-plans` will break. Mitigation: Update `subagent-driven-development` in same change.

**[Trade-off] ito-write-change-proposal becomes longer** → More content in one skill. Acceptable for consolidation benefits.

## Migration Plan

1. Enhance `ito-write-change-proposal` with writing-plans patterns
2. Update `subagent-driven-development` to reference `ito-write-change-proposal`
3. Delete `writing-plans` from ito-skills
4. Remove from embedded templates
5. Update distribution.rs ITO_SKILLS list
