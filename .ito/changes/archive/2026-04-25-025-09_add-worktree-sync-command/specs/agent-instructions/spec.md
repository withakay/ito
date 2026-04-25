<!-- ITO:START -->
## ADDED Requirements

### Requirement: Sync-aware worktree instructions

The system SHALL provide agent-facing sync guidance for worktree-backed coordination workflows and SHALL direct relevant Ito skills and instruction flows to invoke `ito sync` at mutation and handoff points using quiet best-effort behavior.

- **Requirement ID**: `agent-instructions:sync-aware-worktree-guidance`

#### Scenario: Implementation workflow guidance includes sync before execution

- **GIVEN** a change uses worktree-backed coordination storage
- **WHEN** an agent requests implementation guidance for that change
- **THEN** the guidance tells the agent to run `ito sync` before beginning work that depends on current coordination state

#### Scenario: Handoff workflow guidance includes sync before sharing work

- **GIVEN** a change uses worktree-backed coordination storage
- **WHEN** an agent follows commit, finish, archive, or other handoff-oriented Ito guidance
- **THEN** the guidance tells the agent to run `ito sync` before pushing, handing off, or finalizing coordination-sensitive work

#### Scenario: Quiet sync stays quiet during repeated skill-driven calls

- **GIVEN** an Ito skill invokes `ito sync` repeatedly during a short workflow window
- **AND** no validation problem exists
- **AND** the sync command has no new remote work to push
- **WHEN** the sync command is rate-limited
- **THEN** the surrounding guidance remains concise
- **AND** does not add repeated success chatter just because sync was invoked again

### Requirement: Archive guidance follows configured main integration mode

The system SHALL provide archive guidance that reflects the configured `main` integration mode after coordination-first archive succeeds.

- **Requirement ID**: `agent-instructions:archive-integration-guidance`

#### Scenario: Archive guidance for pull request mode

- **GIVEN** `changes.archive.main_integration_mode` is `pull_request`
- **WHEN** an agent requests archive guidance for a completed worktree-backed change
- **THEN** the guidance tells the agent to archive on the coordination branch first
- **AND** then create an integration branch from `main`
- **AND** then push that branch and raise a pull request

#### Scenario: Archive guidance for auto-merge pull request mode

- **GIVEN** `changes.archive.main_integration_mode` is `pull_request_auto_merge`
- **WHEN** an agent requests archive guidance for a completed worktree-backed change
- **THEN** the guidance tells the agent to archive on the coordination branch first
- **AND** then create and push an integration pull request
- **AND** then request automatic merge when repository policy allows it

#### Scenario: Archive guidance for direct merge mode

- **GIVEN** `changes.archive.main_integration_mode` is `direct_merge`
- **WHEN** an agent requests archive guidance for a completed worktree-backed change
- **THEN** the guidance tells the agent to archive on the coordination branch first
- **AND** then integrate the archived result directly into `main`

### Requirement: Workflow guidance is rendered from CLI instruction templates

The system SHALL render sync, archive, and finish workflow guidance from CLI-generated instruction templates, and wrapper skills SHALL delegate to those instructions instead of duplicating workflow logic.

- **Requirement ID**: `agent-instructions:templated-cli-workflows`

#### Scenario: Archive guidance comes from CLI instructions

- **WHEN** an agent requests archive guidance for a change
- **THEN** the guidance is rendered by `ito agent instruction archive`
- **AND** project template assets provide the instruction content

#### Scenario: Wrapper skills delegate to CLI instructions

- **WHEN** an agent invokes `ito-finish` or `ito-archive`
- **THEN** the skill delegates to the matching `ito agent instruction ...` workflow guidance
- **AND** does not become a separate source of archive/finish policy

### Requirement: Finish workflow prompts for archive

For completed worktree-backed changes, the finish workflow SHALL explicitly ask whether to archive now instead of assuming archive is always immediate or always deferred.

- **Requirement ID**: `agent-instructions:finish-archive-prompt`

#### Scenario: Finish asks whether to archive now

- **GIVEN** a completed change uses worktree-backed coordination storage
- **WHEN** an agent follows finish guidance for that change
- **THEN** the guidance includes the question `Do you want to archive this change now?`

#### Scenario: Accepting the prompt follows archive integration mode

- **GIVEN** a completed change uses worktree-backed coordination storage
- **AND** `changes.archive.main_integration_mode` is `pull_request`
- **WHEN** the user answers yes to the finish archive prompt
- **THEN** the guidance tells the agent to archive on the coordination branch first
- **AND** then continue with the configured PR-based integration flow

#### Scenario: Declining the prompt leaves archive pending

- **GIVEN** a completed change uses worktree-backed coordination storage
- **WHEN** the user answers no to the finish archive prompt
- **THEN** the guidance reports that archive/spec dissemination is still pending
- **AND** tells the agent how to archive later with `ito-archive`
<!-- ITO:END -->
