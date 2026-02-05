<!-- ITO:START -->

# User Guidance

This file is for optional, user-authored guidance that Ito will inject into `ito agent instruction <artifact>` outputs.

Use this file for project-specific preferences (style, constraints, defaults). Avoid editing tool prompt files directly (`.opencode/`, `.github/`, `.codex/`, `.claude/`) unless you intend to maintain those changes across `ito update`.

- Ito may update this header block over time.
- Add your guidance below the `<!-- ITO:END -->` marker.

<!-- ITO:END -->

## Your Guidance

### Use agents and Subagents whenever possible

Always attempt to make use of subagents to delegate tasks to. Try and use appropriate subagents for a given task, but if you are not sure use a general agent. This helps reduce load and manage context to improve efficiency and focus.

### Proposing Changes

- When proposing a change that modifies Rust code, ensure that the proposal adheres to Rust coding conventions and best practices.
- Use the `rust-style` skill to check that any proposed changes conform to established Rust formatting and linting rules.
- Research subject matter thoroughly to ensure that proposed changes are well-informed and justified.

### Applying Changes

- When a change proposal is implemented (AKA applied) use the `rust-style` skill to ensure that the linting and formatting rules are followed.
- Ask the @code-simplifier subagent to simplify and refine any Rust code that has been modified as part of the change proposal implementation. This helps ensure that all code adheres to project coding standards and best practices.
- **Always** run `make check` after applying changes to verify code quality and functionality. `make check` runs tests, linters, and other quality checks to ensure the codebase remains stable.

### Archiving Changes

- When a change proposal is archived run `make version-bump-patch` to increase the patch portion of the version string.
- Typically archiving a change means it was completed, any commits around this should respect the conventional commit format and treat this as a new feature `feat(scope): Foo implemented`

### Git commits

- Always use conventional commit style.
