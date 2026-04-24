<!-- ITO:START -->
# Tasks for: 029-02_agent-memory-abstraction

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 029-02_agent-memory-abstraction
ito tasks next 029-02_agent-memory-abstraction
ito tasks start 029-02_agent-memory-abstraction 1.1
ito tasks complete 029-02_agent-memory-abstraction 1.1
```

______________________________________________________________________

## Wave 1

- **Depends On**: None

### Task 1.1: Add `MemoryConfig` type and JSON schema entry

- **Files**: `ito-rs/crates/ito-config/src/config/types.rs`, `schemas/ito-config.schema.json`
- **Dependencies**: None
- **Action**: Define `MemoryConfig` as a `#[serde(tag = "provider")]` enum with two variants: `commands { store: String, search: String }` and `skill { skill: String }`. Add `memory: Option<MemoryConfig>` to `ItoConfig`. Regenerate schema via `make config-schema` and commit.
- **Verify**: `cargo test -p ito-config`; `make config-schema-check`.
- **Done When**: Roundtrip tests for both variants pass; schema check green; existing configs without `memory` still load.
- **Requirements**: `agent-memory-abstraction:optional-config`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 1.2: Validate `MemoryConfig` — placeholders, provider-specific fields, skill discoverability

- **Files**: `ito-rs/crates/ito-core/src/config.rs` (or wherever `validate_config_value` lives)
- **Dependencies**: Task 1.1
- **Action**: Extend config validation to check: (a) `commands.store` contains `{text}`, (b) `commands.search` contains `{query}`, (c) `skill.skill` resolves to an installed skill in any known skills directory. Surface errors via the existing `validate_config_value` pathway with actionable messages.
- **Verify**: Unit tests covering each failure mode and the success paths.
- **Done When**: `ito validate --strict` rejects bad configs with clear messages; valid configs pass.
- **Requirements**: `agent-memory-abstraction:commands-provider`, `agent-memory-abstraction:skill-provider`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement memory resolver in `ito-core`

- **Files**: `ito-rs/crates/ito-core/src/memory/mod.rs` (new)
- **Dependencies**: None
- **Action**: Implement a small module that: (a) loads `MemoryConfig` from `ItoContext`, (b) exposes helpers that resolve the instruction to emit for store/search operations, where the rendered instruction is either a command template (commands provider) or a skill invocation hint (skill provider). Preserve literal `{text}` / `{query}` placeholders in instruction output (the agent substitutes them at execution time, not Ito). Treat unknown placeholders as opaque literal text.
- **Verify**: Unit tests for both providers + not-configured case; property tests for placeholder rendering.
- **Done When**: All tests pass; `cargo clippy` clean; the module has `#![warn(missing_docs)]` coverage.
- **Requirements**: `agent-memory-abstraction:placeholder-semantics`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 2.2: Add `ito agent instruction memory-capture` and `memory-search` artifacts

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-templates/assets/instructions/agent/memory-capture.md.j2`, `ito-rs/crates/ito-templates/assets/instructions/agent/memory-search.md.j2`
- **Dependencies**: Task 2.1
- **Action**: Add two new instruction artifacts. Each template renders a section that dispatches on the resolved provider: commands → show the rendered command line; skill → show the "invoke this skill" guidance; absent → show setup hints with one-line examples for each provider shape.
- **Verify**: Snapshot tests for each of the three branches (commands, skill, not-configured).
- **Done When**: `ito agent instruction memory-capture` and `memory-search` produce the three expected outputs.
- **Requirements**: `agent-memory-abstraction:memory-capture-artifact`, `agent-memory-abstraction:memory-search-artifact`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Add memory-capture reminder to `apply.md.j2`

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/apply.md.j2`
- **Dependencies**: None
- **Action**: Append a trailing section, guarded by `{% if memory.configured %}`, with heading `Capture memories`. The section directs the agent to review the session for useful items and run `ito agent instruction memory-capture` (or the rendered provider command/skill).
- **Verify**: Snapshot tests cover both branches (configured / not configured). Existing apply snapshots updated if needed.
- **Done When**: Snapshot suite green; reminder appears only when memory is configured.
- **Requirements**: `agent-instructions:apply-memory-reminder`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 3.2: Add memory-capture + wrap-up reminders to `finish.md.j2`

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/finish.md.j2`
- **Dependencies**: None
- **Action**: Append two sections: (1) memory-capture reminder (guarded by `{% if memory.configured %}`), identical in intent to the apply version; (2) `Refresh archive and specs` reminder that always renders, always covers canonical specs + agent-facing docs, and references the archive step only when it is not already covered by the existing "Do you want to archive this change now?" prompt.
- **Verify**: Snapshot tests for both memory states; dedupe assertion confirming the archive step is mentioned at most once per finish output.
- **Done When**: Snapshot suite green; dedupe assertion passes.
- **Requirements**: `agent-instructions:finish-wrap-up-reminder`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Update config docs and examples

- **Files**: `docs/config.md`, `.ito/architecture.md` (if memory belongs in the architecture overview)
- **Dependencies**: None
- **Action**: Document the new `memory` section with one commands-provider example (generic, no specific backend) and one skill-provider example (generic skill id). Explicitly note that there is no default provider.
- **Verify**: Visual review; `make config-schema-check` still green.
- **Done When**: Docs updated and committed.
- **Requirements**: `agent-memory-abstraction:no-default-provider`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 4.2: Run strict validation

- **Files**: _(no changes)_
- **Dependencies**: Task 4.1
- **Action**: Run `ito validate 029-02_agent-memory-abstraction --strict`. Address any findings.
- **Verify**: Command exits 0.
- **Done When**: Strict validation passes.
- **Requirements**: `agent-memory-abstraction:optional-config`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending
<!-- ITO:END -->
