<!-- ITO:START -->
# Tasks for: 016-16_archive-instruction-and-flag-support

## Execution Notes

- **Tracking**: Use `ito tasks` CLI for status updates
- **Status legend**: `[ ] pending` · `[>] in-progress` · `[x] complete` · `[-] shelved`

```bash
ito tasks status 016-16_archive-instruction-and-flag-support
ito tasks next 016-16_archive-instruction-and-flag-support
ito tasks start 016-16_archive-instruction-and-flag-support 1.1
ito tasks complete 016-16_archive-instruction-and-flag-support 1.1
```

---

## Wave 1

- **Depends On**: None

### Task 1.1: Add archive instruction template

- **Files**: `ito-rs/crates/ito-templates/assets/instructions/agent/archive.md.j2`
- **Dependencies**: None
- **Action**:
  Add an `agent/archive.md.j2` Jinja2 template with two rendering modes:
  - With `change` context: emit a targeted instruction telling the agent to run `ito audit reconcile --change {{ change }} && ito archive {{ change }} --yes`
  - Without `change` context: emit generic archive guidance — what `ito archive` does, when to use it, and the audit pre-check steps; optionally list available change IDs when `available_changes` is non-empty
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Template renders correctly for both modes; tests pass
- **Updated At**: 2026-04-02
- **Status**: [x] complete

### Task 1.2: Handle `archive` artifact in `handle_agent_instruction`

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Add an `if artifact == "archive"` branch before the `resolve_instructions` fallthrough (after the `finish` block). The branch should:
  1. Parse `--change` (optional, not required)
  2. If `--change` is provided, resolve the change ID via `resolve_change_target` and render the targeted template context
  3. If `--change` is absent, render the generic archive guidance template context (include available change IDs from the change repo)
  4. Call `emit_instruction(want_json, "archive", instruction)`
- **Verify**:
  - `ito agent instruction archive` → prints generic guidance, no error
  - `ito agent instruction archive --change <id>` → prints targeted instruction with `ito archive <id>`
  - `cargo test -p ito-cli --test instructions_more`
- **Done When**: Both invocation forms work; existing artifacts unaffected; tests pass
- **Updated At**: 2026-04-02
- **Status**: [x] complete

---

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update `ito agent instruction` help text and examples

- **Files**: `ito-rs/crates/ito-cli/src/cli/agent.rs`
- **Dependencies**: None
- **Action**:
  - Add archive examples to the `after_help` string: `ito agent instruction archive` and `ito agent instruction archive --change <id>`
- **Verify**: `ito agent instruction --help` shows archive examples
- **Done When**: Help output documents the archive artifact with examples
- **Updated At**: 2026-04-02
- **Status**: [x] complete

### Task 2.2: Add integration tests

- **Files**: `ito-rs/crates/ito-cli/tests/instructions_more.rs`, `ito-rs/crates/ito-templates/src/instructions_tests.rs`
- **Dependencies**: None
- **Action**:
  Add tests covering:
  - `ito agent instruction archive` (no change) → exit 0, output contains archive guidance keywords
  - `ito agent instruction archive --change <valid-id>` → exit 0, output contains `ito archive <id>`
  - `ito agent instruction archive --change <invalid-id>` → exit non-zero
  - Template unit tests for both rendering modes
- **Verify**: `cargo test -p ito-cli --test instructions_more && cargo test -p ito-templates`
- **Done When**: All test cases pass
- **Updated At**: 2026-04-02
- **Status**: [x] complete

<!-- ITO:END -->
