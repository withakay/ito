<!-- ITO:START -->
## Why

Relative paths in Ito CLI output are interpreted incorrectly when agents run from unexpected working directories (for example inside a worktree), which leads to files and directories created in the wrong location. Absolute paths are unambiguous regardless of cwd, so standardizing on them prevents this class of errors.

## What Changes

- Normalize all CLI text output that includes filesystem paths to emit absolute paths derived from the project root.
- Ensure instruction templates and project/skill templates render absolute paths using resolved project root context (including context files and tracking paths).
- Ensure JSON output fields that represent filesystem paths are absolute and adjust tests accordingly.
- Document any intentional exceptions where absolute paths are impossible or would mislead users.

## Capabilities

### New Capabilities

- `absolute-path-output`: All Ito CLI output that includes filesystem paths uses absolute paths by default.

### Modified Capabilities

- `worktree-aware-template-rendering`: Worktree setup instructions must render absolute paths instead of relative patterns.

## Impact

- Template assets and instruction rendering (project templates, skills, and instruction templates).
- CLI output formatting for list/show/validate/tasks and error messaging.
- Worktree config context and project root resolution.
<!-- ITO:END -->
