# Tasks for: 001-10_comprehensive-cli-help-system

## Execution Notes

- **Tool**: Any (OpenCode, Codex, Claude Code)
- **Mode**: Sequential
- **Template**: Enhanced task format with waves, verification, and status tracking
- **Tracking**: Use the tasks CLI to drive status updates and pick work

```bash
ito tasks status 001-10_comprehensive-cli-help-system
ito tasks next 001-10_comprehensive-cli-help-system
ito tasks start 001-10_comprehensive-cli-help-system 1.1
ito tasks complete 001-10_comprehensive-cli-help-system 1.1
```

______________________________________________________________________

## Wave 1: Fix Subcommand Help Routing

- **Depends On**: None

### Task 1.1: Fix agent instruction help routing

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: None
- **Action**:
  In `handle_agent()`, ensure that when `instruction` subcommand is detected, the help check happens on the subcommand args, not the parent args. The pattern:
  1. Extract subcommand args first
  2. Check for help flag in subcommand args
  3. Show `AGENT_INSTRUCTION_HELP` if found
  4. Otherwise proceed with handler
- **Verify**: `ito agent instruction -h` shows instruction-specific help with artifacts list
- **Done When**: `ito agent instruction -h` shows `AGENT_INSTRUCTION_HELP` content
- **Updated At**: 2026-01-31
- **Status**: [x] completed

### Task 1.2: Audit and fix all nested command help routing

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Review all commands with subcommands and ensure help routing is correct:
  - `tasks` (init, status, next, start, complete, shelve, unshelve, add, show)
  - `plan` (init, status)
  - `state` (show, decision, blocker, note, focus, question)
  - `workflow` (init, list, show)
  - `config` (path, list, get, set, unset)
  - `create` (module, change)
  - `show` (module)
  - `validate` (module)
  Apply the same fix pattern as Task 1.1 where needed.
- **Verify**: Test `-h` on several subcommands: `ito tasks status -h`, `ito config get -h`
- **Done When**: All subcommands show their own help when `-h` is passed
- **Updated At**: 2026-01-31
- **Status**: [x] completed

______________________________________________________________________

## Wave 2: Add Help All Dump

- **Depends On**: Wave 1

### Task 2.1: Create help dump data structure

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: None
- **Action**:
  Create a struct or vector that collects all help texts in order. This could be:
  ```rust
  struct CommandHelp {
      path: &'static str,  // e.g., "ito agent instruction"
      help: &'static str,  // the help constant
  }

  const ALL_HELP: &[CommandHelp] = &[
      CommandHelp { path: "ito", help: HELP },
      CommandHelp { path: "ito init", help: INIT_HELP },
      // ...
  ];
  ```
- **Verify**: The data structure compiles and contains all commands
- **Done When**: `ALL_HELP` constant defined with all command paths and help texts
- **Updated At**: 2026-01-31
- **Status**: [x] completed

### Task 2.2: Implement help --all command

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: Task 2.1
- **Action**:
  Add handling for `ito help --all`:
  1. Check if first arg is "help" and second is "--all"
  2. Iterate through `ALL_HELP` and print each with separator
  3. Format with headers showing command path
- **Verify**: `ito help --all | head -100` shows formatted output
- **Done When**: `ito help --all` outputs complete CLI reference
- **Updated At**: 2026-01-31
- **Status**: [x] completed

### Task 2.3: Add --help-all global flag

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: Task 2.2
- **Action**:
  Add handling for `ito --help-all` as an alias:
  1. Check if first arg is "--help-all"
  2. Call the same function as `help --all`
- **Verify**: `ito --help-all | head -100` shows same output as `ito help --all`
- **Done When**: Both forms work identically
- **Updated At**: 2026-01-31
- **Status**: [x] completed

### Task 2.4: Add JSON output for help dump

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: Task 2.2
- **Action**:
  Add `--json` flag support to `help --all`:
  1. Parse help constants to extract structure (or maintain a separate structured version)
  2. Output as JSON with commands, options, subcommands
  3. Consider using serde for serialization
- **Verify**: `ito help --all --json | jq '.commands[0].name'` returns valid JSON
- **Done When**: JSON output includes all commands with their options
- **Updated At**: 2026-01-31
- **Status**: [x] completed

______________________________________________________________________

## Wave 3: Improve Help Text

- **Depends On**: Wave 2

### Task 3.1: Add navigation footer to help constants

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: None
- **Action**:
  Update each `*_HELP` constant to include appropriate footer:
  - For commands with subcommands: `\n\nRun 'ito <cmd> <subcmd> -h' for subcommand options.`
  - For leaf commands: `\n\nRun 'ito -h' to see all commands.`
  - For top-level: `\n\nRun 'ito <command> -h' for command options, or 'ito help --all' for complete reference.`
- **Verify**: `ito -h` shows footer hint
- **Done When**: All help outputs include navigation hints
- **Updated At**: 2026-01-31
- **Status**: [x] completed

### Task 3.2: Update top-level HELP with better option hints

- **Files**: `ito-rs/crates/ito-cli/src/main.rs`
- **Dependencies**: None
- **Action**:
  Update the main `HELP` constant to show key options inline for common commands. For example:
  ```
  list [--json|--specs|--modules]   List items (changes by default)
  init [--tools <...>] [path]       Initialize Ito in your project
  ```
  Focus on the most commonly used 5-6 commands.
- **Verify**: `ito -h` shows option hints for key commands
- **Done When**: Top-level help shows inline option hints
- **Updated At**: 2026-01-31
- **Status**: [x] completed

______________________________________________________________________

## Wave 4: Testing & Validation

- **Depends On**: Wave 3

### Task 4.1: Add tests for help system

- **Files**: `ito-rs/crates/ito-cli/tests/` or integration tests
- **Dependencies**: None
- **Action**:
  Add tests that verify:
  1. `ito agent instruction -h` shows instruction help (not agent help)
  2. `ito help --all` outputs non-empty content
  3. `ito --help-all` outputs same as `ito help --all`
  4. `ito help --all --json` outputs valid JSON
- **Verify**: `cargo test -p ito-cli`
- **Done When**: All new tests pass
- **Updated At**: 2026-01-31
- **Status**: [x] completed

### Task 4.2: Manual validation of help walkthrough

- **Files**: None (manual testing)
- **Dependencies**: Task 4.1
- **Action**:
  Walk through the entire command tree with `-h`:
  1. Start at `ito -h`
  2. For each command, run `ito <cmd> -h`
  3. For each subcommand, run `ito <cmd> <subcmd> -h`
  4. Verify all show appropriate help with footers
  5. Test `ito help --all` outputs complete reference
- **Verify**: Manual verification
- **Done When**: All commands show correct help at every level
- **Updated At**: 2026-01-31
- **Status**: [x] completed

______________________________________________________________________

## Task Status Legend

- `[ ] pending` - Not started yet
- `[ ] in-progress` - Currently being worked on
- `[x] complete` - Finished and verified
- `[-] shelved` - Intentionally not-to-be-done (reversible)
