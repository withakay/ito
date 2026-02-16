<!-- ITO:START -->
## ADDED Requirements

### Requirement: CLI text output uses absolute paths

The CLI SHALL emit absolute filesystem paths in human-readable stdout/stderr output whenever a filesystem path is displayed.

This requirement applies to ephemeral output meant to be consumed immediately (by a human or an agent), such as:

- `ito agent instruction ...` output
- error messages
- diagnostic output
- helper commands like `ito path ...`

This requirement does NOT mean that repo files written to disk (and expected to be committed to git) should contain machine-specific absolute paths.

#### Scenario: Command renders a filesystem path in text output

- **WHEN** any command prints a filesystem path to stdout or stderr
- **THEN** the path SHALL be absolute and resolved from the project root context

#### Scenario: Instruction artifacts include paths

- **WHEN** Ito renders instruction artifacts (for example via `ito agent instruction ...`) that include filesystem paths
- **THEN** the rendered paths SHALL be absolute

#### Scenario: Files written to disk remain portable

- **WHEN** Ito writes templates, skills, or other project files to disk that are expected to be committed to git
- **THEN** those files MUST NOT embed machine-specific absolute paths
- **AND** SHOULD use repo-relative paths or runtime resolution helpers (for example, instructing scripts to call `ito path ...`)

### Requirement: JSON output uses absolute paths

The CLI SHALL emit absolute filesystem paths in JSON output fields that represent filesystem paths.

#### Scenario: JSON output contains path fields

- **WHEN** a command is executed with `--json`
- **THEN** all filesystem path fields in the JSON output SHALL be absolute

### Requirement: Exceptions are explicit and documented

Any intentional exception to absolute path output SHALL be explicitly documented in the relevant command/spec and clearly labeled in the output as a relative path.

#### Scenario: Exception requires a relative path

- **WHEN** a command must emit a relative path by design
- **THEN** the output SHALL label it as relative and the exception SHALL be documented in the command's spec or help text
<!-- ITO:END -->
