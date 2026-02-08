# Ito Architecture

This document describes the architectural guidelines, patterns, and constraints that govern the Ito codebase. For detailed requirements and scenarios behind each component, consult the corresponding specs in `.ito/specs/` — inline references are provided throughout. You can browse all specs with `ito list --specs`.

## Overview

Ito is a spec-driven development tool for AI coding assistants. It provides a structured workflow for managing change proposals, specifications, tasks, modules, and archiving — all backed by markdown files on disk. The CLI is written in Rust and ships as a single binary (`ito`).

## Layered (Onion) Architecture

The project enforces a strict layered architecture with automated guardrails (`make arch-guardrails`):

```
                    ┌──────────────────────────────┐
  Adapters (L3):    │   ito-cli     ito-web        │
                    └──────────────┬───────────────┘
                                   │
                    ┌──────────────▼───────────────┐
  Core (L2):        │          ito-core             │
                    └──────────────┬───────────────┘
                                   │
          ┌────────────────────────▼────────────────────────┐
  Domain  │  ito-domain   ito-templates   ito-config        │
  (L1/0): │  ito-logging  ito-common                        │
          └─────────────────────────────────────────────────┘
```

### Dependency Rules

These rules are enforced by `arch_guardrails.py` and checked via `make arch-guardrails`.

**Forbidden edges:**

- `ito-domain` must NOT depend on `ito-cli`, `ito-web`, or `ito-core`
- `ito-core` must NOT depend on `ito-cli` or `ito-web`
- `ito-cli` must NOT depend on `ito-domain` directly (must route through `ito-core`)

**Required edges:**

- `ito-core` MUST depend on `ito-domain`
- `ito-cli` MUST depend on `ito-core`
- `ito-web` MUST depend on `ito-core`

## Crate Structure

The Rust workspace lives at `ito-rs/` and uses resolver v3 (edition 2024). It contains nine crates:

| Crate | Layer | Description |
|---|---|---|
| `ito-common` | 0 | Foundational utilities: filesystem abstraction, ID parsing/validation, path builders, fuzzy matching, I/O wrappers. Zero domain knowledge. See [`specs/ito-common-crate`](specs/ito-common-crate/spec.md), [`specs/filesystem-trait`](specs/filesystem-trait/spec.md). |
| `ito-config` | 0 | Configuration loading and normalization. Resolves the Ito directory, reads repo-local and global config, exposes `ItoContext`. See [`specs/ito-config-crate`](specs/ito-config-crate/spec.md), [`specs/cascading-config`](specs/cascading-config/spec.md). |
| `ito-domain` | 1 | Domain models (`Change`, `Module`, `Task`, `Audit`, `Workflow`, `Planning`, `Schemas`) and repository traits (ports). See [`specs/ito-domain`](specs/ito-domain/spec.md). |
| `ito-templates` | 1 | Embedded assets for `ito init`/`ito update`. Uses `include_dir!` to embed project templates, home templates, skills, and adapters at compile time. See [`specs/rust-installers`](specs/rust-installers/spec.md). |
| `ito-logging` | 1 | Append-only telemetry to JSONL files. Computes anonymized project IDs via SHA-256 + per-user salt. See [`specs/ito-logging`](specs/ito-logging/spec.md). |
| `ito-core` | 2 | Business logic and orchestration. Implements repositories (filesystem adapters), archive, audit, create, list, show, validate, workflow, harness integrations, and installers. "Policy heavy, UI light." See [`specs/ito-core`](specs/ito-core/spec.md). |
| `ito-cli` | 3 | Thin adapter: CLI argument parsing (`clap`), command dispatch, output formatting, diagnostics (`miette`). See [`specs/rust-cli-plumbing`](specs/rust-cli-plumbing/spec.md). |
| `ito-web` | 3 | Feature-gated web server (`axum`) for browsing/editing projects. Provides API, auth, frontend, terminal emulation. See [`specs/cli-serve`](specs/cli-serve/spec.md). |
| `ito-test-support` | Support | Test helpers: mock repositories, PTY helpers, output normalization, deterministic snapshots. |

## Domain Purity

`ito-domain` has strict API bans enforced via baseline counts in the architecture guardrails:

- **No `miette::`** — error reporting belongs in adapters
- **No `std::fs`** in production code — uses `FileSystem` trait for testability (see [`specs/filesystem-trait`](specs/filesystem-trait/spec.md))
- **No `std::process::Command`** — domain must not spawn processes

## Key Design Patterns

### Repository Pattern

Repository **traits** (ports) live in `ito-domain`; filesystem **implementations** (adapters) live in `ito-core`. This hides the markdown storage format as an implementation detail and enables mock repositories for testing.

```rust
use ito_domain::changes::ChangeRepository;
use ito_domain::modules::ModuleRepository;
use ito_domain::tasks::TaskRepository;

let change_repo = ChangeRepository::new(ito_path);
let changes = change_repo.list()?;
let change = change_repo.get("005-01_foo")?;
```

Do NOT parse markdown files directly for change/task data. Always use the repository.

See [`specs/change-repository`](specs/change-repository/spec.md), [`specs/module-repository`](specs/module-repository/spec.md), and [`specs/task-repository`](specs/task-repository/spec.md) for the full requirements.

### Layered Error Types

Each layer owns its error type:

- `DomainError` (`ito-domain`): I/O, NotFound, AmbiguousTarget
- `CoreError` (`ito-core`): wraps DomainError + Io, Validation, Parse, Process, Sqlite, NotFound, Serde
- CLI layer converts `CoreError` into `miette` diagnostic reports for rich terminal output

### Embedded Assets Pattern

Templates are compiled into the binary via `include_dir!`:

```rust
static DEFAULT_PROJECT_DIR: Dir<'static> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/default/project");
```

This means `ito init` works without any runtime filesystem dependencies for template content. Custom Ito directory names (e.g., `.myito` instead of `.ito`) are handled by path/content rewriting at install time. See [`specs/rust-installers`](specs/rust-installers/spec.md) and [`specs/cli-init`](specs/cli-init/spec.md).

### Managed Block Markers

Files installed by Ito use `<!-- ITO:START -->` / `<!-- ITO:END -->` markers to delineate Ito-managed content from user content. This allows `ito update` to refresh managed blocks without clobbering user additions.

### Harness Adapter Pattern

AI tool integrations ("harnesses") follow an adapter pattern:

- `Harness` trait defines the interface (in `ito-core::harness::types`)
- Concrete implementations: `OpencodeHarness`, `StubHarness` (for testing)
- Harness-specific bootstrap files live in `ito-templates/assets/adapters/`

All harnesses must maintain feature parity. Skills are shared from `assets/skills/` and installed to all harnesses. See [`specs/rust-parity-harness`](specs/rust-parity-harness/spec.md) and [`specs/tool-adapters`](specs/tool-adapters/spec.md).

## Template and Asset Architecture

Templates are the canonical source for what `ito init` installs:

```
ito-rs/crates/ito-templates/assets/
├── default/
│   ├── project/          # Installed to project root
│   │   ├── .ito/         # AGENTS.md, user-guidance.md, project.md
│   │   ├── .claude/      # Claude Code commands/prompts
│   │   ├── .opencode/    # OpenCode commands/prompts
│   │   ├── .github/      # GitHub Copilot prompts
│   │   └── .codex/       # Codex prompts
│   └── home/             # Installed to ~/.config/
├── skills/               # Shared skills installed to ALL harnesses
├── adapters/             # Harness-specific bootstrap files
├── commands/             # Shared command definitions
└── agents/               # Agent prompt definitions
```

Never edit repo-root `.claude/`, `.opencode/`, `.github/` directly. Those are outputs of `ito init`. Edit the source templates in `ito-rs/crates/ito-templates/assets/`.

## Domain Model

### Core Entities

- **Change**: A proposed set of modifications. Contains `proposal.md`, optional `design.md`, `tasks.md`, and spec deltas. Has computed status: `Draft` | `Ready` | `InProgress` | `Paused` | `Complete`. See [`specs/change-creation`](specs/change-creation/spec.md), [`specs/cli-change`](specs/cli-change/spec.md).
- **Module**: Groups related changes (epics). Format: `NNN_module-name`. Enforces scope (which specs a change may modify). See [`specs/cli-module`](specs/cli-module/spec.md), [`specs/module-repository`](specs/module-repository/spec.md).
- **Task**: Individual work items parsed from `tasks.md`. Supports both checkbox format (`- [ ]`) and an enhanced format with dependencies, waves, and status tracking. See [`specs/cli-tasks`](specs/cli-tasks/spec.md), [`specs/task-repository`](specs/task-repository/spec.md).
- **Spec**: The authoritative truth of what IS built. Lives in `.ito/specs/[capability]/spec.md`. Uses `#### Scenario:` format with `WHEN`/`THEN` conditions.
- **Audit Event**: JSONL event log for CLI operations, supporting reconciliation and drift detection. See [`specs/execution-logs`](specs/execution-logs/spec.md).

### Three-Stage Workflow

1. **Propose**: Create change directory with `proposal.md`, `tasks.md`, spec deltas
2. **Implement**: Execute tasks sequentially, track progress
3. **Archive**: Move to `changes/archive/`, update canonical specs

See [`specs/cli-archive`](specs/cli-archive/spec.md) and [`specs/cli-validate`](specs/cli-validate/spec.md) for archive and validation requirements.

### Foundational Invariant

Specs are truth. Changes are proposals. Keep them in sync.

## Architectural Decisions

1. **Markdown as storage format**: All Ito data (specs, proposals, tasks, modules) is stored as markdown files. The repository pattern abstracts this away from consumers.
2. **Specs are truth, changes are proposals**: `.ito/specs/` represents what IS built. `.ito/changes/` represents what SHOULD change.
3. **AI-tool agnostic**: Supports multiple AI harnesses (Claude Code, OpenCode, GitHub Copilot, Codex) with shared skills and harness-specific adapters.
4. **Compile-time embedded assets**: Templates are embedded in the binary via `include_dir!`, making `ito init` self-contained.
5. **Domain purity**: `ito-domain` must remain deterministic and free of I/O, process spawning, and presentation-layer dependencies.
6. **CLI as thin adapter**: `ito-cli` owns argument parsing and output formatting but delegates all logic to `ito-core`.
7. **Optional web UI**: The web server is feature-gated (`ito-web`), keeping the default CLI build lean.

## Quality Enforcement

- `cargo fmt --check` and `cargo clippy` with `-D warnings -D clippy::dbg_macro -D clippy::todo -D clippy::unimplemented`
- `#![warn(missing_docs)]` on all library crates
- `RUSTDOCFLAGS="-D warnings" cargo doc` — docs must build cleanly
- Architecture guardrails via `make arch-guardrails`
- Max file length: 1000 lines per Rust file (enforced by `check_max_lines.py`)
- Pre-commit hooks via `prek`: fmt, clippy, test-coverage
- Pre-push hooks: test-coverage, test
- No breaking of the build on warnings: `RUSTFLAGS="-D warnings"` for all test and lint targets

See [`specs/rust-clippy-policy`](specs/rust-clippy-policy/spec.md), [`specs/rust-documentation-standards`](specs/rust-documentation-standards/spec.md), and [`specs/repo-precommit-quality-gates`](specs/repo-precommit-quality-gates/spec.md) for detailed requirements.

## Guiding Principles

- **YAGNI**: No speculative features or abstractions
- **KISS**: Clarity over cleverness
- **DRY**: Abstract common patterns into reusable crates
- **Boring Technology**: Proven patterns, no unjustified frameworks
- **Simplicity First**: Default to <100 lines of new code, single-file implementations until proven insufficient

## Related Documents

- [Rust style guide](user-rust-style.md) — control flow, pattern matching, naming, type safety, documentation conventions
- [Project conventions](project.md) — project-specific settings
- [User guidance](user-guidance.md) — project-specific agent guidance
- [Templates AGENTS.md](../ito-rs/crates/ito-templates/AGENTS.md) — template maintenance and harness sync guidance
- [All specs](specs/) — authoritative requirements for every capability (`ito list --specs` to browse)
