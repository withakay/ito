## MODIFIED Requirements

### Requirement: Preferred help surface is small and stable

The CLI SHALL expose a small stable top-level command surface that is the only supported UX shown in `ito --help`.

#### Scenario: Top-level help shows only stable commands and visible experimentals

- **WHEN** users execute `ito --help`
- **THEN** it lists the stable commands:
  - `init`, `update`
  - `dashboard`
  - `status`, `ralph`
  - `create`, `list`, `show`, `trace`, `validate`, `archive`, `split`
  - `config`, `completions`
- **AND** it lists only the visible experimental commands:
  - `x-templates`, `x-schemas`

#### Scenario: Top-level help hides deprecated and internal commands

- **WHEN** users execute `ito --help`
- **THEN** it does not list deprecated shims or internal commands, including:
  - legacy noun-group shims: `change`, `spec`, `module`, `completion`, `skills`, `view`
  - legacy verb shims: `get`, `set`, `unset`, `reset`, `edit`, `path`, `generate`, `install`, `uninstall`
  - hidden experimental commands: all `x-*` except `x-templates` and `x-schemas`
