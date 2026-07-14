<!-- ITO:START -->
## Why

Ito’s backend state API is currently structured as a **single-project, filesystem-coupled server** that assumes it is started from (and has direct disk access to) the project’s `.ito/` directory.

We want Ito backend to operate as a **network-accessible, multi-tenant service** (LAN/Internet) that can serve many `{org}/{repo}` projects from a single instance, even when the server does **not** have any git checkout of those projects.

## What Changes

- **BREAKING**: Make the backend API **multi-tenant** by scoping all project state routes under `/api/v1/projects/{org}/{repo}/...`.
- Add a backend-server configuration section (`backendServer.*`) to control:
  - storage root (`dataDir`) and storage backend selection
  - allowed orgs and allowed repos per org (including “allow all repos”)
  - authentication (admin tokens + derived per-project tokens)
- Introduce a backend “project store” repository abstraction so the backend server is not tightly coupled to filesystem markdown storage.
- Keep filesystem markdown storage as the default backend store.
- Add a **SQLite-backed project store** as a proof-of-concept, wired through the same repository abstraction.
- Add **YAML front matter support** to module and change artifacts to support fast header reads and store integrity metadata such as `created_at` and `updated_at` (and related fields).

## Capabilities

### New Capabilities

- `backend-project-store`: The backend server can resolve `{org}/{repo}` to a project store and perform repository operations without requiring a git checkout.
- `backend-client-project-scope`: Ito clients can be configured with the `{org}/{repo}` namespace used to address a project on a shared backend.
- `backend-agent-instructions`: Agent-facing instructions, skills, and prompts describe how to configure and use the multi-tenant backend.
- `artifact-front-matter`: Module and change artifacts support YAML front matter for metadata, including stable created/modified timestamps.

### Modified Capabilities

- `backend-state-api`: Routes are project-scoped and operate on backend-managed project storage.
- `backend-auth`: Authentication supports admin tokens and derived per-project tokens (HMAC seed).
- `backend-event-ingest`: Event ingest becomes project-scoped and writes to the correct project audit log.
- `backend-artifact-store`: Artifact storage is backed by a swappable repository and persists integrity metadata.
- `config`: Configuration schema gains `backendServer.*` and backend server config can be supplied via file/env/args.

## Impact

- Backend server (`ito-backend`) routing and auth middleware will change.
- Configuration schema (`ito-config`) will change to add server-side backend configuration.
- Core repository implementations (`ito-core`) will be extended for front matter parsing/writing and to host a SQLite store implementation.
- Domain ports (`ito-domain`) will grow to include the backend project-store abstraction.
- New tests are required for:
  - route scoping by `{org}/{repo}`
  - token validation (admin vs derived project)
  - allowlist enforcement
  - front matter metadata roundtrips
  - filesystem vs SQLite store behavior parity
<!-- ITO:END -->
