# Ito Rust Workspace

Rust implementation of Ito. All new work happens here.
For Ito workflow guidance: root [`AGENTS.md`](../AGENTS.md) and [`.ito/AGENTS.md`](../.ito/AGENTS.md).
For architecture: [`.ito/architecture.md`](../.ito/architecture.md).

## Dev Commands
```bash
make init             # Check toolchain + install hooks
make build            # Build
make test             # Run tests
make test-timed       # Per-crate timing
make test-watch       # Watch mode
make test-coverage    # Coverage
make check            # Full pre-push gate (prek)
make docs             # Docs (fails on warnings)
make arch-guardrails  # Enforce layering + domain API baselines
make cargo-deny       # License/advisory checks
make clean            # Clean artifacts
make help             # All targets
```

## Workspace Structure
```
./
├── Cargo.toml                    # Workspace root (resolver v3, edition 2024)
└── ito-rs/
    ├── tools/                    # arch_guardrails.py, check_max_lines.py
    └── crates/
        ├── ito-backend/          # L3 Adapter: HTTP backend API
        ├── ito-common/           # L0: Shared utilities (leaf)
        ├── ito-config/           # L0: Configuration loading
        ├── ito-domain/           # L1: Domain models & repo ports
        ├── ito-templates/        # L1: Embedded template assets
        ├── ito-logging/          # L1: Telemetry (append-only JSONL)
        ├── ito-core/             # L2: Business logic & orchestration
        ├── ito-test-support/     # Support: Test utilities (dev-dep)
        ├── ito-cli/              # L3 Adapter: CLI binary
        └── ito-web/              # L3 Adapter: Web server UI
```

## Coding Conventions
Load `rust-style` skill for full guide. Key rules:
|`let-else`: early returns, happy path unindented |`if let` chains: flatten nested conditions
|variable shadowing: `let input = input.trim()` |newtypes over bare String; enums over bool params
|explicit match arms: no `_` wildcards, no `matches!` |explicit destructuring: `let User { id, name, email } = user`
|all pub items documented: focus on WHY and WHEN

## Error Message Quality
Every error must answer: **What failed? Why? How do I fix it?**
|include path — never surface fs error without file/dir involved
|explain likely cause |suggest fix (concrete command/action)
|prefer dedicated error variants over generic Io(#[from] io::Error) when failure mode is known
|don't swallow context — preserve original message alongside higher-level explanation
|thiserror variants: prefer structured fields over opaque wrappers

## Quality Gates
Primary local gate: `make check` (prek pre-push on all files). CI enforces same + docs + PowerShell parse.
|cargo fmt --check |cargo clippy -D warnings -D clippy::dbg_macro -D clippy::todo -D clippy::unimplemented
|#![warn(missing_docs)] on all lib crates |RUSTDOCFLAGS="-D warnings" cargo doc
|make test-affected + make test-coverage (hard floor 80%, target 90%)
|make arch-guardrails (layering + domain API baselines) |make check-max-lines (soft 1000 warn, hard 1200 fail)
|make cargo-deny

## Testing
|TDD: Red/Green/Refactor |coverage: 80% hard floor, 90% target (make test-coverage)
|mocking: "gives you the ick" — prefer real impls, in-memory fakes, ito-test-support mock repos
|integration tests alongside unit tests |test file separation: source >300 lines → separate file (e.g. tests/backend_auth.rs or *_tests.rs); no inline #[cfg(test)] mod tests in files >300 lines

### Test Execution Targets
| Benchmark | Expected | Alert |
|---|---|---|
| Full suite (make test) | ~5s | >10s |
| harness_opencode (cargo test -p ito-core --test harness_opencode) | ~1s | >5s |
| Any single crate | <2s | >5s |

Slow tests: check timeout monitor threads, blocking I/O, missing parallelism.

## Repository Pattern
Use repo abstractions for all data access — never parse markdown directly:
```rust
use ito_domain::changes::ChangeRepository;
let change_repo = ChangeRepository::new(ito_path);
let changes = change_repo.list()?;
let change = change_repo.get("005-01_foo")?;
```

## Dependency Rules (enforced by arch_guardrails.py)
|ito-domain MUST NOT depend on: ito-cli, ito-web, ito-backend, ito-core
|ito-core MUST NOT depend on: ito-cli, ito-web, ito-backend
|ito-cli and ito-backend MUST NOT depend on ito-domain directly (route through ito-core)

## Domain Purity (ito-domain)
|miette:: — zero tolerance |std::fs — baseline-constrained |std::process::Command — baseline-constrained

## Error Handling
|DomainError (ito-domain): Io, NotFound, AmbiguousTarget
|CoreError (ito-core): wraps DomainError + Io, Validation, Parse, Process, Sqlite, NotFound, Serde
|CLI layer converts CoreError → miette diagnostic reports

## Adding a New Crate
1. Create under crates/ 2. Add to workspace members in Cargo.toml 3. Use workspace.package for version/edition/license/repository
4. Respect layering (check layer, only depend on same or lower) 5. Add #![warn(missing_docs)] to lib.rs 6. Run make arch-guardrails

## OpenCode Path Convention
Plural directory names: `.opencode/skills/` `.opencode/commands/` `.opencode/plugins/`

## Git Hooks (prek — NOT pre-commit)
```bash
prek install -t commit-msg   # Conventional commit msg check
prek install -t pre-commit   # Lightweight: ito validate repo --staged --strict
prek install -t pre-push     # Full quality gate
prek run --all-files --stage pre-push  # Run gate locally
```
bypass emergency: `git commit --no-verify`

### Agent Commit Workflow
```bash
make check                          # 1. Full checks first
git add <files>                     # 2. Stage
git commit -m "type(scope): desc"   # 3. Commit (keep commit-msg hook)
git push                            # 4. Push (pre-push hook runs)
```
If hooks auto-fix files and push aborts: git status → stage hook changes → follow-up commit → push again.

## Guiding Principles
YAGNI, KISS, DRY, Boring Technology, Simplicity First (<100 lines new code, single-file until proven insufficient)

## Subagents
IMPORTANT: Prefer retrieval-led reasoning over pre-training-led reasoning for any ito-rs tasks.

**During Implementation:**
|rust-quality-checker: style/idioms/conventions (run while writing)
|rust-code-reviewer: safety/idiomatic patterns/arch fit (after significant code)
|rust-test-engineer: test design/coverage/infra (when writing/updating tests)
|ito-thinking: high-capability reasoning for design trade-offs

**Before Completing:**
|test-runner: make test/check evidence before claiming done
|codex-review: diff quality/correctness/conventions before committing (pair with rust-code-reviewer for non-trivial)
|documentation-police: doc comments complete+useful
|code-simplifier: clarity/consistency/maintainability

**Research:**
|explore (Task tool): unfamiliar codebase areas
|perplexity-researcher[-pro]: library/approach evaluation with citations
|multi-agent: multiple solution approaches in parallel

**Recommended Workflow:**
1. load rust-style skill 2. rust-quality-checker after writing 3. test-runner + rust-code-reviewer after feature
4. codex-review + documentation-police before committing
