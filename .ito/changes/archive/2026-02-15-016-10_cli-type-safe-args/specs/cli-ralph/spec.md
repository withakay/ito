## MODIFIED Requirements

### Requirement: Harness selection via typed enum

The `--harness` flag on `ito ralph` SHALL accept values parsed by `clap::ValueEnum` from a bridge enum rather than a free-form string. The accepted values, default, and user-visible behaviour SHALL remain unchanged.

#### Scenario: Harness flag uses ValueEnum parsing

- **WHEN** the `RalphArgs` struct is parsed by clap
- **THEN** the `harness` field SHALL be of type `HarnessArg` (not `String`)
- **AND** invalid values SHALL be rejected by clap at parse time

#### Scenario: Accepted harness values unchanged

- **GIVEN** the `--harness` flag on `ito ralph`
- **THEN** the following values SHALL be accepted: `opencode`, `claude`, `codex`, `copilot`, `github-copilot`, `stub`
- **AND** the default SHALL remain `opencode`

#### Scenario: Help output lists user-facing harnesses

- **WHEN** a user runs `ito ralph --help`
- **THEN** the `--harness` flag SHALL list possible values
- **AND** `stub` SHALL NOT appear in the listed values
- **AND** `copilot` SHALL appear (not `github-copilot`)
