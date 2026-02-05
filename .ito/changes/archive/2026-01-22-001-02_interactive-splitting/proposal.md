## Why

Currently, when a change contains too many delta specs (limit: 10), users receive a passive validation warning: "Consider splitting changes with more than 10 deltas". There is no actionable route to address this warning, forcing users to manually restructure changes or ignore the warning, which leads to large, unreviewable changes.

## What Changes

- Update `ito validate` to be more interactive for large changes
- Detect when delta count exceeds the threshold (10)
- Offer an interactive flow to split the change or override the warning
- Create a mechanism to move delta specs to a new change automatically

## Capabilities

### New Capabilities

- `interactive-change-splitting`: Interactive CLI flow triggered during validation of large changes. Offers options to split deltas into a new change, suppress the warning, or ignore.
- `delta-migration-utility`: Utility to move selected delta specs (and their associated implementations/tasks if possible) from one change to another.

### Modified Capabilities

- `validator-warnings`: Enhance the validator to not just warn but suggest actionable remediation paths for threshold violations.

## Impact

- **UX**: Validation becomes a proactive guide rather than just a checker
- **Code Quality**: Encourages smaller, atomic changes by making splitting easy
- **CLI**: `ito validate` becomes interactive in certain warning scenarios
