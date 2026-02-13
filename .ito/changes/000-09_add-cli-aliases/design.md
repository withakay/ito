# Design: CLI Aliases Implementation

## Overview

Add 2-letter aliases to all CLI commands and subcommands, plus short flags `-c` and `-m`, using clap's built-in `visible_alias` and `short` attributes.

## Implementation Approach

### File Locations

The CLI is primarily defined in `ito-rs/crates/ito-cli/src/cli.rs`, with command-specific definitions in `ito-rs/crates/ito-cli/src/commands/`.

### Main Command Aliases

Add `visible_alias` attributes to each variant in the `Commands` enum:

```rust
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(visible_alias = "cr")]
    Create(CreateArgs),

    #[command(visible_alias = "ls")]
    List(ListArgs),

    #[command(visible_alias = "sh")]
    Show(ShowArgs),

    #[command(visible_alias = "st")]
    Status(StatusArgs),

    #[command(visible_alias = "va")]
    Validate(ValidateArgs),

    #[command(visible_alias = "ar")]
    Archive(ArchiveArgs),

    #[command(visible_alias = "ts")]
    Tasks(TasksArgs),

    #[command(visible_alias = "pl")]
    Plan(PlanArgs),

    #[command(visible_alias = "sa")]
    State(StateArgs),

    #[command(visible_alias = "ag")]
    Agent(AgentArgs),

    #[command(visible_alias = "co")]
    Config(ConfigArgs),

    #[command(visible_alias = "in")]
    Init(InitArgs),

    #[command(visible_alias = "up")]
    Update(UpdateArgs),

    #[command(visible_alias = "au")]
    Audit(crate::commands::audit::AuditArgs),

    #[command(visible_alias = "ra")]
    Ralph(RalphArgs),

    #[command(visible_alias = "cp")]
    Completions(CompletionsArgs),

    #[command(visible_alias = "se")]
    #[cfg(feature = "web")]
    Serve(ServeArgs),

    #[command(visible_alias = "ss")]
    Stats(StatsArgs),

    #[command(visible_alias = "he")]
    Help(HelpArgs),
    // ...
}
```

### Subcommand Aliases

Add aliases to each variant in subcommand enums:

**TasksAction:**
```rust
#[derive(Subcommand, Debug, Clone)]
pub enum TasksAction {
    #[command(visible_alias = "in")]
    Init { ... },

    #[command(visible_alias = "st")]
    Status { ... },

    #[command(visible_alias = "nx")]
    Next { ... },

    #[command(visible_alias = "rd")]
    Ready { ... },

    #[command(visible_alias = "go")]
    Start { ... },

    #[command(visible_alias = "co")]
    Complete { ... },

    #[command(visible_alias = "sv")]
    Shelve { ... },

    #[command(visible_alias = "us")]
    Unshelve { ... },

    #[command(visible_alias = "ad")]
    Add { ... },

    #[command(visible_alias = "sw")]
    Show { ... },
}
```

**CreateAction:**
```rust
#[derive(Subcommand, Debug, Clone)]
pub enum CreateAction {
    #[command(visible_alias = "mo")]
    Module { ... },

    #[command(visible_alias = "ch")]
    Change { ... },
    // ...
}
```

**StateAction:**
```rust
#[derive(Subcommand, Debug, Clone)]
pub enum StateAction {
    #[command(visible_alias = "sw")]
    Show,

    #[command(visible_alias = "de")]
    Decision { ... },

    #[command(visible_alias = "bl")]
    Blocker { ... },

    #[command(visible_alias = "no")]
    Note { ... },

    #[command(visible_alias = "fo")]
    Focus { ... },

    #[command(visible_alias = "qu")]
    Question { ... },
}
```

**AgentCommand:**
```rust
#[derive(Subcommand, Debug, Clone)]
pub enum AgentCommand {
    #[command(visible_alias = "in")]
    Instruction(AgentInstructionArgs),
    // ...
}
```

**PlanAction:**
```rust
#[derive(Subcommand, Debug, Clone)]
pub enum PlanAction {
    #[command(visible_alias = "in")]
    Init,

    #[command(visible_alias = "st")]
    Status,
}
```

**ConfigCommand:**
```rust
#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommand {
    #[command(visible_alias = "pa")]
    Path(ConfigCommonArgs),

    #[command(visible_alias = "ls")]
    List(ConfigCommonArgs),

    #[command(visible_alias = "ge")]
    Get { ... },

    #[command(visible_alias = "se")]
    Set { ... },

    #[command(visible_alias = "un")]
    Unset { ... },

    #[command(visible_alias = "sc")]
    Schema { ... },
    // ...
}
```

**Audit subcommands** (in commands/audit.rs):
```rust
#[derive(Subcommand, Debug, Clone)]
pub enum AuditCommand {
    #[command(visible_alias = "lo")]
    Log(AuditLogArgs),

    #[command(visible_alias = "re")]
    Reconcile(AuditReconcileArgs),

    #[command(visible_alias = "va")]
    Validate(AuditValidateArgs),

    #[command(visible_alias = "st")]
    Stats(AuditStatsArgs),
}
```

### Short Flags

Add `short` attributes to relevant argument fields:

**TasksArgs:**
```rust
#[derive(Args, Debug, Clone)]
pub struct TasksArgs {
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub action: Option<TasksAction>,
}

// In subcommands that take change_id:
pub struct StartArgs {
    #[arg(short = 'c', long)]
    pub change: Option<String>,
    pub task_id: String,
}
```

**AgentInstructionArgs:**
```rust
#[derive(Args, Debug, Clone)]
pub struct AgentInstructionArgs {
    #[arg(short = 'c', long)]
    pub change: Option<String>,
    // ...
}
```

**ArchiveArgs:**
```rust
#[derive(Args, Debug, Clone)]
pub struct ArchiveArgs {
    #[arg(short = 'c', long)]
    pub change: Option<String>,
    // ...
}
```

**StatusArgs:**
```rust
#[derive(Args, Debug, Clone)]
pub struct StatusArgs {
    #[arg(short = 'c', long)]
    pub change: Option<String>,
    // ...
}
```

**ValidateArgs:**
```rust
#[derive(Args, Debug, Clone)]
pub struct ValidateArgs {
    #[arg(short = 'c', long)]
    pub change: Option<String>,
    #[arg(short = 'm', long)]
    pub module: Option<String>,
    // ...
}
```

**ShowArgs:**
```rust
#[derive(Args, Debug, Clone)]
pub struct ShowArgs {
    #[arg(short = 'c', long)]
    pub change: Option<String>,
    #[arg(short = 'm', long)]
    pub module: Option<String>,
    // ...
}
```

**RalphArgs:**
```rust
#[derive(Args, Debug, Clone)]
pub struct RalphArgs {
    #[arg(short = 'c', long)]
    pub change: Option<String>,
    #[arg(short = 'm', long)]
    pub module: Option<String>,
    // ...
}
```

**CreateAction::Change:**
```rust
Change {
    #[arg(short = 'm', long)]
    module: Option<String>,
    // ...
}
```

## Testing Strategy

1. **Unit tests**: Verify each alias resolves to the correct command
2. **Integration tests**: Test end-to-end alias usage via the test harness
3. **Help output tests**: Ensure aliases appear in --help output

## Risks and Mitigations

- **Conflict with existing flags**: Check that `-c` and `-m` aren't already used. Verified: they're not.
- **Alias collision**: Ensure no two commands share the same alias. The 2-letter scheme prevents this.
- **Clap limitations**: `visible_alias` is well-supported; no concerns.

## Dependencies

- `clap` (already in Cargo.toml) - provides `visible_alias` attribute
- No new dependencies required
