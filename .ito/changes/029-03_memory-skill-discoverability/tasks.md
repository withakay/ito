<!-- ITO:START -->
# Tasks for: 029-03_memory-skill-discoverability

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates where possible
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add installed `ito-memory` skill

- **Files**: `ito-rs/crates/ito-templates/assets/skills/ito-memory/SKILL.md`
- **Dependencies**: None
- **Action**: Add a single provider-agnostic skill that explains capture, search, and query through `ito agent instruction memory-*`.
- **Verify**: `cargo test -p ito-templates`
- **Done When**: The skill exists in shared template assets and is picked up by skill distribution tests.
- **Requirements**: `agent-memory-abstraction:installed-ito-memory-skill`
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 1.2: Make memory artifacts visible in CLI help

- **Files**: `ito-rs/crates/ito-cli/src/cli/agent.rs`, `ito-rs/crates/ito-cli/tests/agent_instruction_memory.rs`
- **Dependencies**: None
- **Action**: Add `memory-capture`, `memory-search`, and `memory-query` to `ito agent instruction --help` artifact listings and examples; cover with tests.
- **Verify**: `cargo test -p ito-cli --test agent_instruction_memory`
- **Done When**: CLI help output contains all three memory artifact names and at least one memory example with `--query`.
- **Requirements**: `agent-memory-abstraction:memory-artifacts-in-help`
- **Updated At**: 2026-04-25
- **Status**: [>] in-progress

### Task 1.3: Document memory skill discoverability

- **Files**: `docs/config.md`
- **Dependencies**: None
- **Action**: Update memory docs to mention the installed `ito-memory` skill and the help surface.
- **Verify**: Manual review plus relevant markdown checks in `make check`.
- **Done When**: Docs explain how agents discover the memory skill and instruction artifacts.
- **Requirements**: `agent-memory-abstraction:installed-ito-memory-skill`, `agent-memory-abstraction:memory-artifacts-in-help`
- **Updated At**: 2026-04-25
- **Status**: [>] in-progress

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Validate and run focused tests

- **Files**: `.ito/changes/029-03_memory-skill-discoverability/**`, `ito-rs/crates/ito-templates/**`, `ito-rs/crates/ito-cli/**`, `docs/config.md`
- **Dependencies**: None
- **Action**: Validate the change package and run focused Rust tests.
- **Verify**: `ito validate 029-03_memory-skill-discoverability --strict && cargo test -p ito-templates && cargo test -p ito-cli --test agent_instruction_memory`
- **Done When**: Validation and focused tests pass.
- **Requirements**: `agent-memory-abstraction:installed-ito-memory-skill`, `agent-memory-abstraction:memory-artifacts-in-help`
- **Updated At**: 2026-04-25
- **Status**: [ ] pending
<!-- ITO:END -->
