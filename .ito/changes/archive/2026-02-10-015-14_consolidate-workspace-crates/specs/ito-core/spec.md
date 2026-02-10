## MODIFIED Requirements

### Requirement: ito-core contains business logic only

The `ito-core` crate SHALL contain Ito business logic and orchestration, plus infrastructure implementations that are part of core use-cases (for example repository implementations and harness integrations).

`ito-core` SHALL remain isolated from CLI and framework concerns.

`ito-core` SHALL NOT define general-purpose utility modules (`io`, `paths`, `id`, `match_`, `discovery`).

#### Scenario: Core does not depend on CLI frameworks

- **WHEN** running `cargo tree -p ito-core`
- **THEN** it does not include `clap`
- **AND** it does not include `crossterm`
- **AND** it does not include `axum`

#### Scenario: Core does not define utility modules

- **WHEN** examining `ito-core` public API
- **THEN** there are no `io`, `paths`, `id`, or `match_` modules implemented in `ito-core`

### Requirement: ito-core dependencies

The `ito-core` crate SHALL depend on workspace crates: `ito-config`, `ito-domain`, `ito-common`, `ito-templates`.

`ito-core` SHALL NOT depend on: `ito-cli`, `ito-web`, `ito-logging`.

#### Scenario: Core depends on config and domain

- **WHEN** running `cargo tree -p ito-core`
- **THEN** it includes `ito-config`
- **AND** it includes `ito-domain`

#### Scenario: Core does not depend on CLI

- **WHEN** running `cargo tree -p ito-core`
- **THEN** it does not include `ito-cli`

## ADDED Requirements

### Requirement: ito-core provides adapter integration surface

`ito-core` SHALL provide adapter-facing modules and re-exports so adapters can implement commands without bypassing core APIs for Ito repository reads/writes.

At minimum:

- `ito-core` SHALL provide a `harness` module (absorbed from the former `ito-harness` crate) containing:
  - `harness/types` (`Harness` trait, `HarnessRunConfig`, `HarnessRunResult`, `HarnessName`)
  - `harness/opencode` (`OpencodeHarness`)
  - `harness/stub` (`StubHarness`)
- `ito-core` SHALL provide helpers for reading/updating Ito markdown/state files, including:
  - `ito_core::state::{read_state, update_state}`
  - `ito_core::tasks::read_tasks_markdown`
  - `ito_core::planning::read_planning_status`
  - `ito_core::show::read_module_markdown`
  - `ito_core::validate::validate_tasks_file`
- `ito-core` SHALL re-export selected utilities needed by adapters, including:
  - `ito_common::match_::nearest_matches`
  - `ito_common::id::parse_module_id`
  - domain schema types via `ito_domain::schemas::*`

#### Scenario: Harness types are available from core

- **WHEN** implementing an adapter
- **THEN** it can use `ito_core::harness::types::Harness` without depending on a separate harness crate

#### Scenario: CLI does not access Ito repo data directly

- **WHEN** implementing CLI commands
- **THEN** Ito repo reads/updates are performed through `ito-core` APIs, not direct file I/O
