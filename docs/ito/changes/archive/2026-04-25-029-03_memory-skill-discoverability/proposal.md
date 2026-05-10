<!-- ITO:START -->
## Why

Ito already has provider-agnostic memory instruction artifacts, but agents do not reliably discover them: no installed skill explains the memory workflow, and CLI help omits the memory artifacts that LLMs use as the main discoverability surface.

## What Changes

- Add one installable `ito-memory` skill that teaches agents how to capture, search, and query configured Ito memory providers through `ito agent instruction memory-*` artifacts.
- Install that skill with the normal Ito template distribution used by `ito init`, `ito init --upgrade`, and `ito update`.
- Update `ito agent instruction --help` artifact listings and examples so `memory-capture`, `memory-search`, and `memory-query` are visible anywhere agents look for supported instruction artifacts.
- Update docs to describe the installed skill and the help surface expectations.
- Add tests covering skill distribution and CLI help discoverability.

## Capabilities

### New Capabilities

- _(none)_

### Modified Capabilities

- `agent-memory-abstraction`: Adds the installed `ito-memory` skill and explicit help-surface discoverability requirements for the existing memory instruction artifacts.

## Impact

- `ito-rs/crates/ito-templates/assets/skills/ito-memory/SKILL.md`
- `ito-rs/crates/ito-cli/src/cli/agent.rs`
- `ito-rs/crates/ito-cli/tests/agent_instruction_memory.rs`
- `ito-rs/crates/ito-templates/tests/` or existing template distribution tests
- `docs/config.md`
<!-- ITO:END -->
