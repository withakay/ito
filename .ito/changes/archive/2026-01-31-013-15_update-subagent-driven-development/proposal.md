## Why

The `subagent-driven-development` skill has extensive references to deprecated patterns:
- `superpowers:*` skill syntax (no longer exists)
- `executing-plans` skill (being removed in 013-12)
- `writing-plans` skill (being removed in 013-13)
- `docs/plans/` output location (ito uses `.ito/changes/`)
- `TodoWrite` for tracking (ito uses `ito tasks` CLI)

The skill needs a major update to work with the ito workflow.

## What Changes

- Replace all `superpowers:*` references with modern `ito-*` prefixed skill names
- Replace `executing-plans` references with `ito-apply-change-proposal`
- Replace `writing-plans` references with `ito-write-change-proposal`
- Replace `docs/plans/` with `.ito/changes/<id>/tasks.md`
- Replace `TodoWrite` with `ito tasks` CLI
- Update subagent context to use `ito agent instruction apply`

## Capabilities

### Modified Capabilities

- `subagent-driven-development`: Modernized to use ito workflow, removing all deprecated references

## Impact

- **ito-skills/skills/subagent-driven-development/SKILL.md**: Major rewrite
- **Embedded templates**: Update `ito-subagent-driven-development`
- Skill continues to provide value (dispatch subagent per task with two-stage review) but integrated with ito
