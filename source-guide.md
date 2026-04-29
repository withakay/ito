# Source Guide: Ito Workspace

## Responsibility
Ito is a Rust workspace for change-driven development workflows: proposals, specs, tasks, validation, worktrees, orchestration, backend sync, template installation, and web/backend adapters. This guide is an orientation map for agents; source code remains authoritative.

## Entry Points
- `Cargo.toml`: workspace membership and shared dependency versions.
- `Makefile`: developer verification targets such as `make check`, `make test`, and release helpers.
- `ito-rs/crates/ito-cli/src/main.rs`: CLI binary entrypoint.
- `ito-rs/crates/ito-web/src/main.rs`: standalone web adapter binary.
- `ito-rs/crates/*/source-guide.md`: crate-level atlas pages.

## Design
- `ito-domain` owns data shapes and traits.
- `ito-core` owns use-cases, persistence adapters, installers, validation, orchestration, and workflow semantics.
- `ito-cli`, `ito-web`, and `ito-backend` are adapters over `ito-core`.
- `ito-config`, `ito-common`, `ito-templates`, `ito-logging`, and `ito-test-support` provide shared infrastructure.

## Flow
1. CLI args are parsed in `ito-cli` and routed to app handlers.
2. App handlers compose configuration from `ito-config` and call `ito-core` use-cases.
3. Core use-cases read/write repositories defined by `ito-domain` traits and implemented by filesystem, backend, or remote adapters.
4. Template and instruction assets come from `ito-templates` and are installed or rendered by `ito-core`.
5. Optional adapters expose the same behavior over HTTP (`ito-backend`) or a browser UI (`ito-web`).

## Directory Summary
| Directory | Responsibility | Guide |
|---|---|---|
| `ito-rs/` | Rust workspace source tree. | `ito-rs/source-guide.md` |
| `ito-rs/crates/` | Crate-by-crate architecture index. | `ito-rs/crates/source-guide.md` |
| `.ito/` | Ito workflow specs, changes, modules, prompts, and project config. | Ito instructions |

## Gotchas
- Main/control checkout is read-only for Ito change work; use a dedicated worktree.
- `.ito/`, `.opencode/`, `.github/`, and `.codex/` contain Ito-managed files and may be overwritten by `ito init` / `ito update`.
- `source-guide.md` files are orientation only. Verify behavior in source before editing.
- Refresh `source-guide.json` after updating source guides so future agents can detect drift.

## Tests
- Prefer `make check` for broad verification.
- Use targeted `cargo test -p <crate>` or integration-test filters for fast feedback.
- The repo has a max-lines guardrail; touching already-large files can trigger `make check` failures.
