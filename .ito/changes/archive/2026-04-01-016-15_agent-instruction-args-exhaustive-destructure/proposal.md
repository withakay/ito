# Change: Replace sync-by-comment with exhaustive destructuring for AgentInstructionArgs

## Why

`reconstruct_agent_args` and `handle_agent_instruction_clap` both manually enumerated every field of `AgentInstructionArgs` to convert it back into a `Vec<String>`. A comment asked developers to "keep in sync" with the struct, but nothing enforced it — adding a new field to the clap struct would silently produce incomplete argument reconstruction and instruction forwarding.

## What Changes

- Add `AgentInstructionArgs::to_argv()` method using exhaustive `let` destructuring so the compiler rejects any struct change that isn't handled
- Replace both manual field-enumeration sites (`reconstruct_agent_args`, `handle_agent_instruction_clap`) with calls to `to_argv()`
- Remove the now-unnecessary `push_optional_flag` helper and the "keep in sync" comment

## Impact

- Affected specs: cli-instructions
- Affected code: `ito-cli/src/cli.rs`, `ito-cli/src/app/instructions.rs`
- Risk: None — pure refactor, no behavioral change. All existing tests pass.
