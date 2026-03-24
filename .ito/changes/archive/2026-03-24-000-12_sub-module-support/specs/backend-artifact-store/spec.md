<!-- ITO:START -->
## ADDED Requirements

### Requirement: Backend artifact store accepts sub-module change IDs

All backend artifact storage implementations (filesystem, SQLite, Cloudflare R2) SHALL accept change IDs in both `NNN-NN_name` and `NNN.SS-NN_name` formats without treating the dot in sub-module IDs as invalid.

#### Scenario: Write artifact with sub-module change ID succeeds

- **GIVEN** the backend is configured with any supported artifact store
- **WHEN** a client writes an artifact for change ID `024.01-03_add-jwt`
- **THEN** the artifact is stored successfully under the sub-module change key
- **AND** the dot in the ID is treated as a valid character in the storage key

#### Scenario: Read artifact with sub-module change ID returns stored content

- **GIVEN** an artifact for change `024.01-03_add-jwt` was previously written
- **WHEN** a client reads that artifact by ID `024.01-03_add-jwt`
- **THEN** the backend returns the stored content with correct front matter

#### Scenario: List artifacts includes sub-module changes

- **WHEN** the backend lists changes or artifacts
- **THEN** changes with sub-module IDs (`NNN.SS-NN_name`) appear in the listing
- **AND** they are sorted in canonical ID order alongside plain module changes

#### Scenario: Existing plain-module-ID artifacts are unaffected

- **GIVEN** artifacts previously stored under `NNN-NN_name` change IDs
- **WHEN** the backend is upgraded to support sub-module IDs
- **THEN** all existing artifacts remain readable and writable without migration
<!-- ITO:END -->
