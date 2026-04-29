<!-- ITO:START -->
## ADDED Requirements

### Requirement: Engine exposes a deterministic pre-commit-system detector

The `ito-core::validate_repo` module SHALL expose a `detect_pre_commit_system(project_root)` function that returns one of `Prek`, `PreCommit`, `Husky`, `Lefthook`, or `None`. The detection order SHALL be deterministic and documented; later checks SHALL only run when earlier checks did not match.

- **Requirement ID**: pre-commit-hook-detection:detector-api

#### Scenario: Function is callable from CLI and skill code paths

- **GIVEN** a project root path
- **WHEN** code calls `detect_pre_commit_system(project_root)`
- **THEN** the function SHALL return a `PreCommitSystem` enum value without performing any write operation

### Requirement: Detection order distinguishes prek from plain pre-commit

The detector SHALL classify a project as `Prek` when `.pre-commit-config.yaml` is present AND any prek-specific marker is present (such as `prek` resolvable on `PATH`, a `mise.toml` mentioning `prek`, or a `prek:` toolchain hint inside `.pre-commit-config.yaml`). Otherwise, when `.pre-commit-config.yaml` exists alone, the project SHALL be classified as `PreCommit`.

- **Requirement ID**: pre-commit-hook-detection:prek-vs-precommit

#### Scenario: prek is detected when toolchain hint is present

- **GIVEN** `.pre-commit-config.yaml` exists at repo root
- **AND** `mise.toml` contains a `prek` line
- **WHEN** `detect_pre_commit_system` runs
- **THEN** it SHALL return `Prek`

#### Scenario: Plain pre-commit is detected without prek markers

- **GIVEN** `.pre-commit-config.yaml` exists at repo root
- **AND** no prek markers are present
- **WHEN** `detect_pre_commit_system` runs
- **THEN** it SHALL return `PreCommit`

### Requirement: Detector identifies Husky and lefthook

The detector SHALL classify a project as `Husky` when a `.husky/` directory exists at the repo root or when `package.json` contains a top-level `husky` key. The detector SHALL classify a project as `Lefthook` when any of `lefthook.yml`, `lefthook.yaml`, `.lefthook.yml`, or `.lefthook.yaml` exists at the repo root.

- **Requirement ID**: pre-commit-hook-detection:husky-and-lefthook

#### Scenario: Husky is detected via .husky directory

- **GIVEN** `.husky/` directory exists at the repo root
- **WHEN** `detect_pre_commit_system` runs
- **THEN** it SHALL return `Husky`

#### Scenario: Lefthook is detected via configuration file

- **GIVEN** `lefthook.yml` exists at the repo root
- **AND** `.pre-commit-config.yaml` does not exist
- **WHEN** `detect_pre_commit_system` runs
- **THEN** it SHALL return `Lefthook`

### Requirement: Detector returns None when no system is present

The detector SHALL return `None` when none of the supported markers are present at the repo root.

- **Requirement ID**: pre-commit-hook-detection:none-default

#### Scenario: Empty repo returns None

- **GIVEN** a project root that contains no `.pre-commit-config.yaml`, no `.husky/`, no `husky` key in `package.json`, and no lefthook configuration files
- **WHEN** `detect_pre_commit_system` runs
- **THEN** it SHALL return `None`

### Requirement: Detector is read-only and side-effect free

The detector SHALL only read filesystem state. It SHALL NOT install hook frameworks, modify configuration files, or shell out to network services.

- **Requirement ID**: pre-commit-hook-detection:read-only

#### Scenario: Detector does not write any file

- **WHEN** `detect_pre_commit_system` runs against any fixture project
- **THEN** the project tree on disk SHALL be byte-identical before and after the call
<!-- ITO:END -->
