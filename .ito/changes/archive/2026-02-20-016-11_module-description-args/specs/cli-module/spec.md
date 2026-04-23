## MODIFIED Requirements

### Requirement: Verb-first module entrypoints

The CLI SHALL expose verb-first command entrypoints for module operations, while keeping `ito module ...` as a deprecated compatibility shim.

#### Scenario: List modules via verb-first command

- **WHEN** user executes `ito list --modules`
- **THEN** behavior matches `ito module list`

#### Scenario: Create module via verb-first command

- **WHEN** user executes `ito create module <name>`
- **THEN** behavior matches `ito module new <name>`

#### Scenario: Create module with description argument

- **WHEN** user executes `ito create module <name> --description <text>`
- **THEN** the created module metadata includes the provided description text
- **AND** command exits successfully without requiring manual post-create edits

#### Scenario: Show module via verb-first command

- **WHEN** user executes `ito show module <id>`
- **THEN** behavior matches `ito module show <id>`

#### Scenario: Validate module via verb-first command

- **WHEN** user executes `ito validate module <id>`
- **THEN** behavior matches `ito module validate <id>`

#### Scenario: Deprecated module shim remains callable

- **WHEN** user executes `ito module <subcommand>`
- **THEN** the command executes successfully
- **AND** prints a deprecation warning pointing to the equivalent verb-first command
