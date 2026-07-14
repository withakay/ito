<!-- ITO:START -->
## Context

`ito-backend` currently binds to a single `project_root`/`.ito` path at process startup and serves routes like `/api/v1/modules/{module_id}` against that single on-disk repository.

For a backend that can be accessed over the network and shared across multiple projects, this coupling is incorrect:

- the backend host may not have the project git repo
- one backend instance should serve many `{org}/{repo}` namespaces
- storage location should be configurable and independent of the backend executable’s working directory

Additionally, module and change metadata (created date, last modified date, integrity signals) should be stable and efficiently readable without scanning full documents.

## Goals / Non-Goals

**Goals:**

- Multi-tenant backend API where project identity is explicit in the route: `/api/v1/projects/{org}/{repo}/...`.
- Backend-managed storage rooted at a configurable data directory (default under the current user’s home / XDG data directory).
- Enforce an allowlist policy:
  - orgs MUST be explicitly allowed
  - per org, repos MAY be `*` (all) or an explicit allowlist
- Authentication:
  - admin (super) token(s) authorize all projects on the instance
  - derived per-project tokens authorize exactly one `{org}/{repo}`
  - derived tokens use `HMAC-SHA256(seed, "{org}/{repo}")` and are deterministic
- Storage abstraction:
  - backend uses a project-store repository port (swappable)
  - default filesystem markdown implementation
  - SQLite proof-of-concept implementation behind the same port
- Front matter:
  - module and change artifacts accept optional YAML front matter
  - store stable `created_at` and `updated_at` timestamps and other integrity metadata
  - front matter is ignored by existing markdown parsing logic

**Non-Goals:**

- Full multi-user identity and RBAC (beyond token tiers and allowlists)
- Remote git operations or cloning repositories on the backend
- Converting all Ito artifacts to a new canonical format (front matter is additive)

## Decisions

### Decision: Multi-tenant routing is mandatory

All state endpoints are project-scoped under `/api/v1/projects/{org}/{repo}`. The previous single-project routes are removed (multi-tenant-only).

Rationale: avoids ambiguous project selection and supports a single backend instance serving many repos.

### Decision: Derived project tokens via HMAC with secret seed

Use `HMAC-SHA256(token_seed, "{org}/{repo}")` for derived tokens.

Rationale: avoids storing per-project tokens while keeping deterministic tokens across restarts; avoids insecure “known salt hash” schemes.

### Decision: Allowlist enforced before token validation

Requests for disallowed org/repo are rejected even with valid tokens (including admin) unless explicitly configured otherwise.

Rationale: defense-in-depth; prevents accidental exposure of namespaces.

### Decision: Project-store port to decouple backend from filesystem

Introduce a domain-level port for “project storage resolution” so `ito-backend` does not assume filesystem markdown repositories.

Implementation strategy:

- `ito-domain`: define a small port/trait set for backend project store operations.
- `ito-core`: provide implementations:
  - filesystem store rooted at `<dataDir>/projects/{org}/{repo}/.ito/`
  - sqlite store rooted at `<dataDir>/sqlite/ito-backend.db` (exact path configurable)
- `ito-backend`: compose handlers using the store port.

### Decision: YAML front matter for module/change artifacts

Module and change markdown artifacts MAY start with YAML front matter delimited by `---` / `---`.

Front matter fields (initial set):

- `schema_version` (string)
- `created_at` (RFC3339 UTC)
- `updated_at` (RFC3339 UTC)
- `created_by` (string, optional)
- `updated_by` (string, optional)
- `integrity` (object; optional future fields like checksum)

Rationale: supports fast header reads and stable timestamps independent of filesystem mtime.

## Risks / Trade-offs

- **Breaking route change** → mitigate with clear docs and versioned prefix; this is early enough to break.
- **Token seed leakage** → mitigate by supporting env var / secret manager injection and avoiding logging.
- **Path traversal / invalid org/repo identifiers** → mitigate with strict identifier validation and never using raw strings as paths.
- **SQLite parity drift vs filesystem store** → mitigate with conformance tests that run against both stores.

## Migration Plan

- No migration is required for existing filesystem `.ito/` repositories because:
  - filesystem store remains default
  - front matter is optional; when absent, repositories fall back to existing behavior

When enabling multi-tenant backend, users point clients to the new base URL and use project-scoped routes.

## Open Questions

- Should admin tokens bypass allowlists (default: no)?
- Exact policy for auto-creating missing `{org}/{repo}` roots:
  - create on first write only, or on first read as empty?
<!-- ITO:END -->
