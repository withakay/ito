## ADDED Requirements

### Requirement: QA testing area infrastructure

The system SHALL provide a QA testing area with scripts for manual or LLM-driven extended integration testing.

#### Scenario: Create qa/ directory structure

- **WHEN** running the initial setup for the QA testing area
- **THEN** the system creates a `qa/` directory at the repository root
- **AND** creates subdirectories for organizing test scripts by capability

### Requirement: Ito Ralph integration test

The system SHALL provide an integration test script that simulates real-world usage of Ito Ralph.

#### Scenario: Test script creates demo environment

- **WHEN** executing `qa/test-ralph-loop.sh`
- **THEN** the script creates a temporary demo directory with a random name
- **AND** initializes a ito project in that directory
- **AND** creates a simple change proposal
- **AND** runs ito ralph against that change
- **AND** verifies the output produces expected results
- **AND** cleans up the temporary directory

#### Scenario: Test script verifies hello world output

- **GIVEN** a change proposal that creates a bash script outputting "hello world"
- **WHEN** the test script runs ito ralph with that change
- **THEN** the test verifies that a shell script is created
- **AND** the test verifies the script contains "hello world"
