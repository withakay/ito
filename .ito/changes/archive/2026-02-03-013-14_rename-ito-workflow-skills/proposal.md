## Why

The current ito workflow skill names (`ito-proposal`, `ito-apply`) are too terse and don't trigger on common user language. Users asking to "create a feature", "design a change", "write a spec", "implement tasks", or "execute a plan" won't discover these skills.

## What Changes

- **Rename `ito-proposal` to `ito-write-change-proposal`**
- **Rename `ito-apply` to `ito-apply-change-proposal`**
- **Keyword-stuff descriptions** for discoverability:
  - `ito-write-change-proposal`: "Use when creating, designing, planning, proposing, specifying a feature, change, requirement, enhancement, fix, modification, spec, or writing tasks"
  - `ito-apply-change-proposal`: "Use when implementing, executing, applying, building, coding, developing a feature, change, requirement, enhancement, fix, modification, spec, or running tasks"

## Capabilities

### Modified Capabilities

- `ito-proposal` → `ito-write-change-proposal`: Renamed with keyword-rich description
- `ito-apply` → `ito-apply-change-proposal`: Renamed with keyword-rich description

## Impact

- **Embedded templates**: Rename skill directories
- **ito skill (router)**: Update routing logic to use new names
- **Other ito-* skills**: Update any references to old names
- **013-12 and 013-13**: Update to reference new skill names
