<!-- ITO:START -->
## ADDED Requirements

### Requirement: ito validate gains a repo subcommand

The `ito validate` command SHALL accept a new `repo` subcommand that runs the configuration-aware repository validation engine. The existing `ito validate` behaviour for changes, specs, modules, and bulk validation SHALL remain unchanged.

- **Requirement ID**: validate-repo-cli-surface:repo-subcommand

#### Scenario: Subcommand discoverable via help

- **WHEN** the user runs `ito validate --help`
- **THEN** the help output SHALL list the `repo` subcommand
- **AND** `ito validate repo --help` SHALL describe the new flags

#### Scenario: Existing artifact validation unchanged

- **GIVEN** a change id `011-05_add-ito-validate-repo-coordination-rules`
- **WHEN** the user runs `ito validate 011-05_add-ito-validate-repo-coordination-rules`
- **THEN** the command SHALL execute the existing artifact-content validator

### Requirement: Repo subcommand exposes hook-friendly flags

The `ito validate repo` subcommand SHALL accept `--staged`, `--strict`, `--json`, `--rule <id>` (repeatable), `--no-rule <id>` (repeatable, mutually exclusive with `--rule`), `--list-rules`, and `--explain <id>` flags. The flag semantics SHALL match the engine's introspection and execution capabilities.

- **Requirement ID**: validate-repo-cli-surface:hook-flags

#### Scenario: --json emits the existing ValidationReport envelope

- **WHEN** the user runs `ito validate repo --json`
- **THEN** the stdout SHALL be a single JSON document matching the schema used by `ito validate <item> --json`
- **AND** the document SHALL include a `summary.totals` block

#### Scenario: --staged enables staged-only rules

- **WHEN** the user runs `ito validate repo --staged`
- **THEN** the engine SHALL receive a non-empty `StagedFiles` snapshot read from `git diff --cached --name-only -z`

#### Scenario: --rule and --no-rule are mutually exclusive

- **WHEN** the user runs `ito validate repo --rule coordination/symlinks-wired --no-rule coordination/symlinks-wired`
- **THEN** the command SHALL exit with usage error code 2

#### Scenario: --list-rules prints active and skipped rules

- **WHEN** the user runs `ito validate repo --list-rules`
- **THEN** the command SHALL print every built-in rule with its `active` flag and gating value
- **AND** the command SHALL exit 0 without performing any rule check

#### Scenario: --explain prints the gating values for a single rule

- **WHEN** the user runs `ito validate repo --explain coordination/symlinks-wired`
- **THEN** the command SHALL print the rule id, severity, and the resolved gate values
- **AND** the command SHALL exit 0

### Requirement: Repo subcommand uses documented exit codes

The `ito validate repo` subcommand SHALL exit with code 0 when no `ERROR` issues were emitted, code 1 when at least one `ERROR` issue was emitted (or any `WARNING` under `--strict`), and code 2 for usage errors or unloadable configuration.

- **Requirement ID**: validate-repo-cli-surface:exit-codes

#### Scenario: Clean run exits 0

- **GIVEN** the active rule set produces no issues
- **WHEN** the user runs `ito validate repo`
- **THEN** the exit code SHALL be 0

#### Scenario: Error issue exits 1

- **GIVEN** at least one rule emits an `ERROR` issue
- **WHEN** the user runs `ito validate repo`
- **THEN** the exit code SHALL be 1

#### Scenario: Strict warning exits 1

- **GIVEN** the active rule set produces only `WARNING` issues
- **WHEN** the user runs `ito validate repo --strict`
- **THEN** the exit code SHALL be 1

#### Scenario: Unloadable config exits 2

- **GIVEN** `.ito/config.json` contains a JSON syntax error
- **WHEN** the user runs `ito validate repo`
- **THEN** the exit code SHALL be 2
<!-- ITO:END -->
