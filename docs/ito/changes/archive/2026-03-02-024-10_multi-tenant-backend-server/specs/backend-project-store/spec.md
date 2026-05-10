## ADDED Requirements

### Requirement: Backend resolves org/repo to a project store rooted at a configurable data directory

The backend server SHALL store Ito project state in backend-managed storage rooted at a configurable `dataDir`.

By default, the backend server MUST store data under the current user’s data directory (XDG-aware):

- If `$XDG_DATA_HOME` is set: `$XDG_DATA_HOME/ito/backend`
- Else: `$HOME/.local/share/ito/backend`

Within the data directory, project storage SHALL be namespaced by organization and repository:

`<dataDir>/projects/{org}/{repo}/...`

#### Scenario: First access creates missing org/repo directory structure

- **GIVEN** `{org}/{repo}` does not exist in backend storage
- **WHEN** the backend receives a request that writes state for `{org}/{repo}`
- **THEN** the backend creates the required directory structure
- **AND** the request succeeds

### Requirement: Backend enforces allowed orgs and repos

The backend server MUST enforce an allowlist policy to prevent serving arbitrary namespaces.

- Orgs MUST be explicitly allowed.
- For each allowed org, repos MAY be:
  - `*` (all repos allowed)
  - an explicit list of allowed repos

#### Scenario: Disallowed org is rejected

- **GIVEN** org `evilcorp` is not in the allowed org list
- **WHEN** a client requests `/api/v1/projects/evilcorp/anything/changes`
- **THEN** the backend returns an authorization error

#### Scenario: Allowed org with all repos permitted

- **GIVEN** org `withakay` is allowed
- **AND** repo policy for `withakay` is `*`
- **WHEN** a client requests any repo under `withakay`
- **THEN** the backend authorizes based on token scope

#### Scenario: Allowed org with restricted repos

- **GIVEN** org `acme-inc` is allowed
- **AND** repo policy for `acme-inc` is `["infra", "payments"]`
- **WHEN** a client requests `/api/v1/projects/acme-inc/hr/changes`
- **THEN** the backend rejects the request

### Requirement: Backend project storage implementation is swappable

The backend server MUST interact with project storage through a repository abstraction so the underlying storage implementation can be replaced.

The backend MUST provide:

- a filesystem-based store implementation as the default
- a SQLite-based store implementation as a proof-of-concept

#### Scenario: Filesystem store and SQLite store provide equivalent read behavior

- **GIVEN** equivalent project state exists in both the filesystem and SQLite stores
- **WHEN** a client requests change and module reads through the backend
- **THEN** the backend returns semantically equivalent JSON responses
