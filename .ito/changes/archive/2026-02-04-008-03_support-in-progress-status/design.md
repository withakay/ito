## Context

The Ito task system currently supports two task formats:

1. **Enhanced format**: Full-featured with waves, dependencies, verification steps, and all status values (pending, in-progress, complete, shelved)
2. **Checkbox format**: Simple `- [ ]`/`- [x]` markdown checkboxes, supporting only pending and complete states

When users attempt to use `ito tasks start` with a checkbox-format tasks.md, they receive: "Checkbox-only tasks.md does not support in-progress. Use 'ito tasks complete' when done."

This limitation forces users who prefer the simpler checkbox format to either:
- Upgrade to enhanced format (more complexity than needed)
- Skip the `start` command entirely (losing visibility into current work)

## Goals / Non-Goals

**Goals:**

- Enable `ito tasks start` to work with checkbox-format tasks.md files
- Introduce a third checkbox marker (`- [~]`) for in-progress status
- Maintain backward compatibility with existing `- [ ]` and `- [x]` markers
- Enforce single in-progress constraint (only one task in-progress at a time)

**Non-Goals:**

- Adding waves or dependencies to checkbox format (keep it simple)
- Adding shelved status to checkbox format
- Changing the enhanced format behavior
- Auto-migration from checkbox to enhanced format

## Decisions

### Decision 1: Use `- [~]` as the in-progress marker

**Choice**: Use tilde (`~`) as the in-progress checkbox character

**Rationale**:
- Tilde visually suggests "work in progress" or "approximately done"
- Common in other tools (e.g., some task managers use `~` for partial completion)
- Single character maintains alignment with `[ ]` and `[x]`
- Not commonly used in existing markdown checkbox variants

Note: The parser also accepts `- [>]` as an in-progress alias for compatibility.

**Alternatives considered**:
- `- [>]` (arrow): Could suggest "next" rather than "current", more ambiguous
- `- [*]` (asterisk): Often used for bullet points, could cause parsing conflicts
- `- [/]` (slash): Sometimes means "cancelled" in other systems
- `- [-]` (dash): Often means "partially complete" or "N/A"

### Decision 2: Enforce single in-progress constraint at write time

**Choice**: Validate that only one task can be in-progress when starting a new task

**Rationale**:
- Matches the mental model of "what am I working on right now"
- Prevents ambiguity in `ito tasks next` output
- Consistent with how agents typically work (one task at a time)

**Implementation**: Check for existing `- [~]` before updating. Return error if found.

### Decision 3: Parsing changes isolated to ito-domain crate

**Choice**: Add `- [~]` recognition in the existing `TasksFormat::Checkbox` parser

**Rationale**:
- Single location for format detection and parsing
- CLI command code doesn't need to know about checkbox internals
- Easier to test in isolation

**Files affected**:
- `ito-rs/crates/ito-domain/src/tasks/parse.rs` (checkbox parsing + format detection)
- `ito-rs/crates/ito-domain/src/tasks/update.rs` (checkbox status updates)
- `ito-rs/crates/ito-cli/src/commands/tasks.rs` (checkbox compat paths for start/complete/next)

## Risks / Trade-offs

**[Risk] Existing tools may not render `- [~]` correctly**
→ Mitigation: Standard markdown renderers show `[~]` literally, which is acceptable. GitHub and VS Code will display the character. No action needed.

**[Risk] Users may have `- [~]` in their tasks for other purposes**
→ Mitigation: Unlikely edge case. The pattern `^- \[~\]` at line start is specific. Document the new behavior.

**[Risk] Checkbox format lacks task IDs**
→ Mitigation: Use 1-indexed line numbers as implicit IDs (current behavior for checkbox complete). Document this clearly.

## Open Questions

None - design is straightforward and self-contained.
