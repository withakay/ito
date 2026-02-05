## Why

The ito archive command currently allows archiving changes without validating completion, which can lead to incomplete changes being archived prematurely. This prevents users from discovering and finishing incomplete work, reducing the overall effectiveness of the change management system. Users should receive clear guidance and actionable next steps when attempting to archive incomplete changes.

## What Changes

- Add validation step to ito archive command that checks change completion status before proceeding
- Modify archive workflow to prevent archiving of incomplete changes
- Implement user-friendly prompts that guide users toward appropriate actions for incomplete changes (e.g., continue implementation, abandon change, mark as draft)
- Provide clear information about what is missing from incomplete changes to help users understand completion requirements
- Ensure validation messages reference relevant ito commands for next steps

## Capabilities

### New Capabilities

- `archive-completion-validation`: Validation step that checks if all required artifacts and implementation are complete before allowing archive
- `archive-incomplete-guidance`: User-friendly prompts and suggestions for actions when attempting to archive incomplete changes

### Modified Capabilities

- `change-archiving`: Modified to include validation step and prevent archiving of incomplete changes

## Impact

- Breaking change: Users will no longer be able to archive incomplete changes
- Improves change management quality by ensuring only complete changes are archived
- Provides clearer guidance to users on how to handle incomplete changes
- Requires ito validate command to be executed before archiving
- May require force flag (--force) for edge cases where archiving incomplete changes is necessary
