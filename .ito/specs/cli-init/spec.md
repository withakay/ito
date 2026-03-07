## ADDED Requirements

### Requirement: Init supports explicit storage mode selection

`ito init` SHALL support selecting the project storage mode as either local filesystem mode or backend mode.

The non-interactive selectors SHALL be `--local` and `--backend`.

#### Scenario: User selects local mode during init

- **WHEN** the user runs `ito init` and selects local mode
- **THEN** Ito completes initialization without backend migration steps

#### Scenario: User selects backend mode during init

- **WHEN** the user runs `ito init` and selects backend mode
- **THEN** Ito enters backend setup flow before backend mode is finalized

#### Scenario: Non-interactive backend selection

- **WHEN** the user runs `ito init --backend`
- **THEN** Ito selects backend mode without showing the storage-mode selection prompt

#### Scenario: Non-interactive local selection

- **WHEN** the user runs `ito init --local`
- **THEN** Ito selects local mode without showing the storage-mode selection prompt

#### Scenario: Conflicting storage selectors are rejected

- **WHEN** the user runs `ito init --local --backend`
- **THEN** Ito exits with an error indicating the flags are mutually exclusive

### Requirement: Init uses deterministic storage and import prompt wording

Interactive `ito init` MUST use stable, explicit prompt text for storage-mode and import decisions.

#### Scenario: Storage-mode prompt text is explicit

- **WHEN** `ito init` runs interactively without `--local` or `--backend`
- **THEN** Ito prompts exactly `Choose project state mode:`
- **AND** provides choices `Local` and `Backend`

#### Scenario: Import decision prompt text is explicit

- **GIVEN** backend mode is selected
- **AND** local changes exist under `.ito/changes/` or `.ito/changes/archive/`
- **WHEN** init asks whether to import
- **THEN** Ito prompts exactly `Local changes detected. Import active and archived changes into backend now?`
- **AND** provides choices `Yes` and `No`

### Requirement: Backend-mode init gates on local change import decision

When backend mode is selected and local change artifacts exist, `ito init` MUST require an explicit import decision.

#### Scenario: User confirms import during backend-mode init

- **GIVEN** local changes exist under `.ito/changes/` or `.ito/changes/archive/`
- **WHEN** backend mode is selected during `ito init`
- **AND** the user answers `Yes` to the import prompt
- **THEN** Ito runs backend import for active and archived local changes

#### Scenario: User declines import during backend-mode init

- **GIVEN** local changes exist under `.ito/changes/` or `.ito/changes/archive/`
- **WHEN** backend mode is selected during `ito init`
- **AND** the user answers `No` to the import prompt
- **THEN** Ito aborts backend-mode setup with actionable guidance
- **AND** backend mode is not enabled

#### Scenario: Backend init with `--yes` and local changes requires explicit import policy

- **GIVEN** local changes exist under `.ito/changes/` or `.ito/changes/archive/`
- **WHEN** the user runs `ito init --backend --yes`
- **THEN** Ito fails with an error instructing the user to pass `--import-local-changes` or `--no-import-local-changes`
- **AND** backend mode is not enabled

#### Scenario: Non-interactive import acceptance flag

- **GIVEN** local changes exist under `.ito/changes/` or `.ito/changes/archive/`
- **WHEN** the user runs `ito init --backend --import-local-changes`
- **THEN** Ito runs backend import for active and archived local changes without prompting

#### Scenario: Non-interactive import rejection flag

- **GIVEN** local changes exist under `.ito/changes/` or `.ito/changes/archive/`
- **WHEN** the user runs `ito init --backend --no-import-local-changes`
- **THEN** Ito aborts backend-mode setup with actionable guidance
- **AND** backend mode is not enabled

#### Scenario: Conflicting import policy flags are rejected

- **WHEN** the user runs `ito init --backend --import-local-changes --no-import-local-changes`
- **THEN** Ito exits with an error indicating the flags are mutually exclusive

### Requirement: Backend-mode init removes local change artifacts after verified import

When backend-mode import succeeds and parity validation passes, `ito init` SHALL remove local change artifacts so backend mode has a single source of truth.

#### Scenario: Local change artifacts are removed after verified import

- **GIVEN** backend-mode init import completed successfully
- **AND** import parity validation passed
- **WHEN** init finalizes backend mode
- **THEN** local change artifacts under `.ito/changes/` are removed
- **AND** subsequent change reads in backend mode resolve from backend state

#### Scenario: Cleanup preserves archive directory scaffold

- **GIVEN** backend-mode init import completed successfully
- **AND** import parity validation passed
- **WHEN** local cleanup executes
- **THEN** Ito removes imported change artifact content from `.ito/changes/`
- **AND** keeps required directory scaffolding needed by local tooling commands

#### Scenario: Local artifacts are retained when import or validation fails

- **GIVEN** backend-mode init import or parity validation fails
- **WHEN** init exits
- **THEN** local change artifacts remain present
- **AND** backend mode is not finalized
