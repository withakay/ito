<!-- ITO:START -->
## Why

Pull requests can remain blocked when CI fails and no one has time to triage logs and prepare a fix quickly. Adding an on-demand PR fix workflow gives maintainers a consistent way to invoke automated diagnosis and remediation directly from PR context.

## What Changes

- Add a new slash-command workflow triggered by `/pr-fix` (and `:eyes:` reaction) that targets the active pull request.
- Define a focused PR-fix agent prompt that reads PR context, analyzes failing CI logs, applies fixes, runs project checks/formatters, pushes improvements, and comments with a change summary.
- Configure constrained permissions, safe outputs, network defaults, and a bounded timeout suitable for automation in this repository.

## Capabilities

### New Capabilities

- `pr-fix-workflow`: On-demand pull request remediation workflow driven by slash command and PR context.

### Modified Capabilities

- None.

## Impact

- Adds a new workflow definition under the repository automation/workflow configuration.
- Improves PR turnaround by reducing manual CI-failure triage and fix iteration.
- No runtime API changes; impacts repository automation behavior and maintainer workflow.
<!-- ITO:END -->
