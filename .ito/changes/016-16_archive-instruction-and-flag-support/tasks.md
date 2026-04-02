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

### Task 1.1: Add archive instruction template

- **Files**: `ito-rs/crates/ito-templates/assets/default/project/` (or embedded `agent/archive.md.j2`)
- **Dependencies**: None
- **Action**:
  Add an `agent/archive.md.j2` Jinja2 template (or equivalent embedded string) with two rendering modes:
  - With `change` context: emit a targeted instruction telling the agent to run `ito audit reconcile --change {{ change }} && ito archive {{ change }} --yes`
  - Without `change` context: emit generic archive guidance — what `ito archive` does, when to use it, and the audit pre-check steps; optionally list available change IDs when `available_changes` is non-empty
- **Verify**: `cargo test -p ito-templates`
- **Done When**: Template renders correctly for both modes; tests pass
- **Status**: [ ] pending

### Task 1.2: Handle `archive` artifact in `handle_agent_instruction`

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs`
- **Dependencies**: Task 1.1
- **Action**:
  Add an `if artifact == "archive"` branch before the `resolve_instructions` fallthrough (around line 191, after the `finish` block). The branch should:
  1. Parse `--change` (optional, not required)
  2. If `--change` is provided, resolve the change ID via `resolve_change_target` and render the targeted template context
  3. If `--change` is absent, render the generic archive guidance template context (include available change IDs from the change repo)
  4. Call `emit_instruction(want_json, "archive", instruction)`
- **Verify**: 
  - `ito agent instruction archive` → prints generic guidance, no error
  - `ito agent instruction archive --change 009-02_event-sourced-audit-log` → prints targeted instruction with `ito archive 009-02_event-sourced-audit-log`
  - `cargo test -p ito-cli -- archive`
- **Done When**: Both invocation forms work; existing artifacts unaffected; tests pass
- **Status**: [ ] pending

---

## Wave 2

- **Depends On**: Wave 1

### Task 2.1: Update `ito agent instruction` help text and examples

- **Files**: `ito-rs/crates/ito-cli/src/app/instructions.rs` (or the clap help strings / long-help template)
- **Dependencies**: None
- **Action**:
  - Add `archive` to the `Artifacts:` section of the `ito agent instruction --help` output with a description
  - Add an example: `ito agent instruction archive --change 009-02_event-sourced-audit-log`
  - Add a no-change example: `ito agent instruction archive`
- **Verify**: `ito agent instruction --help` shows archive in the artifacts list with examples
- **Done When**: Help output documents the archive artifact correctly
- **Status**: [ ] pending

### Task 2.2: Add integration tests

- **Files**: `ito-rs/crates/ito-cli/tests/` (or inline `#[cfg(test)]` in instructions.rs if file is under 300 lines)
- **Dependencies**: Task 1.2
- **Action**:
  Add tests covering:
  - `ito agent instruction archive` (no change) → exit 0, output contains archive guidance keywords
  - `ito agent instruction archive --change <valid-id>` → exit 0, output contains `ito archive <id>`
  - `ito agent instruction archive --change <invalid-id>` → exit non-zero, error message mentions the unknown ID
- **Verify**: `cargo test -p ito-cli -- agent_instruction_archive`
- **Done When**: All three test cases pass
- **Status**: [ ] pending

<!-- ITO:END -->
