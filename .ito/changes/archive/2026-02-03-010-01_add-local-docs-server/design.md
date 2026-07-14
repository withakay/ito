## Context

We want a per-project web UI that makes it easy to browse Ito artifacts and project docs. The server should be runnable from Ito (`ito serve start`) and should not require committing any generated site artifacts.

The user preference is to lean on Caddy as the external server process.

## Goals / Non-Goals

**Goals:**
- Start a local server with one command.
- Browse Markdown as HTML with lightweight navigation.
- Configurable bind address and port (defaults: `127.0.0.1:9009`).
- Per-project lifecycle management (start/stop), with state stored under `.ito/`.
- Optional token gating when binding to non-loopback.

**Non-Goals:**
- A full hosted docs solution (TLS, users, OAuth).
- GitHub default-branch discovery.
- A perfect Markdown renderer (start simple).

## Proposed Architecture

### Decision: Use stock Caddy + pre-rendered HTML

- Caddy runs as an external dependency (`caddy run`) and serves only a generated site tree under `.ito/.state/docs-server/site/`.
- Ito generates this site tree on `ito serve start` by copying only allowlisted directories into the site tree and rendering Markdown (`*.md`) to HTML (`*.md.html`) alongside directory `index.html` listings.
- This avoids requiring any Caddy Markdown plugins and keeps behavior deterministic.

## Code organization

`ito-cli` has grown large enough that adding a server feature directly into the top-level CLI file would be hard to maintain. This change should keep files comfortably under ~1000 SLOC by splitting the `serve` implementation into focused modules.

Proposed placement:

- `ito-rs/crates/ito-cli/src/commands/serve/`
  - `serve.rs` (subcommand wiring)
- `ito-rs/crates/ito-core/src/docs_server/`
  - `mod.rs` (config + lifecycle + Caddyfile generation)
  - `site.rs` (site generation: copying allowlisted dirs + Markdown rendering)

The CLI layer should remain a thin wrapper around `ito-core` behavior.

### Process model

- `ito serve start` generates a project-specific Caddy configuration and starts `caddy run` in the background.
- Ito stores server state under `.ito/.state/docs-server/`:
  - `Caddyfile`
  - `state.json` (pid/port/bind/token)
  - generated site tree under `site/` (HTML + directory indexes)

### Serving and navigation

- Serve pre-rendered HTML and directory indexes from `.ito/.state/docs-server/site/`.
- The site includes an `index.html` landing page with quick links to the allowlisted roots.
- Markdown files are rendered to `*.md.html` so direct navigation works.

### Path allowlist

To reduce accidental exposure, the server should only expose:
- `.ito/changes/`
- `.ito/specs/`
- `.ito/modules/`
- `.ito/planning/` (if exists)
- `.ito/research/` (if exists)
- `docs/` (if exists)
- `documents/` (if exists)

Everything else in the repo root should be inaccessible.

## Configuration

Use project config loaded via existing cascading config sources.

Proposed keys (project-level):
- `serve.port` (number, default 9009)
- `serve.bind` (string, default `127.0.0.1`)
- `serve.token` (string, optional; if absent and binding is non-loopback, Ito generates one)

## Token gating

Token gating MUST be enforced by the server (not only the UI).

Pragmatic approach (stock Caddy):
- Use a path-based token prefix that Caddy can enforce reliably (e.g. `/t/<token>/...`).
- When binding to a non-loopback address, Ito generates a token (if not configured) and prints the tokenized URL.
- Caddy rejects all requests that do not include the token path prefix.

## Port selection

If the configured port is busy, attempt ports by incrementing until a free port is found. The chosen port should be recorded in the state file and printed.

## Open Questions

- Should `ito serve` (no subcommand) be an alias for `ito serve start`?
