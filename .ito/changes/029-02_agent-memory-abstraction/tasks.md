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

### Task 1.1: Add `MemoryConfig` type with per-operation shape

- **Files**: `ito-rs/crates/ito-config/src/config/types.rs`, `schemas/ito-config.schema.json`
- **Dependencies**: None
- **Action**: Define a `MemoryOpConfig` as a `#[serde(tag = "kind")]` enum with variants `skill { skill: String, options: Option<serde_json::Value> }` and `command { command: String }`. Define `MemoryConfig { capture: Option<MemoryOpConfig>, search: Option<MemoryOpConfig>, query: Option<MemoryOpConfig> }`. Add `memory: Option<MemoryConfig>` to `ItoConfig`. Regenerate schema via `make config-schema` and commit.
- **Verify**: `cargo test -p ito-config`; `make config-schema-check`; roundtrip tests for both variants and for partial configs (capture only, search only, etc.).
- **Done When**: Existing configs without `memory` still load unchanged; schema check green.
- **Requirements**: `agent-memory-abstraction:optional-per-op-config`, `agent-memory-abstraction:per-op-shape`
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 1.2: Validate `MemoryConfig` — shape, required fields, skill discoverability

- **Files**: `ito-rs/crates/ito-core/src/config.rs` (or wherever `validate_config_value` lives)
- **Dependencies**: None
- **Action**: Extend config validation to check: (a) only `capture`, `search`, `query` keys are accepted under `memory`; (b) for each present op, `kind` is `skill` or `command`; (c) required fields for each kind are present (`skill` for `kind: "skill"`, `command` for `kind: "command"`); (d) `kind: "skill"` references an id discoverable under `.agents/skills/`, `.claude/skills/`, or any other known skills directory. Surface errors via the existing `validate_config_value` pathway with actionable messages that name the offending op key and field.
- **Verify**: Unit tests covering each failure mode (unknown op key, unknown kind, missing field per kind, missing skill) and the success paths (mixed shapes, partial configs).
- **Done When**: `ito validate --strict` rejects bad configs with clear messages; valid configs pass.
- **Requirements**: `agent-memory-abstraction:optional-per-op-config`, `agent-memory-abstraction:per-op-shape`
- **Updated At**: 2026-04-25
- **Status**: [x] complete

______________________________________________________________________

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Implement memory resolver in `ito-core`

- **Files**: `ito-rs/crates/ito-core/src/memory/mod.rs` (new), `ito-rs/crates/ito-core/src/memory/rendering.rs` (new)
- **Dependencies**: None
- **Action**: Implement a `memory` module that: (a) loads the resolved `MemoryConfig` from `ItoContext`; (b) exposes three entry points — `render_capture(inputs: CaptureInputs)`, `render_search(inputs: SearchInputs)`, `render_query(inputs: QueryInputs)` — each returning a `RenderedInstruction` enum with variants `Command { line: String }`, `Skill { skill_id: String, inputs: StructuredInputs, options: serde_json::Value }`, and `NotConfigured { setup_hint: String }`; (c) for command-shape, applies placeholder rendering (scalar shell-quoting via `shell_quote`, list-flag expansion to `--file 'a' --file 'b'` / `--folder 'x'`, unknown placeholders preserved as literal); (d) for skill-shape, emits structured input key/value pairs plus the opaque `options` object verbatim.
- **Verify**: Unit tests for each operation × each render branch × each placeholder edge case (list expansion, missing optional scalar, shell-metacharacter quoting, unknown placeholder pass-through). `cargo clippy` clean; `#![warn(missing_docs)]` coverage.
- **Done When**: All tests pass; the three entry points return the expected render branches deterministically.
- **Requirements**: `agent-memory-abstraction:placeholder-rendering`, `agent-memory-abstraction:skill-input-delegation`
- **Updated At**: 2026-04-25
- **Status**: [x] complete

### Task 2.2: Add `memory-capture`, `memory-search`, `memory-query` CLI artifacts

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`, `ito-rs/crates/ito-templates/assets/instructions/agent/memory-capture.md.j2`, `ito-rs/crates/ito-templates/assets/instructions/agent/memory-search.md.j2`, `ito-rs/crates/ito-templates/assets/instructions/agent/memory-query.md.j2`
- **Dependencies**: None
- **Action**: Add three new `ito agent instruction` subjects. Each CLI parser accepts its operation's inputs: `memory-capture` takes `--context`, repeatable `--file`, repeatable `--folder`; `memory-search` takes required `--query`, optional `--limit` (default 10), optional `--scope`; `memory-query` takes required `--query`. Each artifact delegates to the resolver from Task 2.1 and renders one of three template branches (command / skill / not-configured). Not-configured branch shows one minimal example of each shape (skill and command) tailored to that specific operation's placeholders.
- **Verify**: Snapshot tests for every (operation, branch) cell of the 3×3 grid. Integration tests that assert the CLI usage error when required inputs are missing.
- **Done When**: All nine snapshots stable; required-input validation returns non-zero with a usage message.
- **Requirements**: `agent-memory-abstraction:three-branch-artifacts`, `agent-memory-abstraction:operation-input-schemas`
- **Updated At**: 2026-04-25
- **Status**: [>] in-progress

______________________________________________________________________

## Wave 3

- **Depends On**: Wave 2

### Task 3.1: Append memory-capture reminder to `apply.md.j2`

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/apply.md.j2`
- **Dependencies**: None
- **Action**: Append a trailing section, guarded by `{% if memory.capture.configured %}`, with heading `Capture memories`. The section directs the agent to review the session for useful items (decisions, gotchas, patterns) and to invoke `ito agent instruction memory-capture` with appropriate `--context`, `--file`, and/or `--folder` inputs.
- **Verify**: Snapshot tests cover both configured and not-configured states for `memory.capture`. Existing apply snapshots updated only where the new section appears.
- **Done When**: Snapshot suite green; reminder appears only when `memory.capture` is configured; search/query-only configs do not render the reminder.
- **Requirements**: `agent-instructions:apply-memory-capture-reminder`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 3.2: Append memory-capture + wrap-up reminders to `finish.md.j2`

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/finish.md.j2`
- **Dependencies**: None
- **Action**: Append two sections. (1) A `Capture memories` reminder guarded by `{% if memory.capture.configured %}`, identical in intent to the apply version. (2) A `Refresh archive and specs` reminder that always renders, with three wrap-up checks: archive confirmation, specs reflect the delivered change, agent-facing docs up to date. Deduplicate the archive check against the existing "Do you want to archive this change now?" prompt: render the archive item only when the existing prompt is not rendered (the change is already archived). Specs and docs checks render unconditionally.
- **Verify**: Snapshot tests for all four combinations of {capture configured | not} × {already archived | not}. Assertion that the archive step is mentioned at most once per finish output.
- **Done When**: Snapshot suite green; dedupe assertion passes.
- **Requirements**: `agent-instructions:finish-wrap-up-reminder`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

______________________________________________________________________

## Wave 4

- **Depends On**: Wave 3

### Task 4.1: Update config docs and architecture

- **Files**: `docs/config.md`, `.ito/architecture.md`
- **Dependencies**: None
- **Action**: Document the new `memory` section: three operation keys (`capture`, `search`, `query`), each optional, each with one of two shapes (`skill`, `command`). Include one worked example per shape for each operation — e.g. a `command` example using `brv` (cross-reference `029-01_add-byterover-integration`) and a `skill` example using a generic skill id. State explicitly that there is no default provider and that a freshly-initialized Ito project has no `memory` section.
- **Verify**: Visual review; `make config-schema-check` still green.
- **Done When**: Docs updated and committed; examples align with the spec's placeholder rendering rules.
- **Requirements**: `agent-memory-abstraction:no-default-provider`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending

### Task 4.2: Run strict validation

- **Files**: _(no changes)_
- **Dependencies**: 4.1
- **Action**: Run `ito validate 029-02_agent-memory-abstraction --strict`. Address any findings.
- **Verify**: Command exits 0.
- **Done When**: Strict validation passes.
- **Requirements**: `agent-memory-abstraction:optional-per-op-config`
- **Updated At**: 2026-04-24
- **Status**: [ ] pending
<!-- ITO:END -->
