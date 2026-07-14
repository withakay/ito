<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Worktree detection uses git porcelain output

Ralph SHALL detect Worktrunk-managed worktrees by invoking Worktrunk's structured listing interface when available. The branch field from Worktrunk's listing output SHALL be compared against the change ID to find a match. If Worktrunk is unavailable or does not return structured worktree data, Ralph MAY fall back to parsing `git worktree list --porcelain` for compatibility with existing worktrees.

- **Requirement ID**: `ralph-worktree-awareness:worktree-detection-uses-git-porcelain-output`

#### Scenario: Branch name matches change ID

- **WHEN** the Worktrunk listing output contains a worktree whose branch is `002-16_ralph-worktree-awareness`
- **THEN** Ralph SHALL treat this as a matching worktree for change `002-16_ralph-worktree-awareness`

#### Scenario: Bare repo worktree is excluded

- **WHEN** the Worktrunk listing output contains a bare/control repository entry
- **THEN** Ralph SHALL NOT consider it as a candidate match

#### Scenario: Worktrunk listing unavailable

- **WHEN** Worktrunk cannot provide structured worktree listing output
- **THEN** Ralph SHALL fall back to the existing git porcelain detection behavior without creating a worktree
<!-- ITO:END -->
