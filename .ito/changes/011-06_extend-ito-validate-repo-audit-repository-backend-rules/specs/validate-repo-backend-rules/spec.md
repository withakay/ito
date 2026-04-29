<!-- ITO:START -->
## ADDED Requirements

### Requirement: Rule backend/token-not-committed prevents leaked authentication tokens

When `backend.enabled = true`, the system SHALL emit an `ERROR` issue when `backend.token` is present in any **committed** configuration layer. The check SHALL inspect each cascading config layer individually rather than the merged view, so a token set via `ITO_BACKEND_TOKEN` (env var) or `.ito/config.local.json` (gitignored) is acceptable. The rule's severity SHALL be `ERROR` regardless of the engine's `--strict` flag because a committed token is a security incident.

- **Requirement ID**: validate-repo-backend-rules:token-not-committed

#### Scenario: Token in committed config.json fails

- **GIVEN** `backend.enabled = true`
- **AND** the committed `.ito/config.json` contains a non-empty `backend.token`
- **WHEN** rule `backend/token-not-committed` runs
- **THEN** it SHALL emit an `ERROR` issue
- **AND** the issue's `fix` metadata SHALL list the supported alternatives (env var, `config.local.json`, system keychain)

#### Scenario: Token in config.local.json passes

- **GIVEN** `backend.enabled = true`
- **AND** `backend.token` appears only in `.ito/config.local.json`
- **AND** `.ito/config.local.json` is gitignored
- **WHEN** rule `backend/token-not-committed` runs
- **THEN** it SHALL emit no issues

#### Scenario: Token resolved from env var passes

- **GIVEN** `backend.enabled = true`
- **AND** `backend.token` is unset in every config layer
- **AND** the `ITO_BACKEND_TOKEN` env var is non-empty at runtime
- **WHEN** rule `backend/token-not-committed` runs
- **THEN** it SHALL emit no issues

#### Scenario: Strict flag does not weaken severity

- **GIVEN** `backend.enabled = true`
- **AND** `backend.token` is present in committed config
- **WHEN** rule `backend/token-not-committed` runs without `--strict`
- **THEN** the emitted issue SHALL have level `ERROR`
- **AND** running with `--strict` SHALL produce the same `ERROR` severity

### Requirement: Rule backend/url-scheme-valid enforces a parseable URL

When `backend.enabled = true`, the system SHALL emit an `ERROR` issue when `backend.url` does not parse as a valid URL or its scheme is anything other than `http` or `https`. Empty or unset values SHALL also fail because backend mode requires an addressable endpoint.

- **Requirement ID**: validate-repo-backend-rules:url-scheme-valid

#### Scenario: HTTPS URL passes

- **GIVEN** `backend.enabled = true`
- **AND** `backend.url = "https://api.example.com"`
- **WHEN** rule `backend/url-scheme-valid` runs
- **THEN** it SHALL emit no issues

#### Scenario: Non-http(s) scheme fails

- **GIVEN** `backend.enabled = true`
- **AND** `backend.url = "ftp://files.example.com"`
- **WHEN** rule `backend/url-scheme-valid` runs
- **THEN** it SHALL emit an `ERROR` issue noting the unsupported scheme

#### Scenario: Unparseable URL fails

- **GIVEN** `backend.enabled = true`
- **AND** `backend.url = "not a url"`
- **WHEN** rule `backend/url-scheme-valid` runs
- **THEN** it SHALL emit an `ERROR` issue

### Requirement: Rule backend/project-org-repo-set enforces multi-tenant routing identifiers

When `backend.enabled = true`, the system SHALL emit an `ERROR` issue when either `backend.project.org` or `backend.project.repo` is empty or absent, because multi-tenant backend routing requires both identifiers.

- **Requirement ID**: validate-repo-backend-rules:project-org-repo-set

#### Scenario: Both identifiers present passes

- **GIVEN** `backend.enabled = true`
- **AND** `backend.project.org = "withakay"`
- **AND** `backend.project.repo = "ito"`
- **WHEN** rule `backend/project-org-repo-set` runs
- **THEN** it SHALL emit no issues

#### Scenario: Missing org fails

- **GIVEN** `backend.enabled = true`
- **AND** `backend.project.org` is empty
- **AND** `backend.project.repo = "ito"`
- **WHEN** rule `backend/project-org-repo-set` runs
- **THEN** it SHALL emit an `ERROR` issue identifying `backend.project.org`

#### Scenario: Missing repo fails

- **GIVEN** `backend.enabled = true`
- **AND** `backend.project.org = "withakay"`
- **AND** `backend.project.repo` is empty
- **WHEN** rule `backend/project-org-repo-set` runs
- **THEN** it SHALL emit an `ERROR` issue identifying `backend.project.repo`
<!-- ITO:END -->
