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

Note: Files under `.ito/`, `.opencode/`, `.github/`, and `.codex/` are installed/updated by Ito (`ito init`, `ito update`) and may be overwritten.
Add project-specific guidance in `.ito/user-guidance.md` (injected into agent instruction outputs) and/or below this managed block.

Keep this managed block so 'ito update' can refresh the instructions.

<!-- ITO:END -->

## Supported Implementation

`ito-rs/` is the supported Ito implementation and should be favored for all new work.

## Prompt Templates

Ito project/home templates are owned by the Rust embedded assets:

- `ito-rs/crates/ito-templates/assets/default/project/`
- `ito-rs/crates/ito-templates/assets/default/home/`

## Rust `ito init` Embedded Markdown

`ito init` (Rust CLI) installs files from embedded assets, not from this repo's checked-in `.opencode/` directory.

- **Shared skills**: `ito-rs/crates/ito-templates/assets/skills/` - installed to all harnesses
- **Shared adapters**: `ito-rs/crates/ito-templates/assets/adapters/` - harness-specific bootstrap files
- **Project templates**: `ito-rs/crates/ito-templates/assets/default/project/` (includes `.ito/`, harness commands/prompts)
- **Home templates**: `ito-rs/crates/ito-templates/assets/default/home/` (e.g., `.codex/...`)
- Assets are embedded via `include_dir!` in `ito-rs/crates/ito-templates/src/lib.rs` and written by `ito-rs/crates/ito-core/src/installers/mod.rs`

If you want agents to learn new workflows (e.g., task tracking), update the embedded skill markdown in those assets.

**See `ito-rs/crates/ito-templates/AGENTS.md` for detailed guidance on maintaining templates and keeping harness files in sync.**

## Development Commands

Use the Makefile for common development tasks:

```bash
# Build the project
make build

# Run tests
make test

# Run tests in watch mode
make test-watch

# Run tests with coverage
make test-coverage

# Run linter checks
make check

# Clean build artifacts
make clean

# Show all available targets
make help
```

The Makefile defaults should reflect the supported Rust workflow. Legacy Bun targets (if present) should be explicitly named.

## Git Hooks with prek

This project uses **[prek](https://github.com/j178/prek)**, a fast Rust-based alternative to `pre-commit`. Despite the name, `prek` uses the same `.pre-commit-config.yaml` configuration file format as Python's `pre-commit` tool.

**Important**: Use `prek` commands, NOT `pre-commit` commands:

```bash
# Install hooks (run after cloning)
prek install                    # Install pre-commit hook
prek install -t pre-push        # Install pre-push hook

# Run hooks manually
prek run                        # Run on staged files
prek run --all-files            # Run on all files
prek run --stage pre-push       # Run pre-push hooks

# Other useful commands
prek list                       # List available hooks
prek auto-update                # Update hook versions
```

**Current hook stages**:

- **pre-commit**: fmt, clippy, test-coverage
- **pre-push**: test-coverage, test

## OpenCode Path Convention

**IMPORTANT**: OpenCode uses **plural** directory names for its configuration paths:

- `.opencode/skills/` (NOT `.opencode/skill/`)
- `.opencode/commands/` (NOT `.opencode/command/`)
- `.opencode/plugins/` (NOT `.opencode/plugin/`)

OpenCode accepts both singular and plural forms (e.g., `{command,commands}/**/*.md`), but the plural form is the documented convention.

When writing tests or code that references OpenCode paths, always use the plural form.

## Repository Pattern for Data Access

The codebase uses a repository pattern for accessing Ito data. When working with changes, modules, or tasks, use the repository abstractions in `ito-domain` rather than direct file I/O:

```rust
use ito_domain::changes::ChangeRepository;
use ito_domain::modules::ModuleRepository;
use ito_domain::tasks::TaskRepository;

// Get a repository instance
let change_repo = ChangeRepository::new(ito_path);
let module_repo = ModuleRepository::new(ito_path);
let task_repo = TaskRepository::new(ito_path);

// Query data through the repository
let changes = change_repo.list()?;           // List all changes
let change = change_repo.get("005-01_foo")?; // Get full change with artifacts
let exists = change_repo.exists("005-01_foo"); // Check existence

let modules = module_repo.list()?;           // List all modules
let module = module_repo.get("005")?;        // Get module by ID

let (completed, total) = task_repo.get_task_counts("005-01_foo")?;
```

**Benefits**:
- Single source of truth for data access
- Hides markdown storage format as implementation detail
- Consistent handling of both checkbox and enhanced task formats
- Rich domain objects with computed properties (status, completeness)

**Do NOT**:
- Parse markdown files directly for change/task data
- Use `core_paths::change_dir()` for data access (use for path construction only)
- Duplicate task counting logic

## Coding Conventions

When working in the Rust codebase, follow the project's Rust style guide at `.ito/user-rust-style.md`. This covers:

- Control flow patterns (for loops over iterators, let-else for early returns)
- Pattern matching (if-let chains, explicit matching, no wildcards)
- Variable naming (shadowing over renaming)
- Type safety (newtypes, enums over bools)
- Documentation requirements (all public APIs must be documented)

You can also use the skill `rust-style` for additional guidance.

**Guiding Principles**:

- YAGNI: Avoid adding features or abstractions until they are needed.
- KISS: Keep it simple and straightforward; prefer clarity over cleverness.
- DRY: Avoid duplication by abstracting common patterns into reusable functions, modules or crates.
- Idiomatic Rust: Follow Rust best practices and conventions for safety, performance, and readability.
- Comprehensive Testing: Write tests for new features and edge cases to ensure reliability.
- Documentation: Document public APIs with genuinely useful context, not perfunctory parameter lists. Run `make docs` to verify.
