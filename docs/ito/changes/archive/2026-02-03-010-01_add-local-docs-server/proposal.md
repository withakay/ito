## Why

Reviewing Ito artifacts (changes/specs/modules) is currently file-based and fragmented across editors and terminals. A local web server makes it easy to browse and read Markdown with navigation, especially when collaborating or when an agent needs a consistent, clickable view of the current state.

## What Changes

- Add a per-project local web server command group (`ito serve start|stop`) that serves:
  - `.ito/changes/`
  - `.ito/specs/`
  - `.ito/modules/`
  - `.ito/planning/` (if present)
  - `.ito/research/` (if present)
  - `docs/` (if present)
  - `documents/` (if present)
- Render Markdown as HTML with simple file-based navigation.
- Default bind/port: `127.0.0.1:9009` (configurable via project config at `.ito/config.json`).
- If the configured/default port is unavailable, auto-select the next available port by incrementing.
- Add `ito serve stop` to terminate the server for the current project.
- Require an external dependency (Caddy) and provide a clear error if it is not installed.
- Optional: support binding to non-loopback addresses (e.g. `0.0.0.0`) and include a tokenized URL (path prefix) to reduce casual exposure.
- Keep the implementation maintainable by placing the feature in dedicated `serve` modules (avoid adding more large blocks to the top-level CLI file; target <1000 SLOC per file).

## Capabilities

### New Capabilities

- `cli-serve`: local docs server lifecycle + config.

### Modified Capabilities

<!-- None -->

## Impact

- Adds a new CLI entrypoint and background process management (per-project state directory, pid tracking).
- Introduces an external runtime dependency (Caddy) and related platform-specific install guidance.
- Touches project config reading (uses existing cascading config loading) to configure port/bind/token behavior.
- Encourages further CLI modularization to prevent a single `ito-cli` source file from growing without bound.
