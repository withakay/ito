# Tasks for: 000-09_add-cli-aliases

## Execution Notes

- **Tool**: OpenCode, Codex, Claude Code
- **Mode**: Sequential
- **Template**: Enhanced task format with waves
- **Tracking**: Use `ito tasks` CLI commands

```bash
ito tasks status 000-09_add-cli-aliases
ito tasks next 000-09_add-cli-aliases
ito tasks start 000-09_add-cli-aliases 1.1
ito tasks complete 000-09_add-cli-aliases 1.1
```

______________________________________________________________________

## Wave 1: Main Command Aliases

- **Depends On**: None

### Task 1.1: Add aliases to Commands enum

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: None
- **Action**: Add `visible_alias` attributes to all Commands enum variants (cr, ls, sh, st, va, ar, ts, pl, sa, ag, co, in, up, au, ra, cp, se, ss, he)
- **Verify**: `cargo build -p ito-cli` succeeds
- **Done When**: All main commands have 2-letter aliases and build passes
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 1.2: Add aliases to CreateAction enum

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: Task 1.1
- **Action**: Add `visible_alias = "mo"` to Module and `visible_alias = "ch"` to Change
- **Verify**: `cargo build -p ito-cli` succeeds
- **Done When**: Create subcommands have aliases and build passes
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 1.3: Add aliases to TasksAction enum

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: Task 1.1
- **Action**: Add aliases: in, st, nx, rd, go, co, sv, us, ad, sw
- **Verify**: `cargo build -p ito-cli` succeeds
- **Done When**: All tasks subcommands have aliases and build passes
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Wave 2: Subcommand Aliases

- **Depends On**: Wave 1

### Task 2.1: Add aliases to StateAction enum

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: Task 1.1
- **Action**: Add aliases: sw, de, bl, no, fo, qu
- **Verify**: `cargo build -p ito-cli` succeeds
- **Done When**: State subcommands have aliases and build passes
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 2.2: Add aliases to AgentCommand enum

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: Task 1.1
- **Action**: Add alias: in (for instruction)
- **Verify**: `cargo build -p ito-cli` succeeds
- **Done When**: Agent subcommands have aliases and build passes
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 2.3: Add aliases to PlanAction enum

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: Task 1.1
- **Action**: Add aliases: in, st
- **Verify**: `cargo build -p ito-cli` succeeds
- **Done When**: Plan subcommands have aliases and build passes
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 2.4: Add aliases to ConfigCommand enum

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: Task 1.1
- **Action**: Add aliases: pa, ls, ge, se, un, sc
- **Verify**: `cargo build -p ito-cli` succeeds
- **Done When**: Config subcommands have aliases and build passes
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 2.5: Add aliases to AuditCommand enum

- **Files**: `ito-rs/crates/ito-cli/src/commands/audit.rs`
- **Dependencies**: Task 1.1
- **Action**: Add aliases: lo, re, va, st
- **Verify**: `cargo build -p ito-cli` succeeds
- **Done When**: Audit subcommands have aliases and build passes
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Wave 3: Short Flags

- **Depends On**: Wave 2

### Task 3.1: Add -c and -m flags to relevant structs

- **Files**: `ito-rs/crates/ito-cli/src/cli.rs`
- **Dependencies**: Task 2.1, Task 2.2, Task 2.3, Task 2.4, Task 2.5
- **Action**: Add `short = 'c'` to change params and `short = 'm'` to module params in TasksArgs subcommands, AgentInstructionArgs, ArchiveArgs, StatusArgs, ValidateArgs, ShowArgs, RalphArgs, CreateAction::Change
- **Verify**: `cargo build -p ito-cli` succeeds
- **Done When**: All commands support -c and -m short flags where applicable
- **Updated At**: 2026-02-11
- **Status**: [x] complete

______________________________________________________________________

## Wave 4: Testing and Validation

- **Depends On**: Wave 3

### Task 4.1: Run full test suite

- **Files**: All files in ito-cli
- **Dependencies**: Task 3.1
- **Action**: Run `make test` and `make check` to ensure all tests pass and code compiles cleanly
- **Verify**: `make test` passes, `make check` passes
- **Done When**: All tests pass, no linting errors
- **Updated At**: 2026-02-11
- **Status**: [x] complete

### Task 4.2: Manual verification of aliases

- **Files**: None (manual testing)
- **Dependencies**: Task 3.1
- **Action**: Run ito --help and verify aliases appear; test a few key aliases manually
- **Verify**: `ito --help`, `ito ts --help`, `ito ls`, `ito cr ch --help`
- **Done When**: Aliases are visible in help and functional
- **Updated At**: 2026-02-11
- **Status**: [x] complete
- **Resolution Notes**:
  - Initial testing appeared to fail because wrong binary was used
  - Root cause: Was testing with root workspace binary (`/path/to/root/target/debug/ito`) instead of worktree binary (`ito-worktrees/000-09_add-cli-aliases/target/debug/ito`)
  - All aliases work correctly when using the worktree binary
  - Fixed minor inconsistency: Changed `List` command from `alias` to `visible_alias` for consistency
  - Added comprehensive integration tests in `ito-rs/crates/ito-cli/tests/aliases.rs`
  - Updated `AGENTS.md` with guidance on testing in worktrees to prevent this issue in the future

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[>] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
