## ADDED Requirements

### Requirement: Document module and change ID formats

The documentation SHALL describe flexible ID input formats accepted by CLI commands.

#### Scenario: Flexible module ID formats documented

- **WHEN** reading the CLI Commands Reference section
- **THEN** documentation explains that `1`, `01`, `001`, `1_foo` are all valid module ID inputs

#### Scenario: Flexible change ID formats documented

- **WHEN** reading the CLI Commands Reference section
- **THEN** documentation explains that `1-2_bar`, `001-02_bar`, `1-00003_bar` are all valid change ID inputs

#### Scenario: Canonical format explained

- **WHEN** reading ID format documentation
- **THEN** documentation explains IDs are normalized to `NNN` for modules and `NNN-NN_name` for changes

### Requirement: Document interactive module selection

The documentation SHALL describe the interactive module selection flow in `/ito-proposal`.

#### Scenario: Module selection flow documented

- **WHEN** reading the Proposal section
- **THEN** documentation describes the three module selection options when no module is specified

#### Scenario: Last worked-on module explained

- **WHEN** reading module selection documentation
- **THEN** documentation explains how the system tracks and offers last worked-on module

### Requirement: Add ID format examples section

The documentation SHALL include a dedicated section showing ID format examples.

#### Scenario: Examples section exists

- **WHEN** reading agent-workflow.md
- **THEN** there is a section titled "ID Format Examples" or similar

#### Scenario: Module ID examples provided

- **WHEN** reading ID format examples
- **THEN** examples show: `1` → `001`, `01` → `001`, `1_foo` → module `001`

#### Scenario: Change ID examples provided

- **WHEN** reading ID format examples
- **THEN** examples show: `1-2_bar` → `001-02_bar`, `1-00003_bar` → `001-03_bar`
