## ADDED Requirements

### Requirement: Backend provides Cloudflare R2 blob storage implementation

The backend MUST provide a Cloudflare R2 blob storage implementation as an artifact store option for serverless edge deployments.

The R2 implementation SHALL:
- Conform to the same repository abstraction as filesystem and SQLite artifact stores
- Store artifact content as blobs in R2 buckets
- Store revision metadata alongside artifacts using R2 custom metadata
- Maintain equivalent read/write semantics as other storage backends
- Support optimistic concurrency control via R2 metadata
- Handle R2-specific connection and object operations

#### Scenario: R2 store provides equivalent read behavior to other backends

- **GIVEN** artifact bundles exist in R2 storage
- **WHEN** a client reads artifacts through the backend
- **THEN** the backend returns Markdown with inlined YAML front matter
- **AND** the response is semantically equivalent to filesystem or SQLite backends

#### Scenario: R2 store persists artifacts with revision metadata

- **GIVEN** the backend is configured to use R2 storage
- **WHEN** a client writes an artifact via the backend API
- **THEN** the artifact content is stored as a blob in R2
- **AND** revision metadata is stored in R2 custom metadata
- **AND** subsequent reads include the correct revision identifier

#### Scenario: R2 store rejects stale artifact writes

- **GIVEN** artifact revision `r2` exists in R2
- **WHEN** a client attempts to write using stale revision `r1`
- **THEN** the backend detects the revision mismatch via R2 metadata
- **AND** returns a conflict response with current revision metadata

#### Scenario: R2 store enforces immutability for archived changes

- **GIVEN** a change is marked as archived in R2 metadata
- **WHEN** a client attempts to update any artifact for that change
- **THEN** the backend rejects the request
- **AND** the R2 artifact remains unchanged

### Requirement: Backend configuration supports Cloudflare R2 selection

The backend configuration schema MUST support selecting Cloudflare R2 as the artifact store backend.

Configuration MUST include:
- R2 bucket binding name or connection details
- R2-specific options (bucket name, region, custom metadata handling)
- Fallback behavior if R2 is unavailable

#### Scenario: Backend initializes with R2 configuration

- **GIVEN** backend configuration specifies R2 as the artifact store
- **WHEN** the backend server starts
- **THEN** the backend initializes the R2 repository adapter
- **AND** all artifact store operations use R2

#### Scenario: Invalid R2 configuration is rejected at startup

- **GIVEN** backend configuration specifies R2 but provides invalid bucket details
- **WHEN** the backend server attempts to start
- **THEN** the backend fails to start with a clear error message indicating the R2 configuration issue

### Requirement: R2 artifact storage maintains front matter format

When serving artifacts from R2, the backend MUST return Markdown with inlined YAML front matter identical to other storage backends.

The R2 implementation SHALL:
- Store raw Markdown content as blobs
- Store metadata (created_at, updated_at, revision, integrity hash) in R2 custom metadata
- Inline metadata as YAML front matter when serving artifacts
- Ensure the returned format is identical regardless of whether artifacts are stored in R2, filesystem, or SQLite

#### Scenario: R2 artifacts include front matter on read

- **GIVEN** an artifact is stored in R2
- **WHEN** a client reads the artifact
- **THEN** the response includes YAML front matter with created_at, updated_at, revision, and integrity.body_sha256
- **AND** the format matches artifacts from filesystem or SQLite stores
