<!-- ITO:START -->
# Ito Instructions

These instructions are for AI assistants working in this project.

Always open `@/.ito/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/.ito/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Project setup: run `/ito-project-setup` (or `ito agent instruction project-setup`) until `.ito/project.md` is marked `<!-- ITO:PROJECT_SETUP:COMPLETE -->`.

Note: Files under `.ito/`, `.opencode/`, `.github/`, and `.codex/` are installed/updated by Ito (`ito init`, `ito update`) and may be overwritten.
Add project-specific guidance in `.ito/user-prompts/guidance.md` (shared), `.ito/user-prompts/<artifact>.md` (artifact-specific), and/or below this managed block.

Keep this managed block so `ito init --upgrade` can refresh the managed instructions non-destructively.
To refresh only the Ito-managed content in this file, run: `ito init --upgrade`

<!-- ITO:END -->

## Rust Coding Style

When working in the Rust codebase, follow the project's style guide at `.ito/user-rust-style.md`. Key conventions:

- Control flow: for loops over iterators, let-else for early returns
- Pattern matching: if-let chains, explicit matching, no wildcards
- Variable naming: shadowing over renaming
- Type safety: newtypes, enums over bools
- Documentation: all public APIs must be documented with genuinely useful context

Run `make docs` to build documentation and catch missing doc comments.
