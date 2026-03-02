## ADDED Requirements

### Requirement: Module and change artifacts support YAML front matter metadata

Ito module and change markdown artifacts SHALL support an optional YAML front matter header delimited by `---` lines at the beginning of the file.

The system MUST treat front matter as metadata and MUST ignore it when parsing the markdown body.

#### Scenario: Read artifact with front matter

- **GIVEN** `module.md` begins with a valid YAML front matter block
- **WHEN** loading the module via the module repository
- **THEN** the module loads successfully
- **AND** the markdown body is parsed correctly
- **AND** the front matter metadata is available to callers that request it

#### Scenario: Read artifact without front matter

- **GIVEN** `module.md` contains only markdown with no front matter
- **WHEN** loading the module via the module repository
- **THEN** the module loads successfully
- **AND** the module metadata uses repository defaults for any missing metadata fields

### Requirement: Front matter stores stable created and updated timestamps

When front matter is present, it SHALL support stable timestamps that are not derived from filesystem metadata.

At minimum the system SHALL support:

- `created_at` (RFC3339 UTC)
- `updated_at` (RFC3339 UTC)

#### Scenario: created_at is stable across copies

- **GIVEN** an artifact with front matter containing `created_at`
- **WHEN** the artifact is copied to a different filesystem location
- **THEN** the repository still reports the same created timestamp

#### Scenario: updated_at is updated on repository writes

- **GIVEN** an artifact with front matter containing `updated_at`
- **WHEN** the artifact is modified through repository write operations
- **THEN** the repository updates `updated_at` to the current time

### Requirement: Front matter provides integrity checks for identifiers

Front matter MAY include identifiers such as `change_id` and `module_id`.

If present, the repository MUST validate that these identifiers match the directory-derived identifiers and MUST return an error when they mismatch.

#### Scenario: Mismatched change_id is rejected

- **GIVEN** a change directory named `024-10_multi-tenant-backend-server`
- **AND** `proposal.md` front matter declares `change_id: 999-99_bad`
- **WHEN** loading the change via the change repository
- **THEN** the repository returns an error indicating the change ID is inconsistent

### Requirement: Front matter supports checksum-based corruption detection metadata

Front matter SHALL support an optional content checksum field (for example `integrity.body_sha256`) that can be used to detect accidental corruption.

If a checksum is present, the repository MUST validate it and MUST report an error when it mismatches.

#### Scenario: Artifact checksum mismatch is detected

- **GIVEN** an artifact front matter declares an `integrity.body_sha256` value
- **AND** the artifact markdown body does not match that checksum
- **WHEN** loading the artifact via the repository
- **THEN** the repository reports an error indicating the artifact content is inconsistent
