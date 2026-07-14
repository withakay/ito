<!-- ITO:PROJECT_SETUP:COMPLETE -->

# Project Context

## Purpose

Ito is a public, terminal-first spec-driven design tool for AI-assisted software
development. It turns research and intent into reviewed proposals, capability
specifications, implementation tasks, verification evidence, and archived
current truth without becoming a general project-management system.

## Tech Stack

- Rust workspace with `ito-cli` as the default distributed product.
- `ito-core` contains application policy; `ito-domain` defines storage-neutral
  models and repository traits; adapters live in the CLI, web, and backend
  crates.
- Jinja-based embedded templates generate project instructions and agent
  harness assets.
- Markdown and YAML/JSON under `.ito/` form the reviewed workflow contract.
- Make, Cargo, cargo-llvm-cov, cargo-deny, cargo-dist, and mdBook support
  verification and release planning.

## Project Conventions

### Code Style

- Follow idiomatic Rust and keep public APIs documented.
- Keep unit tests in sibling `*_tests.rs` modules; keep crate integration tests
  under `tests/`.
- Prefer small, focused changes and conventional commit messages.
- Keep CLI handlers thin; business rules belong in `ito-core` or domain types.

### Architecture Patterns

- Tracked `.ito` artifacts on reviewed `main` are the canonical workflow
  authority. Implementation worktrees may isolate code but never own Ito state.
- A proposal package is reviewed and integrated before implementation starts.
  Main-first preflight resolves one immutable authority commit and rejects
  implementation from stale or unrelated branches.
- The standard product keeps iteration available through `ito-loop` and ships
  the `web` feature only.
- `backend` and `coordination-branch` are independent experimental Cargo
  features, disabled and absent from standard release artifacts. Legacy
  coordination detection and the prompt-driven `migrate-to-main` recovery path
  remain available in standard builds.
- Default harness installations expose exactly seven lifecycle skills:
  `ito`, `ito-proposal`, `ito-research`, `ito-apply`, `ito-review`,
  `ito-archive`, and `ito-loop`.
- The retired `docs/ito` mirror and tmux integration must not be regenerated.

### Testing Strategy

- Use focused crate tests while developing, then run `make check` for the
  standard lane.
- Run `make feature-matrix-check` and `make check-experimental` for changes that
  touch Cargo features, coordination, backend, or release boundaries.
- The coverage floor is 80 percent for both lines and regions.
- Run strict Ito validation and traceability for proposal/spec changes.
- Regenerate schemas and managed assets from canonical sources and prove a
  second generation pass is idempotent.
- Use reproducible Showboat demos for user-visible workflow behavior.

### Git Workflow

- Treat the main/control checkout as read-only and use a dedicated branch and
  worktree for each change.
- Review and integrate proposal artifacts into the configured target branch
  before implementation. `pull_request` is the default integration mode;
  `direct_merge` is an explicit alternative.
- Do not push, tag, publish, archive, or merge without explicit authorization.
- Preserve unrelated user changes and retained migration/rollback state.

## Domain Context

Ito's core model is capability-oriented. Current requirements live in
`.ito/specs`; a change records proposal/design/tasks plus ADDED, MODIFIED, or
REMOVED deltas under `.ito/changes/<change-id>`. Review establishes intent,
apply implements the approved tasks, iteration refines implementation, and
archive promotes accepted deltas into current specs.

## Important Constraints

- Spec and change IDs, requirement IDs, task dependencies, and traceability are
  validated contracts, not presentation-only metadata.
- Default and experimental feature lanes must remain independently buildable.
- Generated files under managed harness roots may be overwritten; edit their
  canonical template sources first.
- Never bypass main-first readiness to mutate a change that has not been
  integrated into its configured authority.
- Retained external coordination state is rollback evidence and must not be
  reset, cleaned, committed, pushed, or deleted by ordinary project work.

## External Dependencies

- Git and Worktrunk for branch/worktree isolation.
- Caddy for the optional local documentation server.
- cargo-dist and release-plz for non-publishing release planning and release
  automation.
- Optional Docker, Kubernetes, and service-manager assets support explicitly
  built experimental backend deployments.
