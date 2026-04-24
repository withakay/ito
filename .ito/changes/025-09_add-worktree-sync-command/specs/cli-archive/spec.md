<!-- ITO:START -->
## ADDED Requirements

### Requirement: Worktree archive is coordinated before main integration

When coordination storage mode is `worktree`, `ito archive <change-id>` SHALL archive the change on the coordination branch first, push that archive state through the coordination worktree, and only then proceed to any configured integration flow for `main`.

- **Requirement ID**: `cli-archive:coordination-first-archive`

#### Scenario: Successful coordination-first archive

- **GIVEN** coordination storage mode is `worktree`
- **AND** archive validation succeeds for `<change-id>`
- **WHEN** the user runs `ito archive <change-id>`
- **THEN** the change is archived on the coordination branch first
- **AND** the coordination branch archive state is pushed before `main` integration steps begin
- **AND** other working copies can receive the archived change through the normal coordination sync path

#### Scenario: Coordination archive failure blocks main integration

- **GIVEN** coordination storage mode is `worktree`
- **AND** archive validation succeeds for `<change-id>`
- **BUT** the coordination archive cannot be pushed successfully
- **WHEN** the user runs `ito archive <change-id>`
- **THEN** the system does not proceed to `main` integration
- **AND** reports that the archive is not yet disseminated to working copies

### Requirement: Archive follows configured main integration mode

After a successful coordination-first archive, the archive workflow SHALL follow the configured `main` integration mode.

- **Requirement ID**: `cli-archive:main-integration-mode`

#### Scenario: Direct merge mode integrates archive into main automatically

- **GIVEN** `changes.archive.main_integration_mode` is `direct_merge`
- **AND** coordination-first archive succeeds
- **WHEN** the archive workflow continues
- **THEN** the archived result is integrated into `main` automatically
- **AND** the workflow reports success only after the `main` integration step completes

#### Scenario: Pull request mode prepares a PR-based integration

- **GIVEN** `changes.archive.main_integration_mode` is `pull_request`
- **AND** coordination-first archive succeeds
- **WHEN** an agent follows archive guidance
- **THEN** the guidance instructs the agent to create an integration branch from `main`
- **AND** apply the archived result to that branch
- **AND** push the branch and create a pull request

#### Scenario: Pull request auto-merge mode requests automatic merge

- **GIVEN** `changes.archive.main_integration_mode` is `pull_request_auto_merge`
- **AND** coordination-first archive succeeds
- **WHEN** an agent follows archive guidance
- **THEN** the guidance instructs the agent to create and push an integration pull request
- **AND** request automatic merge for that pull request when repository policy allows it

#### Scenario: Coordination-only mode leaves main integration pending

- **GIVEN** `changes.archive.main_integration_mode` is `coordination_only`
- **AND** coordination-first archive succeeds
- **WHEN** the archive workflow completes
- **THEN** the archived change is disseminated through the coordination branch
- **AND** the workflow reports that `main` integration is still pending manual follow-up
<!-- ITO:END -->
