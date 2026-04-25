<!-- ITO:START -->
## Why

Worktree-enabled projects currently give agents enough information to create a worktree, but the guidance still permits reuse of an existing change worktree and does not consistently frame the main checkout as off-limits for change work. This makes it too easy for proposal, planning, or implementation work to happen in the control/main checkout and leaves change-to-branch alignment implicit.

## What Changes

- Update Ito agent instruction guidance so that, when worktrees are enabled, every change proposal and its implementation work starts in a newly created worktree.
- Require the branch name and worktree directory name to line up with the full change ID, including the module and sub-module prefix such as `012-06_change-name`.
- State that one worktree MUST NOT be reused for two changes.
- Allow multiple worktrees for the same change only when each worktree name starts with the full change ID and adds a suffix for the sub-classification.
- Clarify that the main/control checkout is kept clean and is not the place to perform change work.
- Update generated instruction artifacts and worktree-related skill guidance so agents receive the same rule set wherever Ito injects guidance.

## Capabilities

### New Capabilities

<!-- None. -->

### Modified Capabilities

- `cli-artifact-workflow`: worktree-aware instruction artifacts must create fresh per-change worktrees and prohibit main/control checkout work for changes.

## Impact

- Affected code: `ito-rs/crates/ito-templates/assets/instructions/agent/*.md.j2`, worktree-related skill templates, and instruction rendering tests.
- Affected behavior: generated agent instructions and installed guidance become stricter when `worktrees.enabled=true`.
- No external API or dependency changes.
<!-- ITO:END -->
