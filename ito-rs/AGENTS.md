# Ito Rust Workspace

This is the Rust implementation of Ito. All new work should happen here.

For project-level guidance (Ito workflow, change proposals, specs), see the root [`AGENTS.md`](../AGENTS.md) and [`.ito/AGENTS.md`](../.ito/AGENTS.md).

For architectural guidelines (layering, crate structure, design patterns), see [`.ito/architecture.md`](../.ito/architecture.md).

## Development Commands

Use the Makefile (at the repo root) for common tasks:

```bash
make build            # Build the project
make test             # Run tests
make test-watch       # Run tests in watch mode
make test-coverage    # Run tests with coverage
make check            # Run linter checks (fmt + clippy)
make docs             # Build documentation (fails on warnings)
make arch-guardrails  # Check architectural dependency rules
make clean            # Clean build artifacts
make help             # Show all available targets
```

## Workspace Structure

```
./
├── Cargo.toml                    # Workspace root (resolver v3, edition 2024)
└── ito-rs/
    ├── tools/                    # Guardrail scripts (arch_guardrails.py, check_max_lines.py)
    └── crates/
        ├── ito-common/           # Layer 0: Shared utilities (leaf crate)
        ├── ito-config/           # Layer 0: Configuration loading
        ├── ito-domain/           # Layer 1: Domain models & repository ports
        ├── ito-templates/        # Layer 1: Embedded template assets
        ├── ito-logging/          # Layer 1: Telemetry (append-only JSONL)
        ├── ito-core/             # Layer 2: Business logic & orchestration
        ├── ito-test-support/     # Support: Test utilities
        ├── ito-cli/              # Layer 3 (Adapter): CLI binary
        └── ito-web/              # Layer 3 (Adapter): Web server UI
```

See [`.ito/architecture.md`](../.ito/architecture.md#crate-structure) for the full crate table with spec references.

## Coding Conventions

Follow the Rust style guide at [`.ito/user-rust-style.md`](../.ito/user-rust-style.md). Key rules:

- **For loops** over iterator chains (`.filter().map().collect()`)
- **`let-else`** for early returns, keeping the happy path unindented
- **`if let` chains** to flatten nested conditions
- **Variable shadowing** over renaming (`let input = input.trim()`)
- **Newtypes** over bare `String`; **enums** over `bool` params
- **Explicit match arms** — no wildcards (`_`), no `matches!` macro
- **Explicit destructuring** — `let User { id, name, email } = user`
- **All `pub` items documented** — focus on *why* and *when*, not restating the signature

You can also load the `rust-style` skill for the full guide with examples.

## Quality Gates

All of these run in CI and as pre-commit/pre-push hooks via `prek`:

- `cargo fmt --check`
- `cargo clippy` with `-D warnings -D clippy::dbg_macro -D clippy::todo -D clippy::unimplemented`
- `#![warn(missing_docs)]` on all library crates
- `RUSTDOCFLAGS="-D warnings" cargo doc` — docs must build cleanly
- `RUSTFLAGS="-D warnings"` for all test and lint targets
- Architecture guardrails via `make arch-guardrails` (enforces layering, API bans in `ito-domain`)
- Max 1000 lines per `.rs` file (enforced by `check_max_lines.py`)

## Testing

- **TDD by default**: Red/Green/Refactor
- **Coverage target**: 100% aspirational, 80% hard floor
- **Mocking policy**: "Mocking should give you the ick." Prefer real implementations, in-memory fakes, or `ito-test-support` mock repositories. Extensive mocking indicates tight coupling.
- **Integration tests** alongside unit tests — use `ito-test-support` for PTY helpers, snapshot normalization, and mock repos.

## Repository Pattern

Use repository abstractions for all data access — never parse markdown files directly:

```rust
use ito_domain::changes::ChangeRepository;
use ito_domain::modules::ModuleRepository;
use ito_domain::tasks::TaskRepository;

let change_repo = ChangeRepository::new(ito_path);
let changes = change_repo.list()?;
let change = change_repo.get("005-01_foo")?;
```

See [`.ito/architecture.md`](../.ito/architecture.md#repository-pattern) for the full pattern description.

## Dependency Rules

These are enforced by `arch_guardrails.py`:

- `ito-domain` must NOT depend on `ito-cli`, `ito-web`, or `ito-core`
- `ito-core` must NOT depend on `ito-cli` or `ito-web`
- `ito-cli` must NOT depend on `ito-domain` directly (route through `ito-core`)

## Domain Purity (`ito-domain`)

Strict API bans enforced via baseline counts:

- **No `miette::`** — error reporting belongs in adapters
- **No `std::fs`** in production code — use the `FileSystem` trait
- **No `std::process::Command`** — domain must not spawn processes

## Error Handling

Each layer owns its error type:

- `DomainError` (`ito-domain`): I/O, NotFound, AmbiguousTarget
- `CoreError` (`ito-core`): wraps DomainError + Io, Validation, Parse, Process, Sqlite, NotFound, Serde
- CLI layer converts `CoreError` into `miette` diagnostic reports

## Adding a New Crate

1. Create the crate under `crates/`
2. Add it to the workspace `members` list in `Cargo.toml`
3. Use `workspace.package` for version, edition, license, repository
4. Respect the layering — check which layer the crate belongs to and only depend on crates at the same or lower layer
5. Add `#![warn(missing_docs)]` to `lib.rs`
6. Run `make arch-guardrails` to verify the dependency rules hold

## OpenCode Path Convention

OpenCode uses **plural** directory names:

- `.opencode/skills/` (NOT `.opencode/skill/`)
- `.opencode/commands/` (NOT `.opencode/command/`)
- `.opencode/plugins/` (NOT `.opencode/plugin/`)

When writing tests or code that references OpenCode paths, always use the plural form.

## Git Hooks (prek)

This project uses **[prek](https://github.com/j178/prek)** (NOT `pre-commit`):

```bash
prek install                    # Install pre-commit hook
prek install -t pre-push        # Install pre-push hook
prek run                        # Run on staged files
prek run --all-files            # Run on all files
```

**Hook stages**: pre-commit (fmt, clippy, test-coverage), pre-push (test-coverage, test).

## Guiding Principles

- **YAGNI**: No speculative features or abstractions
- **KISS**: Clarity over cleverness
- **DRY**: Abstract common patterns into reusable crates
- **Boring Technology**: Proven patterns, no unjustified frameworks
- **Simplicity First**: Default to <100 lines of new code, single-file until proven insufficient

## Subagents for Quality and Standards

The following subagents are available to help maintain code quality. Use them proactively — don't wait until the end to check your work.

### During Implementation

| When | Subagent | What it does |
|---|---|---|
| Writing or modifying Rust code | `rust-quality-checker` | Checks style, idioms, and project conventions |
| After writing a significant piece of code | `rust-code-reviewer` | Reviews for memory safety, idiomatic patterns, and architectural fit |
| Writing or updating tests | `rust-test-engineer` | Specialist for test design, coverage strategy, and test infrastructure |
| Complex architecture decisions | `ito-thinking` | High-capability reasoning for design trade-offs and hard problems |

### Before Completing Work

| When | Subagent | What it does |
|---|---|---|
| Before claiming work is done | `test-runner` | Runs `make test` / `make check` via the Makefile and returns clean pass/fail evidence |
| Before committing | `codex-review` | Reviews the diff for quality, correctness, and convention adherence |
| Checking documentation quality | `documentation-police` | Verifies doc comments are complete and genuinely useful |
| Simplifying complex code | `code-simplifier` | Refines Rust code for clarity, consistency, and maintainability |

### Research and Exploration

| When | Subagent | What it does |
|---|---|---|
| Exploring unfamiliar parts of the codebase | `explore` agent (via Task tool) | Fast codebase search and navigation |
| Evaluating libraries or approaches | `perplexity-researcher` or `perplexity-researcher-pro` | Web research with source citations |
| Multi-approach problem solving | `multi-agent` | Explores multiple solutions in parallel and synthesizes the best one |

### Recommended Workflow

1. **Before writing code**: Load the `rust-style` skill for conventions
2. **After writing code**: Run `rust-quality-checker` to catch style issues early
3. **After completing a feature**: Run `test-runner` to verify tests pass, then `rust-code-reviewer` for a quality check
4. **Before committing**: Run `codex-review` on the diff and `documentation-police` on any new public APIs
