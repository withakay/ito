<!-- ITO:START -->

# User Guidance

This file is for optional, user-authored guidance that Ito will inject into `ito agent instruction <artifact>` outputs.

Use this file for project-specific preferences (style, constraints, defaults). Avoid editing tool prompt files directly (`.opencode/`, `.github/`, `.codex/`, `.claude/`) unless you intend to maintain those changes across `ito update`.

- Ito may update this header block over time.
- Add your guidance below the `<!-- ITO:END -->` marker.

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

Use the `ito-commit` skill for conventional commits aligned to changes. Release-plz handles versioning automatically based on commit history.
