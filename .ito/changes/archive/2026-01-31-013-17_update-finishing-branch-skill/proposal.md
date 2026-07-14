## Why

The `finishing-a-development-branch` skill has two issues:
1. References `executing-plans` (being removed in 013-12)
2. Missing `ito-archive` as an option for completing ito changes

## What Changes

- Replace `executing-plans` reference with `ito-apply`
- Add option 5: "Archive ito change" that invokes `ito-archive`
- Add detection: if working on a ito change, present archive option

## Capabilities

### Modified Capabilities

- `finishing-a-development-branch`: Updated references, added ito-archive option

## Impact

- **ito-skills/skills/finishing-a-development-branch/SKILL.md**: Minor updates
- **Embedded templates**: Update `ito-finish`
- Non-breaking: new option is additive
