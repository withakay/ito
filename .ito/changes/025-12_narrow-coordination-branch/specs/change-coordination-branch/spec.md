<!-- ITO:START -->
## ADDED Requirements

### Requirement: Missing remote coordination branch initializes from empty history

When coordination branch setup finds that the configured remote branch is missing, the system SHALL create the remote coordination branch from an empty root commit instead of pushing the caller's current `HEAD`.

- **Requirement ID**: change-coordination-branch:empty-history-initialization

#### Scenario: Missing branch is created without code history

- **GIVEN** coordination branch sync is enabled
- **AND** `origin/<coordination-branch>` does not exist
- **WHEN** the system ensures the coordination branch exists on `origin`
- **THEN** it creates a root commit from the repository's empty tree
- **AND** pushes that commit to `refs/heads/<coordination-branch>`
- **AND** it does not push the caller's current `HEAD` to the coordination branch

#### Scenario: Existing remote branch is reused

- **GIVEN** coordination branch sync is enabled
- **AND** `origin/<coordination-branch>` already exists
- **WHEN** the system ensures the coordination branch exists on `origin`
- **THEN** it treats the branch as ready
- **AND** it does not create a new root commit

#### Scenario: Reservation initializes the branch before committing metadata

- **GIVEN** coordination branch sync is enabled
- **AND** `origin/<coordination-branch>` does not exist
- **WHEN** the system reserves change metadata on the coordination branch
- **THEN** it first creates `origin/<coordination-branch>` from an empty root commit
- **AND** it checks out the coordination branch in the reservation worktree before committing metadata
- **AND** the reservation commit does not have the caller's current `HEAD` in its parent history
<!-- ITO:END -->
