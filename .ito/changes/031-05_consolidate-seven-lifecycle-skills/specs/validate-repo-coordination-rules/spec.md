<!-- ITO:START -->
## MODIFIED Requirements

### Requirement: Rule coordination/symlinks-wired enforces worktree symlink layout
While legacy coordination-worktree storage remains enabled, the system SHALL evaluate each configured coordination directory and report an `ERROR` when its symlink does not resolve to the corresponding legacy coordination-worktree path. Every remediation message SHALL name a direct CLI or emitted instruction and MUST NOT recommend `ito-update-repo`.

#### Scenario: Healthy legacy symlinks pass
- **GIVEN** legacy coordination-worktree storage is enabled
- **AND** every configured coordination directory resolves to its expected target
- **WHEN** rule `coordination/symlinks-wired` runs
- **THEN** it emits no issues

#### Scenario: Real directory receives migration remediation
- **GIVEN** legacy coordination-worktree storage is enabled
- **AND** a configured coordination path is a real directory instead of its expected symlink
- **WHEN** rule `coordination/symlinks-wired` runs
- **THEN** it emits an `ERROR` identifying the path and expected target
- **AND** the remediation names the direct migration instruction or supported synchronization CLI
- **AND** it does not mention `ito-update-repo`

#### Scenario: Legacy coordination is disabled
- **GIVEN** coordination-worktree storage is feature-disabled or not configured
- **WHEN** repository rules are filtered
- **THEN** `coordination/symlinks-wired` is skipped
<!-- ITO:END -->
