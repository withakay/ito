[Codemap: ito-cli]|L3 adapter: ito CLI surface (arg parsing, routing, terminal output, prompts, integration tests); delegates to ito-core; MUST NOT depend on ito-domain directly

[Entry Points]|src/main.rs: binary → app::main() |src/cli.rs + src/cli/**: Clap arg definitions
|src/app/{mod,entrypoint,run}.rs: dispatch + runtime |src/commands/**: command handlers + UI glue |tests/**: e2e via compiled binary

[Design]|parsing+display here; state+repo in ito-core |handlers: config/ctx → core use-case → format output |integration tests = primary regression guard
|instructions: src/app/instructions.rs dispatch/rendering; worktree_instruction_config.rs owns template-facing worktree context
|features: default={web}; backend and coordination-branch commands are opt-in with compatibility errors in standard builds

[Gotchas]|no business rules in CLI handlers (breaks backend/web parity) |large test files can trip max-lines guardrail |non-interactive paths must use flags not prompts

[Tests]|targeted: cargo test -p ito-cli --test <file> <filter> |broad: cargo test -p ito-cli or make check
