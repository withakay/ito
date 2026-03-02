<!-- ITO:START -->
## Why

Agents are highly effective when they can use familiar shell workflows like `rg`/`grep` to search project artifacts. In backend mode, the artifacts may be remote and not present on disk, which breaks this workflow.

Ito should provide a consistent, bash-like search interface that works the same whether artifacts are local (`.ito/`) or served from a multi-tenant backend.

## What Changes

- Add an `ito grep` command that searches Ito artifacts using ripgrep-style regular expressions.
- Support search scopes:
  - within a single change
  - within a module (across all changes in that module)
  - across all changes in a project
- In backend mode, implement a local on-disk cache and use HTTP conditional requests (`ETag` / `If-None-Match`) to avoid excessive round-trips.
- Provide output controls to avoid overwhelming agents:
  - limit returned matches/lines via CLI flags
  - document how to pipe output through standard bash tools (`head`, `sed`, etc.)
- Implement the search engine in `ito-core` using the ripgrep crate ecosystem (not by shelling out).

## Capabilities

### New Capabilities

- `cli-grep`: The CLI provides a consistent search interface over Ito artifacts for agents.

### Modified Capabilities

- `config`: Configuration may be extended to support any minimal grep/caching settings if needed (default behavior should be safe without configuration).

## Impact

- New CLI surface area (`ito grep ...`).
- New core search module (`ito-core`) and supporting tests.
- Backend-mode grep requires a local cache directory (XDG-aware) and revalidation via `ETag`.
<!-- ITO:END -->
