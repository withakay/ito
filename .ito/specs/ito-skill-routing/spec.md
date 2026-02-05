# ito-skill-routing Specification

## Purpose

TBD - created by archiving change 001-03_add-ito-skill. Update Purpose after archive.

## Requirements

### Requirement: Skill-first command routing

The ito skill SHALL route incoming commands to matching ito-\* skills with higher precedence than the ito CLI. When a command matches both a ito-\* skill and the CLI, the skill MUST be invoked.

#### Scenario: Command matches ito-\* skill

- **WHEN** user invokes ito with command 'archive'
- **THEN** skill checks for ito-archive skill
- **AND** ito-archive skill exists
- **AND** skill invokes ito-archive with provided arguments
- **AND** ito CLI is NOT invoked

#### Scenario: Command matches both skill and CLI

- **WHEN** user invokes ito with command 'status'
- **THEN** skill checks for ito-status skill
- **AND** both ito-status skill and CLI 'status' command exist
- **AND** skill invokes ito-status skill
- **AND** CLI 'status' command is NOT invoked

### Requirement: CLI fallback for unmatched commands

The ito skill SHALL fallback to invoking the ito CLI when no matching ito-\* skill exists. The skill MUST preserve all original command arguments.

#### Scenario: No matching skill exists

- **WHEN** user invokes ito with command 'version'
- **THEN** skill checks for ito-version skill
- **AND** ito-version skill does not exist
- **AND** skill invokes ito CLI with 'version' command
- **AND** all original arguments are passed to CLI

#### Scenario: Skill exists but is not installed

- **WHEN** user invokes ito with command 'archive'
- **AND** ito-archive skill exists in repository
- **BUT** ito-archive is not installed in the agent
- **THEN** skill checks for installed ito-archive skill
- **AND** skill does not find installed ito-archive
- **AND** skill invokes ito CLI with 'archive' command

### Requirement: Argument passthrough

The ito skill MUST pass through all command arguments unchanged to the invoked target (either ito-\* skill or CLI).

#### Scenario: Single argument passthrough

- **WHEN** user invokes ito with command 'view' and argument 'change-123'
- **AND** ito-view skill exists
- **THEN** skill invokes ito-view with argument 'change-123'
- **AND** argument is not modified

#### Scenario: Multiple arguments passthrough

- **WHEN** user invokes ito with command 'validate' and arguments '--strict' and 'change-123'
- **AND** ito-validate skill exists
- **THEN** skill invokes ito-validate with arguments '--strict' and 'change-123'
- **AND** all arguments are passed in original order

#### Scenario: CLI fallback with arguments

- **WHEN** user invokes ito with command 'module' and arguments 'list' and '--json'
- **AND** no ito-module skill exists
- **THEN** skill invokes ito CLI with arguments 'module' 'list' '--json'
- **AND** all arguments are passed unchanged

### Requirement: Command parsing and validation

The ito skill SHALL parse incoming commands to extract the primary command and arguments. The skill MUST validate that at least one command is provided.

#### Scenario: Valid command provided

- **WHEN** user invokes ito with input 'archive 123-45'
- **THEN** skill parses command as 'archive'
- **AND** skill parses arguments as \['123-45'\]
- **AND** routing proceeds

#### Scenario: No command provided

- **WHEN** user invokes ito with no arguments
- **THEN** skill detects missing command
- **AND** skill outputs error message indicating command is required
- **AND** skill does not invoke any skill or CLI

### Requirement: Error handling and reporting

The ito skill SHALL capture and report errors from invoked skills or CLI in a consistent format. Error messages MUST indicate whether the error came from a skill or the CLI.

#### Scenario: Skill invocation fails

- **WHEN** skill invokes ito-archive with arguments
- **AND** ito-archive skill fails with error
- **THEN** skill captures the error output
- **AND** skill reports error with prefix '\[ito-archive skill error\]'
- **AND** original error message is preserved

#### Scenario: CLI invocation fails

- **WHEN** skill invokes ito CLI with command and arguments
- **AND** CLI returns error exit code
- **THEN** skill captures the error output
- **AND** skill reports error with prefix '\[ito CLI error\]'
- **AND** original error message is preserved

### Requirement: Skill discovery

The ito skill SHALL discover available ito-\* skills by querying the installed skills in the agent harness. The skill MUST maintain a cache of discovered skills for performance.

#### Scenario: Initial skill discovery

- **WHEN** ito skill is first invoked
- **THEN** skill queries agent harness for all installed skills
- **AND** skill filters skills matching pattern 'ito-\*'
- **AND** skill builds mapping of commands to skill names
- **AND** mapping is cached for subsequent invocations

#### Scenario: Skill cache invalidation

- **WHEN** ito skill receives command
- **AND** skill cache is stale (older than configured TTL)
- **THEN** skill refreshes skill discovery
- **AND** cache is updated with current installed skills
