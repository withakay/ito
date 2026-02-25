<!-- ITO:START -->

# Shared Guidance

This file is for optional, user-authored guidance shared across instruction artifacts.

- Ito may update this header block over time.
- Add your shared guidance below the `<!-- ITO:END -->` marker.

<!-- ITO:END -->

## Project Guidance

### Rust Code Quality

After modifying Rust code, dispatch these subagents **in parallel**:
- @code-simplifier - Refactors for clarity per `.ito/user-rust-style.md`
- @documentation-police - Ensures public APIs have useful docs
- @rust-code-reviewer - Checks for idiomatic usage, error handling, and best practices

Then run `make check` to verify.

### Running test and checks

Always use the test-with-subagent skill for running builds, tests and checks.

### Commits

Make small, focused commits with clear messages.
Regularly use the `ito-commit` skill for conventional commits aligned with the project's commit message guidelines.
IF you have to do more work to make changes that don't break the build whilst remaining small and focused, so be it.

### Subagent Collaboration

Subagents are first-class tools in this repo. Prefer delegating independent work to specialist subagents (often in parallel), then synthesize the results.

Diversity is good: for non-trivial changes, get at least two independent review passes (for example: `@rust-code-reviewer` + `@codex-review`).

Commonly useful subagents:

- `@explore` - fast codebase navigation/search
- `@test-runner` - runs `make test` / `make check` with curated output
- `@rust-quality-checker` - Rust style/idioms/conventions checks
- `@rust-code-reviewer` - Rust-focused review (safety/idioms/architecture)
- `@rust-test-engineer` - test strategy and coverage design
- `@codex-review` - diff review for correctness and edge cases
- `@documentation-police` - docs coverage/quality
- `@code-simplifier` - refactor for clarity and maintainability
- `@code-quality-squad` - parallel Rust quality workflows
- `@perplexity-researcher` / `@perplexity-researcher-pro` - web research with citations
- `@multi-agent` - explore multiple approaches and synthesize
