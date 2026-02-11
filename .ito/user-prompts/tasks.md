<!-- ITO:START -->

# Tasks Guidance

This file is for optional, user-authored guidance specific to `ito agent instruction tasks`.

- Ito may update this header block over time.
- Add your tasks guidance below the `<!-- ITO:END -->` marker.

<!-- ITO:END -->

## Your Tasks Guidance

### Task Quality

- Write small, actionable tasks with clear completion criteria.
- Prefer tasks that can be completed and verified independently.
- Include or reference a concrete verification command for each task or section.

### Execution and Status Tracking

- Keep exactly one task in progress at a time.
- Use `ito tasks start <change-id> <task-id>` and `ito tasks complete <change-id> <task-id>` for status updates.
- Avoid manual edits to task state unless unavoidable.

### Wave / Batch Convention

- If tasks include explicit waves, finish and verify one wave before moving to the next.
- If tasks are checkbox-only (no wave sections), treat each major section as a logical batch.
