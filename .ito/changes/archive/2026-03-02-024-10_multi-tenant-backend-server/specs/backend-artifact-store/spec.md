## MODIFIED Requirements

### Requirement: Backend stores Markdown artifacts with revision metadata

The backend SHALL persist change artifacts as Markdown blobs with revision metadata for optimistic concurrency.

The backend MUST persist additional integrity metadata for each change:

- `created_at` (RFC3339 UTC)
- `updated_at` (RFC3339 UTC)
- revision identifier (string)

#### Scenario: Artifact read returns Markdown, revision, and timestamps

- **WHEN** a client reads an artifact bundle for a change
- **THEN** the backend returns Markdown content for each artifact
- **AND** the response includes the current revision identifier
- **AND** the response includes `created_at` and `updated_at` metadata

#### Scenario: Artifact write succeeds with current revision

- **GIVEN** a client provides the current artifact revision
- **WHEN** the client writes updated Markdown
- **THEN** the backend stores the updated content
- **AND** updates `updated_at`
- **AND** increments or replaces the artifact revision identifier

### Requirement: Backend serves artifact content as Markdown with inlined YAML front matter

When the backend returns Markdown artifact content, it MUST inline artifact metadata as YAML front matter at the beginning of the Markdown document, regardless of how the backend stores metadata internally.

At minimum, the inlined front matter MUST include:

- `created_at`
- `updated_at`
- `revision`
- `integrity.body_sha256`

#### Scenario: Artifact read returns Markdown with front matter metadata

- **WHEN** a client reads an artifact from the backend
- **THEN** the artifact content begins with a YAML front matter block (`---` ... `---`)
- **AND** the front matter includes `created_at`, `updated_at`, `revision`, and `integrity.body_sha256`
- **AND** the remainder of the document is the Markdown artifact body

#### Scenario: Storage backend does not affect returned Markdown format

- **GIVEN** the backend is configured to use filesystem storage
- **WHEN** a client reads an artifact
- **THEN** the artifact is returned as Markdown with YAML front matter
- **AND** **WHEN** the backend is configured to use sqlite storage
- **THEN** the artifact is returned in the same Markdown-with-front-matter format

### Requirement: Backend provides artifact bundles for sync

The backend SHALL provide a JSON bundle representation of a change’s artifacts for sync workflows.

The bundle MUST include:

- proposal, tasks, and spec delta documents
- design when present
- per-artifact metadata sufficient for change detection and conflict handling (at minimum `revision`, `updated_at`, and `integrity.body_sha256`)

#### Scenario: Bundle read returns all artifacts

- **WHEN** a client reads a change bundle
- **THEN** the backend returns a JSON document containing all artifacts for the change
- **AND** the bundle includes spec delta documents keyed by capability

### Requirement: Backend rejects stale artifact writes

The backend MUST reject artifact updates when the client revision is stale.

#### Scenario: Stale revision write is rejected

- **GIVEN** artifact revision `r2` is current on the backend
- **WHEN** a client attempts to write using stale revision `r1`
- **THEN** the backend returns a conflict response
- **AND** includes current revision metadata in the response

### Requirement: Backend artifact storage is implemented via swappable repositories

The backend’s artifact persistence MUST be implemented through a repository abstraction so it can be backed by multiple storage implementations.

The backend MUST provide:

- filesystem-backed storage as the default
- sqlite-backed storage as a proof of concept

#### Scenario: Swapping storage backend does not change API semantics

- **GIVEN** backend is configured to use filesystem storage
- **WHEN** a client reads and writes artifact bundles
- **THEN** the API behaves as specified
- **AND** switching backend configuration to sqlite storage preserves the same API-level behavior

### Requirement: Archived changes are immutable on the backend

Once a change is archived, the backend MUST treat the change and its artifacts as immutable.

#### Scenario: Artifact write is rejected for archived change

- **GIVEN** a change is archived
- **WHEN** a client attempts to update any artifact for that change
- **THEN** the backend rejects the request
