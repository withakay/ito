## Context

Currently, `ito list` shows task status for changes using three formats:
- "No tasks" when tasks.md is missing or empty
- "X/Y tasks" when some tasks exist
- "✓ Complete" when all tasks are done

However, the JSON output only includes raw counts (`completedTasks`, `totalTasks`) and a string `status` field. This makes it difficult for both agents and the CLI archive command to programmatically identify completed changes. The existing `format_task_status()` function in main.rs already has the logic - it just needs to be exposed more cleanly.

The ito-archive skill (embedded in `ito-templates/assets/default/project/.claude/skill/ito-archive/`) requires a change ID but doesn't help users discover which changes are ready to archive.

## Goals / Non-Goals

**Goals:**
- Make it trivially easy for users and agents to identify completed changes
- Add a `--completed` filter to `ito list` for quick discovery
- Enable interactive archive selection when no change ID is specified
- Keep the implementation minimal and leverage existing logic

**Non-Goals:**
- Changing the archive process itself (validation, spec merging, etc.)
- Adding complex state machine tracking beyond task completion
- Supporting batch archive operations in a single command (iterate instead)

## Decisions

### Decision 1: Add `completed` boolean to ChangeListItem

**Choice**: Add an explicit `completed: bool` field to the `ChangeListItem` struct in `ito-core/src/list.rs`.

**Rationale**: The logic already exists (`completed == total && total > 0`). Making it a first-class field simplifies downstream consumption in both JSON output and CLI filtering. Alternatives:
- Derive from existing fields client-side: More error-prone, duplicates logic
- Use status enum: Over-engineered for this use case

### Decision 2: Add `--completed` flag to `ito list`

**Choice**: Implement as a filter flag that excludes non-completed changes from output.

**Rationale**: Simple, composable with existing flags like `--json`. The filter logic is: `completed_tasks == total_tasks && total_tasks > 0`. Alternatives:
- `--status=completed`: More flexible but YAGNI for now
- Separate command: Unnecessary complexity

### Decision 3: Update ito-archive skill for interactive selection

**Choice**: When invoked without a change ID, the skill should:
1. Run `ito list --completed --json`
2. If empty, inform user no changes are ready
3. If non-empty, present list and ask user to select

**Rationale**: Keeps CLI simple (no interactive prompts in the binary) while providing good UX through the agent skill. Alternatives:
- Interactive CLI prompts: Would require new dependencies (dialoguer/inquire)
- Always require change ID: Poor UX, current pain point

### Decision 4: Status string normalization

**Choice**: Keep existing status strings but ensure consistency:
- `"completed"` - all tasks done (total > 0, completed == total)
- `"in-progress"` - some tasks done (0 < completed < total)
- `"no-tasks"` - no tasks defined (total == 0)

**Rationale**: Aligns with existing behavior, just formalizes the values.

## Risks / Trade-offs

**[Risk]** Changes with tasks.md but no checkbox items show as "no-tasks"
→ **Mitigation**: This is existing behavior and arguably correct - if there are no trackable tasks, we can't determine completion.

**[Risk]** Users may want to archive changes that never had tasks defined
→ **Mitigation**: The `--no-validate` flag on archive already allows this. The skill can note this option when no completed changes are found.

**[Trade-off]** No interactive CLI prompts
→ Agent-based selection provides equivalent UX without adding dependencies to the Rust binary.

## Implementation Notes

Files to modify:
1. `ito-rs/crates/ito-core/src/list.rs` - Add `completed` field to `ChangeListItem`
2. `ito-rs/crates/ito-cli/src/main.rs` - Add `--completed` flag, update JSON serialization
3. `ito-rs/crates/ito-templates/assets/default/project/.claude/skill/ito-archive/skill.md` - Add interactive selection flow

The CLI display already shows "✓ Complete" via `format_task_status()` - no changes needed there.
