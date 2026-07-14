<!-- ITO:START -->
## ADDED Requirements

### Requirement: Worktree-enabled changes use fresh change-named worktrees

When worktrees are enabled in configuration, generated agent instructions SHALL direct proposal and implementation work for each change to occur in a newly created change worktree rather than in the main/control checkout.

- **Requirement ID**: cli-artifact-workflow:fresh-change-worktrees

#### Scenario: Proposal guidance requires a new worktree

- **GIVEN** `worktrees.enabled=true`
- **WHEN** an agent generates proposal instructions for change `012-06_example-change`
- **THEN** the instructions tell the agent to create or switch into a worktree dedicated to `012-06_example-change` before doing proposal work
- **AND** the instructions state that the main/control checkout must remain clean and must not be used for change work

#### Scenario: Apply guidance requires a new worktree

- **GIVEN** `worktrees.enabled=true`
- **WHEN** an agent generates apply instructions for change `012-06_example-change`
- **THEN** the instructions tell the agent to create a new worktree for that change rather than reusing an unrelated or previous change worktree
- **AND** the instructions state that implementation commands must run from the change worktree path

#### Scenario: Worktree and branch names align with change ID

- **GIVEN** `worktrees.enabled=true`
- **AND** the change ID is `012-06_example-change`
- **WHEN** generated instructions show branch or worktree naming guidance
- **THEN** the branch name and primary worktree directory name use `012-06_example-change`
- **AND** names preserve the module and sub-module prefix from the full change ID

#### Scenario: One worktree is not reused for two changes

- **GIVEN** `worktrees.enabled=true`
- **AND** a worktree already exists for change `012-06_example-change`
- **WHEN** an agent starts work for change `012-07_other-change`
- **THEN** the instructions require a separate worktree for `012-07_other-change`
- **AND** the instructions prohibit using the `012-06_example-change` worktree for the new change

#### Scenario: Multiple worktrees for one change remain identifiable

- **GIVEN** `worktrees.enabled=true`
- **AND** an agent needs an additional worktree for change `012-06_example-change`
- **WHEN** generated guidance describes acceptable naming
- **THEN** the additional branch or worktree name starts with `012-06_example-change`
- **AND** the name may append a classifier such as `012-06_example-change-review` or `012-06_example-change-experiment`
<!-- ITO:END -->
