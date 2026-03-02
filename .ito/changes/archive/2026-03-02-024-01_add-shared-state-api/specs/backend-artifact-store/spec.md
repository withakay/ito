## ADDED Requirements

### Requirement: Backend stores Markdown artifacts with revision metadata

The backend SHALL persist change artifacts as Markdown blobs with revision metadata for optimistic concurrency.

#### Scenario: Artifact read returns Markdown and revision

- **WHEN** a client reads an artifact for a change
- **THEN** the backend returns Markdown content
- **AND** the response includes the current artifact revision identifier

#### Scenario: Artifact write succeeds with current revision

- **GIVEN** a client provides the current artifact revision
- **WHEN** the client writes updated Markdown
- **THEN** the backend stores the updated content
- **AND** increments or replaces the artifact revision identifier

### Requirement: Backend rejects stale artifact writes

The backend MUST reject artifact updates when the client revision is stale.

#### Scenario: Stale revision write is rejected

- **GIVEN** artifact revision `r2` is current on the backend
- **WHEN** a client attempts to write using stale revision `r1`
- **THEN** the backend returns a conflict response
- **AND** includes current revision metadata in the response

### Requirement: Backend provides change artifact bundle retrieval

The backend SHALL provide a single operation to retrieve all Markdown artifacts for a change.

#### Scenario: Bundle retrieval returns authored artifacts

- **WHEN** a client requests artifact bundle for a change
- **THEN** the backend returns proposal, design (if present), tasks, and spec delta documents for that change
- **AND** each returned artifact includes revision metadata

### Requirement: Archived changes are immutable on the backend

Once a change is archived, the backend MUST treat the change and its artifacts as immutable.

#### Scenario: Artifact write is rejected for archived change

- **GIVEN** a change is archived
- **WHEN** a client attempts to update any artifact for that change
- **THEN** the backend rejects the request

#### Scenario: Artifact reads remain available for archived change

- **GIVEN** a change is archived
- **WHEN** a client reads artifacts for that change
- **THEN** the backend returns the stored artifacts successfully
