## ADDED Requirements

### Requirement: Init supports opt-in coordination branch provisioning

`ito init` SHALL support an opt-in `--setup-coordination-branch` flag that prepares the configured coordination branch on `origin`.

#### Scenario: Existing coordination branch is already ready

- **GIVEN** `changes.coordination_branch.name` resolves to `<branch>`
- **WHEN** the user runs `ito init --setup-coordination-branch`
- **AND** `origin/<branch>` already exists and is reachable
- **THEN** init completes successfully
- **AND** the CLI reports that the coordination branch is ready

#### Scenario: Missing coordination branch is created during init

- **GIVEN** `changes.coordination_branch.name` resolves to `<branch>`
- **WHEN** the user runs `ito init --setup-coordination-branch`
- **AND** `origin/<branch>` does not exist
- **THEN** Ito creates the branch on `origin`
- **AND** init completes successfully with a created/ready message

#### Scenario: Remote setup failure reports recovery guidance

- **WHEN** the user runs `ito init --setup-coordination-branch`
- **AND** `origin` is missing or push is rejected
- **THEN** init fails with a deterministic error
- **AND** the message explains the remote/authentication step needed before retrying
