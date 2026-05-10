# Cli Completion Specification

## Purpose

Define the `cli-completion` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Completion operations are grouped

The CLI SHALL expose completion operations under the `ito completions` group.

#### Scenario: Generate completions

- **WHEN** user executes `ito completions generate zsh`
- **THEN** output a complete Zsh completion script to stdout

#### Scenario: Install completions

- **WHEN** user executes `ito completions install zsh`
- **THEN** the completion script is installed for that shell

#### Scenario: Uninstall completions

- **WHEN** user executes `ito completions uninstall zsh`
- **THEN** the completion script is uninstalled for that shell

#### Scenario: Deprecated completion shim remains callable

- **WHEN** user executes `ito completion <subcommand>`
- **THEN** the command executes successfully
- **AND** prints a deprecation warning pointing to `ito completions <subcommand>`
- **AND** the shim is hidden from help and omitted from shell completions


### Requirement: Completion Generation

The completion command SHALL generate completion scripts for all supported shells on demand.

#### Scenario: Generating Zsh completion

- **WHEN** user executes `ito completions generate zsh`
- **THEN** output a complete Zsh completion script to stdout
- **AND** include completions for all preferred commands exposed by `ito --help`
- **AND** include only the visible experimental commands (`x-templates`, `x-schemas`)
- **AND** omit hidden/deprecated compatibility shims from suggestions
- **AND** include all command-specific flags and options
- **AND** use Zsh's `_arguments` and `_describe` built-in functions
- **AND** support dynamic completion for change and spec IDs

#### Scenario: Generating Bash completion

- **WHEN** user executes `ito completions generate bash`
- **THEN** output a complete Bash completion script to stdout
- **AND** include completions for all commands and subcommands
- **AND** use `complete -F` with custom completion function
- **AND** populate `COMPREPLY` with appropriate suggestions
- **AND** support dynamic completion for change and spec IDs via `ito __complete`

#### Scenario: Generating Fish completion

- **WHEN** user executes `ito completions generate fish`
- **THEN** output a complete Fish completion script to stdout
- **AND** use `complete -c ito` with conditions
- **AND** include command-specific completions with `--condition` predicates
- **AND** support dynamic completion for change and spec IDs via `ito __complete`
- **AND** include descriptions for each completion option

#### Scenario: Generating PowerShell completion

- **WHEN** user executes `ito completions generate powershell`
- **THEN** output a complete PowerShell completion script to stdout
- **AND** use `Register-ArgumentCompleter -CommandName ito`
- **AND** implement scriptblock that handles command context
- **AND** support dynamic completion for change and spec IDs via `ito __complete`
- **AND** return `[System.Management.Automation.CompletionResult]` objects
