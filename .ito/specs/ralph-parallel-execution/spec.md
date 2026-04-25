<!-- ITO:START -->
## ADDED Requirements

### Requirement: Ralph supports parallel task execution with isolated worktrees

Ralph SHALL support executing independent tasks in parallel using isolated worktrees.

- **Requirement ID**: ralph-parallel-execution:isolated-worktrees

#### Scenario: Parallel run creates isolated worktrees

- **WHEN** parallel execution is enabled
- **THEN** Ralph SHALL run each parallel worker in an isolated worktree or equivalent isolated workspace

### Requirement: Parallel execution supports grouped task batches

Parallel execution SHALL support grouped or wave-based batching for task sources that declare parallel groups.

- **Requirement ID**: ralph-parallel-execution:grouped-batches

#### Scenario: YAML task groups run together

- **WHEN** a YAML task source declares tasks in the same parallel group
- **THEN** Ralph SHALL execute that group concurrently up to the configured parallel limit
<!-- ITO:END -->
