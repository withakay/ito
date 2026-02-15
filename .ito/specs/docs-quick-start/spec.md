## ADDED Requirements

### Requirement: Quick Start guide is published in docs site navigation
The system SHALL publish a Quick Start guide within the documentation site that is accessible from primary navigation.

#### Scenario: Quick Start appears in top-level navigation

- **WHEN** a user opens the generated docs site
- **THEN** a Quick Start page is visible in top-level navigation
- **AND** selecting it opens a dedicated getting-started page

### Requirement: Quick Start covers first successful workflow
The Quick Start guide SHALL document the minimum sequence to install prerequisites, run initial setup, and execute a first successful command path.

#### Scenario: New contributor follows Quick Start end-to-end

- **WHEN** a new contributor follows the Quick Start steps in order
- **THEN** they can complete setup without consulting unrelated documents
- **AND** they can run at least one documented command successfully
- **AND** the guide references where to continue for deeper documentation
