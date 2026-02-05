## Context

The ito CLI (`ito-rs/crates/ito-cli/`) currently uses a 100% hand-rolled argument parsing implementation:

- **`app/mod.rs`**: ~345 lines of manual `std::env::args()` parsing with string matching
- **`app/help.rs`**: ~500 lines of hardcoded help text constants
- **`commands/*.rs`**: Each command has its own manual parsing logic

This approach has served the project but creates significant maintenance burden and prevents access to modern CLI features like shell completions.

**Stakeholders**: All ito CLI users and contributors.

## Goals / Non-Goals

**Goals:**

- Replace hand-rolled parsing with clap derive API
- Eliminate all manual help text constants
- Add shell completion generation for bash/zsh/fish/powershell
- Preserve all existing command names, flags, and behaviors (no breaking changes)
- Improve UX with styled, colored help output

**Non-Goals:**

- Changing the command structure or adding new commands (beyond `completions`)
- Modifying command handler logic (only the parsing layer changes)
- Adding interactive prompts or TUI features
- Supporting additional shells beyond bash/zsh/fish/powershell

## Decisions

### Decision 1: Use clap derive API (not builder API)

**Choice**: Use `#[derive(Parser)]` and `#[derive(Subcommand)]` macros.

**Rationale**:
- Derive API provides the cleanest, most declarative syntax
- Doc comments automatically become help text (zero duplication)
- Type-safe by default
- Aligns with Rust ecosystem conventions

**Alternatives considered**:
- Builder API: More verbose, requires explicit help strings, no benefit for our use case
- Maintain hand-rolled: Does not solve the boilerplate problem

### Decision 2: Incremental migration command-by-command

**Choice**: Migrate one subcommand at a time, starting with `tasks`.

**Rationale**:
- Reduces risk of breaking changes
- Allows validation of each command before proceeding
- `tasks` is the most complex command, making it the best stress test
- Can land partial progress without blocking other work

**Migration order**:
1. `tasks` (most complex, validates approach)
2. `create` (moderate complexity)
3. `list`, `show` (simple)
4. `init`, `config`, `help` (simple)
5. `agent` (moderate complexity)

### Decision 3: Top-level CLI structure

**Choice**: Single `Cli` struct with `Commands` enum.

```rust
#[derive(Parser)]
#[command(name = "ito", version, about = "Ito - structured software change workflow")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize ito in a project or home directory
    Init(InitArgs),
    /// Create changes, modules, or specs
    Create(CreateArgs),
    /// List changes, modules, or specs
    List(ListArgs),
    // ... etc
}
```

**Rationale**: Standard clap pattern, clean separation of concerns.

### Decision 4: Shell completions via clap_complete

**Choice**: Add `completions` subcommand using `clap_complete` crate.

```rust
#[derive(Subcommand)]
pub enum Commands {
    /// Generate shell completion scripts
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
    // ...
}
```

**Rationale**:
- `clap_complete` integrates seamlessly with clap
- Supports all major shells out of the box
- Completion scripts stay in sync with CLI structure automatically

### Decision 5: Preserve existing handler function signatures

**Choice**: Keep existing `handle_*` functions, change only how they're called.

**Rationale**:
- Minimizes risk during migration
- Command logic is already tested and working
- Allows incremental migration without touching business logic

**Pattern**:
```rust
// Before (manual dispatch)
match command.as_str() {
    "tasks" => handle_tasks(args),
    _ => Err(...),
}

// After (clap dispatch)
match cli.command {
    Commands::Tasks(args) => handle_tasks(args),
    // ...
}
```

### Decision 6: Help text styling

**Choice**: Use clap's built-in `Styles` API for consistent colored output.

```rust
fn styles() -> Styles {
    Styles::styled()
        .header(Style::new().bold())
        .usage(Style::new().bold())
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
}
```

**Rationale**: Provides professional UX with minimal code. Respects `NO_COLOR` automatically.

## Risks / Trade-offs

**[Risk] Behavior regression**
→ Mitigation: Add snapshot tests for all command help outputs before migration. Run existing integration tests after each command migration.

**[Risk] Compile time increase (~2-5s)**
→ Mitigation: Acceptable tradeoff for maintenance benefits. Can optimize later with workspace-level caching.

**[Risk] Learning curve for contributors**
→ Mitigation: Clap derive is well-documented and widely used in Rust ecosystem. Most contributors already familiar.

**[Risk] Help text formatting differences**
→ Mitigation: Doc comments can be tuned to match existing output. Minor formatting differences are acceptable if content is equivalent.

## Migration Plan

### Phase 1: Setup (Non-breaking)
1. Add `clap` and `clap_complete` to `Cargo.toml`
2. Add snapshot tests for current CLI output
3. Create `Cli` struct with `Commands` enum alongside existing parsing

### Phase 2: Migrate Commands (One at a time)
1. Convert command to clap struct (e.g., `TasksArgs`)
2. Update dispatch to use new struct
3. Delete old manual parsing for that command
4. Verify snapshot tests pass (or update if formatting changes are intentional)
5. Repeat for next command

### Phase 3: Cleanup
1. Delete `app/help.rs` (all constants)
2. Delete manual parsing utilities in `app/mod.rs`
3. Add `completions` subcommand

### Rollback Strategy
- Each command migration is atomic; can revert individual commits
- Keep old parsing code until all commands migrated
- Version-controlled; can revert entire branch if needed

## Open Questions

None - design is straightforward and follows standard clap patterns.
