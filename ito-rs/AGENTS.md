# Ito Rust Workspace

This is the Rust implementation of Ito. All new work should happen here.

For project-level guidance (Ito workflow, change proposals, specs), see the root [`AGENTS.md`](../AGENTS.md) and [`.ito/AGENTS.md`](../.ito/AGENTS.md).

For architectural guidelines (layering, crate structure, design patterns), see [`.ito/architecture.md`](../.ito/architecture.md).

## Development Commands

Use the Makefile (at the repo root) for common tasks:

```bash
make init             # Check toolchain and install hooks
make build            # Build the project
make test             # Run tests
make test-timed       # Run tests with per-crate timing
make test-watch       # Run tests in watch mode
make test-coverage    # Run tests with coverage
make check            # Run full pre-push quality gate via prek
make docs             # Build documentation (fails on warnings)
make arch-guardrails  # Check architectural dependency rules
make cargo-deny       # Run license/advisory checks
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
        ├── ito-backend/          # Layer 3 (Adapter): HTTP backend API
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

- **`let-else`** for early returns, keeping the happy path unindented
- **`if let` chains** to flatten nested conditions
- **Variable shadowing** over renaming (`let input = input.trim()`)
- **Newtypes** over bare `String`; **enums** over `bool` params
- **Explicit match arms** — no wildcards (`_`), no `matches!` macro
- **Explicit destructuring** — `let User { id, name, email } = user`
- **All `pub` items documented** — focus on *why* and *when*, not restating the signature

You can also load the `rust-style` skill for the full guide with examples.

## Error Message Quality

**Every error message must answer three questions: What failed? Why? How do I fix it?**

Errors are the primary interface between the tool and a stuck user (human or AI agent). A bare OS error like `"I/O error: File exists (os error 17)"` is useless — it gives no path, no context, and no remediation. An actionable error looks like:

```
Cannot acquire lock: /path/to/.ito/workflows/.state/change-allocations.lock

A previous `ito create` may have been interrupted, leaving a stale lock file.
Fix: delete the lock file and retry:

  rm /path/to/.ito/workflows/.state/change-allocations.lock
```

Rules:

- **Include the path** — never surface a filesystem error without the file/directory involved
- **Explain the likely cause** — don't make the user reverse-engineer what happened
- **Suggest a fix** — give a concrete command or action, not just a description
- **Prefer dedicated error variants** over generic `Io(#[from] io::Error)` when the failure mode is known and recoverable
- **Don't swallow context** — if wrapping an error, preserve the original message alongside the higher-level explanation

When writing `thiserror` variants, prefer structured fields over opaque wrappers so the Display impl can produce a complete diagnostic.

## Quality Gates

The primary local gate is `make check` (which runs `prek` at `pre-push` stage on all files).
CI enforces the same Rust checks plus docs-site and PowerShell parse checks.

- File hygiene + markdown/yaml/json checks from `.pre-commit-config.yaml`
- `cargo fmt --check`
- `cargo clippy` with `-D warnings -D clippy::dbg_macro -D clippy::todo -D clippy::unimplemented`
- `#![warn(missing_docs)]` on all library crates
- `RUSTDOCFLAGS="-D warnings" cargo doc` — docs must build cleanly
- `make test-affected` and `make test-coverage` (coverage hard floor 80%, target 90%)
- Architecture guardrails via `make arch-guardrails` (enforces layering and domain API baselines)
- `make check-max-lines` (`check_max_lines.py`: soft 1000-line warning, hard 1200-line failure)
- `make cargo-deny` for license/advisory checks

## Testing

- **TDD by default**: Red/Green/Refactor
- **Coverage policy**: `make test-coverage` enforces an 80% hard floor and reports a 90% target
- **Mocking policy**: "Mocking should give you the ick." Prefer real implementations, in-memory fakes, or `ito-test-support` mock repositories. Extensive mocking indicates tight coupling.
- **Integration tests** alongside unit tests — use `ito-test-support` for PTY helpers, snapshot normalization, and mock repos.
- **Test file separation**: If a source file exceeds 300 lines, put tests in a separate file (e.g., `tests/backend_auth.rs` for `ito-core`, or a sibling `*_tests.rs` module). Do not inline `#[cfg(test)] mod tests` in files over 300 lines.

### Expected Test Execution Times

The full test suite (`make test`) should complete in **under 10 seconds**. Individual crate tests typically run in 1-2 seconds. Use `make test-timed` to see per-crate wall-clock times.

| Benchmark | Expected | Alert Threshold |
|---|---|---|
| Full suite (`make test`) | ~5s | > 10s |
| Harness opencode tests (`cargo test -p ito-core --test harness_opencode`) | ~1s | > 5s |
| Any single crate | < 2s | > 5s |

If tests exceed these thresholds, investigate before merging. Common causes:
- Timeout monitor threads not exiting cleanly (see `process_done` flag pattern in opencode harness)
- Blocking I/O in tests without timeouts
- Missing test parallelism

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

- `ito-domain` must NOT depend on `ito-cli`, `ito-web`, `ito-backend`, or `ito-core`
- `ito-core` must NOT depend on `ito-cli`, `ito-web`, or `ito-backend`
- `ito-cli` and `ito-backend` must NOT depend on `ito-domain` directly (route through `ito-core`)

## Domain Purity (`ito-domain`)

`arch_guardrails.py` enforces baseline-count rules for domain purity:

- **`miette::`**: zero tolerance in `ito-domain`
- **`std::fs`**: baseline-constrained (no net-new usage beyond approved files/counts)
- **`std::process::Command`**: baseline-constrained (no net-new usage beyond approved files/counts)

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
prek install -t commit-msg              # Conventional commit message check
prek install -t pre-push                # Full quality gate
prek run --all-files --stage pre-push   # Run full gate locally
```

**Hook stages**:

- `commit-msg`: conventional commit message validation
- `pre-push`: full quality gate (format, lint, docs, tests, coverage, guardrails, etc.)
- `pre-commit`: lightweight repository validation via `ito validate repo --staged --strict`
  (config-aware rule engine; the script lives at `ito-rs/tools/hooks/pre-commit` and is
  wired into `.pre-commit-config.yaml` as the `ito-validate-repo` local hook). The heavier
  quality gates remain at pre-push. To bypass for an emergency commit:
  `git commit --no-verify`.

### Agent Commit Workflow

Use a check-then-commit flow:

```bash
# 1. Run full checks up front
make check

# 2. Stage and commit normally (keep commit-msg hook active)
git add <files>
git commit -m "type(scope): description"

# 3. Push (pre-push hook runs)
git push
```

If hooks auto-fix files and a push aborts, immediately run `git status`, stage hook-generated changes, create a follow-up commit, then push again.

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
| Before committing | `codex-review` | Reviews the diff for quality, correctness, and convention adherence (pair with `rust-code-reviewer` for a second opinion on non-trivial changes) |
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
4. **Before committing**: Run `codex-review` on the diff (and `rust-code-reviewer` for a second pass on non-trivial changes), plus `documentation-police` on any new public APIs
