# Source Guide: ito-rs/crates

## Responsibility
This directory contains all Rust workspace crates. Use this file to pick the right crate before opening deeper code.

## Entry Points
| Crate | Responsibility | Guide |
|---|---|---|
| `ito-cli` | CLI parsing, command dispatch, user-facing terminal behavior. | `ito-cli/source-guide.md` |
| `ito-core` | Application use-cases, persistence adapters, validation, installers, orchestration. | `ito-core/source-guide.md` |
| `ito-domain` | Domain entities, repository traits, pure status/traceability logic. | `ito-domain/source-guide.md` |
| `ito-config` | Cascading configuration and invocation context. | `ito-config/source-guide.md` |
| `ito-common` | Shared filesystem, path, ID, matching, and URL utilities. | `ito-common/source-guide.md` |
| `ito-templates` | Embedded templates, skills, commands, schemas, and instruction rendering helpers. | `ito-templates/source-guide.md` |
| `ito-backend` | Multi-tenant HTTP backend adapter. | `ito-backend/source-guide.md` |
| `ito-web` | Browser UI and terminal adapter. | `ito-web/source-guide.md` |
| `ito-logging` | Resilient append-only telemetry. | `ito-logging/source-guide.md` |
| `ito-test-support` | Shared test helpers and fake repositories. | `ito-test-support/source-guide.md` |

## Design
- Prefer adding domain concepts to `ito-domain`, executable behavior to `ito-core`, and surface-specific wiring to adapter crates.
- Template content should be embedded through `ito-templates`; installers live in `ito-core`.
- Test-only helpers belong in `ito-test-support`, not production crates.

## Flow
1. Identify whether a change affects model shape, behavior, surface, or assets.
2. Start in the matching crate guide.
3. Follow upstream/downstream links before editing shared traits or public APIs.

## Gotchas
- `ito-cli` tests often exercise `ito-core` behavior end-to-end using the compiled binary.
- `ito-core` is intentionally broad; prefer subsystem modules over adding logic to catch-all files.

## Tests
- Crate-scoped: `cargo test -p <crate>`.
- End-to-end CLI behavior: `cargo test -p ito-cli --test <integration_test>`.
