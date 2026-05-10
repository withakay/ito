## MODIFIED Requirements

### Requirement: Experimental instruction generation command

The CLI SHALL maintain backward compatibility by keeping the `x-instructions` command as a deprecated alias.

#### Scenario: Deprecated alias emits warning

- **WHEN** agent runs `ito x-instructions proposal --change "001-01_my-change"`
- **THEN** system emits deprecation warning to stderr: "ito x-instructions is deprecated, use ito agents instruction"
- **AND** command still executes successfully
- **AND** output is identical to `ito agents instruction`

#### Scenario: Deprecation warning does not break JSON output

- **WHEN** agent runs `ito x-instructions specs --change "001-01_my-change" --json`
- **THEN** deprecation warning is sent to stderr (not stdout)
- **AND** stdout contains only valid JSON output

## ADDED Requirements

### Requirement: Other experimental commands remain hidden

The other `x-` prefixed commands (`x-templates`, `x-schemas`, `x-new`, `x-artifact-experimental-setup`) SHALL remain as hidden experimental commands until individually promoted.

#### Scenario: x-templates remains hidden

- **WHEN** user runs `ito --help`
- **THEN** `x-templates` does NOT appear in the command list

#### Scenario: x-schemas remains hidden

- **WHEN** user runs `ito --help`
- **THEN** `x-schemas` does NOT appear in the command list
