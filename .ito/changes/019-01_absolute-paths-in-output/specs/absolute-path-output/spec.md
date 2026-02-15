<!-- ITO:START -->
## ADDED Requirements

### Requirement: CLI text output uses absolute paths

The CLI SHALL emit absolute filesystem paths in all human-readable output (including instruction artifacts, template-rendered content, and error messages) whenever a filesystem path is displayed.

#### Scenario: Command renders a filesystem path in text output

- **WHEN** any command prints a filesystem path to stdout or stderr
- **THEN** the path SHALL be absolute and resolved from the project root context

#### Scenario: Template-rendered instructions include paths

- **WHEN** Ito renders instruction or project templates that include filesystem paths
- **THEN** the rendered paths SHALL be absolute

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
