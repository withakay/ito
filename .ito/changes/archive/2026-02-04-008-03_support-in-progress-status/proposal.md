## Why

The checkbox-only task format (`- [ ]`/`- [x]`) currently only supports two states: pending and complete. When users attempt to mark a task as in-progress using `ito tasks start`, they receive the error: "Checkbox-only tasks.md does not support in-progress." This creates friction for users who prefer the simpler checkbox syntax but still want to track which task they're actively working on.

## What Changes

- Extend the checkbox format to support a third state: in-progress (`- [~]` or `- [>]`)
- Update the `ito tasks start` command to work with checkbox-format tasks.md files
- Update parsing logic to recognize the new in-progress checkbox marker
- Maintain backward compatibility: existing `- [ ]` and `- [x]` continue to work unchanged

## Capabilities

### New Capabilities

- `checkbox-in-progress`: Extends checkbox-only task format to support in-progress status using a new checkbox marker

### Modified Capabilities

- `cli-tasks`: Update `ito tasks start` command to work with checkbox format, not just enhanced format

## Impact

- **Code**: `ito-domain` crate (task parsing/updating) and `ito-cli` crate (tasks command)
- **Tests**: Task parsing tests need new test cases for in-progress checkbox marker
- **Docs**: Update task format documentation to describe the new marker
- **User Experience**: Users with checkbox-format tasks.md can now use `ito tasks start`
