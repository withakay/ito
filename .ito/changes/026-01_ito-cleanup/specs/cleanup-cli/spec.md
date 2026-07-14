<!-- ITO:START -->
## ADDED Requirements

### Requirement: Upgrade cleanup detection

`ito init --upgrade` SHALL detect orphaned files from previous Ito versions and report them to the user.

- **Requirement ID**: cleanup-cli:upgrade-detection

#### Scenario: Upgrade detects orphaned skills

- **WHEN** `ito init --upgrade` is run
- **AND** the repo contains skill directories that were removed or renamed in the current version
- **THEN** the CLI SHALL print a list of detected orphaned files to stderr
- **AND** the CLI SHALL suggest running `ito init --upgrade --cleanup` to remove them

#### Scenario: Upgrade with no orphans

- **WHEN** `ito init --upgrade` is run
- **AND** the repo has no orphaned Ito files
- **THEN** the CLI SHALL NOT print any cleanup warnings

### Requirement: Upgrade cleanup removal

`ito init --upgrade --cleanup` SHALL remove detected orphaned files after confirmation.

- **Requirement ID**: cleanup-cli:upgrade-removal

#### Scenario: Interactive cleanup with confirmation

- **WHEN** `ito init --upgrade --cleanup` is run interactively
- **THEN** the CLI SHALL list all detected orphaned files
- **AND** the CLI SHALL prompt the user for confirmation before removing each file or all files
- **AND** upon confirmation, the CLI SHALL remove the orphaned files
- **AND** the CLI SHALL print a summary of removed files

#### Scenario: Non-interactive cleanup with force

- **WHEN** `ito init --upgrade --cleanup --force` is run
- **THEN** the CLI SHALL remove all detected orphaned files without prompting
- **AND** the CLI SHALL print a summary of removed files

#### Scenario: Cleanup preserves user-owned files

- **WHEN** `ito init --upgrade --cleanup` is run
- **THEN** the CLI SHALL NOT remove any user-owned files (`.ito/project.md`, `.ito/config.json`, `.ito/user-guidance.md`, `.ito/user-prompts/*`)
- **AND** the CLI SHALL only remove files that match the legacy file registry
<!-- ITO:END -->
