# Proposal: Add CLI Aliases and Short Flags

## Why

Currently, users must type full command names for common ito operations. This creates friction in the CLI workflow, especially for frequently-used commands like `tasks`, `agent`, and `create`. Adding aliases and short flags will make the CLI more ergonomic and faster to use.

Examples of the current friction:
- `ito tasks start 005-01 1.1` - long command for a frequent operation
- `ito agent instruction proposal --change 005-01` - repetitive typing
- `ito create change --module 005 my-change` - verbose for common patterns

## What

Add 2-letter aliases for all commands and subcommands, plus short flags `-c` for `--change` and `-m` for `--module`.

### Command Aliases

| Command | Alias | Subcommands (with aliases) |
|---------|-------|---------------------------|
| `create` | `cr` | `module` → `mo`, `change` → `ch` |
| `list` | `ls` | - |
| `show` | `sh` | `module` → `mo` |
| `status` | `st` | - |
| `validate` | `va` | `module` → `mo` |
| `archive` | `ar` | - |
| `tasks` | `ts` | `init` → `in`, `status` → `st`, `next` → `nx`, `ready` → `rd`, `start` → `go`, `complete` → `co`, `shelve` → `sv`, `unshelve` → `us`, `add` → `ad`, `show` → `sw` |
| `plan` | `pl` | `init` → `in`, `status` → `st` |
| `state` | `sa` | `show` → `sw`, `decision` → `de`, `blocker` → `bl`, `note` → `no`, `focus` → `fo`, `question` → `qu` |
| `agent` | `ag` | `instruction` → `in` |
| `config` | `co` | `path` → `pa`, `list` → `ls`, `get` → `ge`, `set` → `se`, `unset` → `un`, `schema` → `sc` |
| `init` | `in` | - |
| `update` | `up` | - |
| `audit` | `au` | `log` → `lo`, `reconcile` → `re`, `validate` → `va`, `stats` → `st` |
| `ralph` | `ra` | - |
| `completions` | `cp` | - |
| `serve` | `se` | `start` → `st` |
| `stats` | `ss` | - |
| `help` | `he` | - |

### Short Flags

| Flag | Maps to | Applies to |
|------|---------|------------|
| `-c` | `--change` | `tasks`, `agent instruction`, `status`, `archive`, `validate`, `show`, `ralph` |
| `-m` | `--module` | `create change`, `ralph`, `list`, `show`, `validate` |

### Usage Examples

```bash
# Creating changes
ito cr ch -m 005 my-feature          # ito create change --module 005 my-feature
ito cr mo my-module                  # ito create module my-module

# Task management
ito ts go -c 005-01 1.1              # ito tasks start --change 005-01 1.1
ito ts co -c 005-01 1.1              # ito tasks complete --change 005-01 1.1
ito ts rd -c 005-01                  # ito tasks ready --change 005-01

# Agent instructions
ito ag in pr -c 005-01               # ito agent instruction proposal --change 005-01
ito ag in ap -c 005-01               # ito agent instruction apply --change 005-01

# State management
ito sa de "Using JWT tokens"         # ito state decision "Using JWT tokens"
ito sa bl "Waiting for API"          # ito state blocker "Waiting for API"

# Config
ito co ge defaults.schema            # ito config get defaults.schema
ito co se defaults.schema minimal    # ito config set defaults.schema minimal
```

## Impact

- **User experience**: Significantly faster CLI interactions for power users
- **Documentation**: All aliases must be discoverable via `--help`
- **Maintenance**: Aliases are defined declaratively in clap; minimal maintenance overhead
- **Testing**: Unit tests should verify aliases work correctly

## Design Notes

Implementation uses clap's built-in `visible_alias` attribute:

```rust
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(visible_alias = "cr")]
    Create(CreateArgs),
    #[command(visible_alias = "ls")]
    List(ListArgs),
    // ...
}
```

And for subcommands:

```rust
#[derive(Subcommand, Debug, Clone)]
pub enum TasksAction {
    #[command(visible_alias = "go")]
    Start { ... },
    #[command(visible_alias = "co")]
    Complete { ... },
    // ...
}
```

Short flags use clap's `short` attribute:

```rust
#[arg(short = 'c', long)]
pub change: Option<String>,
#[arg(short = 'm', long)]
pub module: Option<String>,
```

## Alternatives Considered

1. **Single-letter aliases**: Rejected - too cryptic and harder to remember
2. **Custom abbreviations**: Rejected - clap handles this natively with visible_alias
3. **Shell completions only**: Rejected - aliases are faster even with completions

## Success Criteria

- [ ] All commands have 2-letter aliases
- [ ] All subcommands have 2-letter aliases
- [ ] `-c` and `-m` short flags work on relevant commands
- [ ] Aliases appear in `--help` output
- [ ] Tests verify alias functionality
- [ ] Documentation updated to show alias examples
