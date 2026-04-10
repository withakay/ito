<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ralph supports branch-per-task workflows

Ralph SHALL support creating dedicated git branches for task execution when branch automation is enabled.

- **Requirement ID**: ralph-git-automation:branch-per-task

#### Scenario: Branch-per-task creates a task branch

- **WHEN** branch-per-task mode is enabled for a Ralph run
- **THEN** Ralph SHALL create or reuse a task-specific branch before executing the task

### Requirement: Ralph can create pull requests for automated work

Ralph SHALL support optional PR creation for completed task branches when PR automation is enabled.

- **Requirement ID**: ralph-git-automation:create-pr

#### Scenario: Completed branch opens a PR

- **WHEN** PR automation is enabled and a task branch completes successfully
- **THEN** Ralph SHALL push the branch and create a pull request using the configured base branch
<!-- ITO:END -->
