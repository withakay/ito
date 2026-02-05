## Context

The `subagent-driven-development` skill dispatches a fresh subagent per task with two-stage review (spec compliance then quality). This is valuable functionality that should be preserved.

However, the skill has extensive references to deprecated patterns that no longer exist or are being removed:
- `superpowers:*` skill syntax
- `executing-plans` and `writing-plans` skills
- `docs/plans/` output location
- `TodoWrite` for tracking

## Goals / Non-Goals

**Goals:**
- Update all references to use ito workflow patterns
- Preserve the core value: subagent-per-task with two-stage review
- Integrate with ito tasks CLI and change artifacts

**Non-Goals:**
- Changing the fundamental approach (subagent dispatch, two-stage review)
- Adding new functionality

## Decisions

### 1. Preserve subagent dispatch pattern

**Decision**: Keep the "fresh subagent per task" approach.

**Rationale**: Valuable for isolation and parallel execution. Aligns with ito-apply-change-proposal multi-agent patterns.

### 2. Preserve two-stage review

**Decision**: Keep spec compliance review then quality review.

**Rationale**: Effective quality gate that catches issues early.

### 3. Use ito CLI for subagent context

**Decision**: Subagents receive context via `ito agent instruction apply --change <id>`.

**Rationale**: Consistent with ito workflow. Subagents get proper context.

## Risks / Trade-offs

**[Risk] Extensive changes** → Many lines need updating. Mitigation: Systematic find/replace with verification.

## Migration Plan

1. Replace all `superpowers:*` with `ito-*` names
2. Replace `executing-plans` with `ito-apply-change-proposal`
3. Replace `writing-plans` with `ito-write-change-proposal`
4. Replace `docs/plans/` with `.ito/changes/<id>/tasks.md`
5. Replace `TodoWrite` with `ito tasks` CLI
6. Update subagent context to use ito CLI
7. Update embedded template
