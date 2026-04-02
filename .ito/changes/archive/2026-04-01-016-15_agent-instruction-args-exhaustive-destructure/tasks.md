## 1. Implementation

- [x] 1.1 Add `to_argv()` method on `AgentInstructionArgs` with exhaustive `let` destructuring
- [x] 1.2 Update `reconstruct_agent_args` to delegate to `instr.to_argv()`
- [x] 1.3 Update `handle_agent_instruction_clap` to delegate to `args.to_argv()`
- [x] 1.4 Remove `push_optional_flag` helper (no longer needed)
- [x] 1.5 Remove stale "keep in sync" comment
- [x] 1.6 Verify build and existing tests pass
