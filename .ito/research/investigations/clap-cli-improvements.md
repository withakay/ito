# Research: Clap CLI Improvements

**Focus**: Features (reducing boilerplate, making a slick CLI)
**Date**: 2026-02-01
**Status**: Complete

## Current State Analysis

The ito CLI (`ito-rs/crates/ito-cli/`) is **entirely hand-rolled** with zero clap usage:

| Aspect | Current Approach | Lines of Code |
|--------|------------------|---------------|
| Argument parsing | `std::env::args()` + manual string matching | ~200+ |
| Help text | Const strings (`HELP`, `TASKS_HELP`, etc.) | ~500+ |
| Subcommands | `match args.first()` | ~100+ |
| Flag parsing | `util::parse_string_flag()` | ~50+ |
| Error messages | Manual formatting | Scattered |

### Pain Points Identified

1. **Duplicate help text**: Every command has hand-written help strings that duplicate info clap would auto-generate
2. **Fragile parsing**: String matching like `args.first() == Some(&"tasks".to_string())` is error-prone
3. **No completions**: Shell tab-completion requires clap or manual shell scripts
4. **No type safety**: Arguments are strings until manually converted
5. **Inconsistent UX**: Help formatting varies across commands
6. **Testing burden**: Manual parsing logic needs extensive testing that clap provides for free

## Clap Features to Leverage

### 1. Derive API (Primary Recommendation)

Replace hand-rolled parsing with derive macros:

```rust
// Before: ~50 lines of manual parsing
let command = args.first().ok_or("no command")?;
let change_id = args.get(1).ok_or("no change-id")?;
// ... more manual parsing ...

// After: ~10 lines with derive
#[derive(Parser)]
#[command(name = "ito", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage tasks for a change
    Tasks {
        #[command(subcommand)]
        action: TasksAction,
    },
    // ...
}

#[derive(Subcommand)]
enum TasksAction {
    /// Start a task
    Start {
        /// The change ID
        change_id: String,
        /// The task ID to start
        task_id: String,
    },
    // ...
}
```

**Boilerplate reduction**: ~70% fewer lines for argument parsing

### 2. Automatic Help Generation

Delete `HELP`, `LIST_HELP`, `TASKS_HELP`, etc. constants:

```rust
// Before: 40 lines of help text
pub const TASKS_HELP: &str = r#"
ito tasks - Task management commands

USAGE:
    ito tasks <COMMAND>
...
"#;

// After: Zero lines (clap generates from doc comments + type info)
/// Task management commands
#[derive(Subcommand)]
enum TasksAction {
    /// Start working on a task
    Start { /* ... */ },
    /// Mark a task as complete
    Complete { /* ... */ },
}
```

**Boilerplate reduction**: ~500 lines of help text eliminated

### 3. ValueEnum for Constrained Choices

```rust
#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Json,
    Text,
    Markdown,
}

#[derive(Args)]
struct OutputArgs {
    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}
```

### 4. Shell Completions via clap_complete

```rust
// Add to Cargo.toml
// clap_complete = "4"

use clap_complete::{generate, Shell};

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, "ito", &mut io::stdout());
}

// CLI usage: ito completions bash > ~/.bash_completion.d/ito
```

**User experience boost**: Tab completion for all commands, subcommands, and flags

### 5. Man Page Generation via clap_mangen

```rust
// Add to Cargo.toml
// clap_mangen = "0.2"

use clap_mangen::Man;

fn generate_man_page(cmd: &Command) -> String {
    let man = Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
```

### 6. Argument Groups and Conflicts

```rust
#[derive(Args)]
#[group(required = true, multiple = false)]
struct Target {
    /// Specify change by ID
    #[arg(long)]
    change_id: Option<String>,

    /// Use active change
    #[arg(long)]
    active: bool,
}
```

### 7. Global Arguments via flatten

```rust
#[derive(Args)]
struct GlobalArgs {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Suppress output
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    global: GlobalArgs,

    #[command(subcommand)]
    command: Commands,
}
```

### 8. Custom Value Parsers

```rust
fn parse_change_id(s: &str) -> Result<ChangeId, String> {
    ChangeId::parse(s).map_err(|e| e.to_string())
}

#[derive(Args)]
struct StartArgs {
    #[arg(value_parser = parse_change_id)]
    change_id: ChangeId,
}
```

### 9. Environment Variable Fallbacks

```rust
#[derive(Args)]
struct ConfigArgs {
    /// Ito home directory
    #[arg(long, env = "ITO_HOME")]
    home: Option<PathBuf>,
}
```

### 10. Styling and Colors via Styles

```rust
use clap::builder::Styles;
use anstyle::{Style, Color, AnsiColor};

fn styles() -> Styles {
    Styles::styled()
        .header(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .usage(Style::new().bold())
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
}

#[derive(Parser)]
#[command(styles = styles())]
struct Cli { /* ... */ }
```

## Migration Strategy

### Phase 1: Core Structure (Low Risk)

1. Add clap to Cargo.toml
2. Create `Cli` struct with `#[derive(Parser)]`
3. Define `Commands` enum with `#[derive(Subcommand)]`
4. Keep existing handler functions, just change how they're called

### Phase 2: Subcommand Migration (Medium Risk)

Migrate one subcommand at a time:
1. `ito tasks` - most complex, best test case
2. `ito create` - moderate complexity
3. `ito list`, `ito show` - simpler
4. Continue until all commands migrated

### Phase 3: Enhanced Features (Low Risk)

1. Add `clap_complete` for shell completions
2. Add custom styles for branded output
3. Add env var fallbacks where appropriate

### Phase 4: Cleanup

1. Delete `help.rs` (all const strings)
2. Delete manual parsing utilities
3. Delete old `app/mod.rs` dispatch logic

## Estimated Impact

| Metric | Before | After | Reduction |
|--------|--------|-------|-----------|
| Parsing code | ~400 lines | ~100 lines | 75% |
| Help text | ~500 lines | 0 lines | 100% |
| Test coverage needed | High | Low (clap-tested) | 60% |
| New features | Manual | Free (completions, man pages) | N/A |

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Behavior changes | Snapshot test current output, compare after migration |
| Compile time increase | clap adds ~2-5s; acceptable tradeoff |
| Learning curve | Team already knows Rust; clap derive is straightforward |
| Breaking changes | Keep command names and flags identical |

## Recommended Clap Features Summary

| Feature | Crate | Purpose |
|---------|-------|---------|
| Derive macros | `clap` | Replace manual parsing |
| Shell completions | `clap_complete` | Tab completion for bash/zsh/fish/powershell |
| Man pages | `clap_mangen` | Generate man pages from CLI definition |
| Styles | `clap` (built-in) | Branded, colorful help output |
| ValueEnum | `clap` (built-in) | Type-safe enum arguments |
| Env fallback | `clap` (built-in) | Config via environment variables |

## Next Steps

1. **Create change proposal** for clap migration
2. **Start with `ito tasks`** as the pilot command (most complex, best validation)
3. **Add snapshot tests** for current CLI output before migration
4. **Migrate incrementally** - one subcommand per PR
