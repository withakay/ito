---
name: ito-list
description: List Ito changes, specs, or modules with status summaries and intelligent interpretation.
---

Use `ito list` to display project items and interpret the results for the user.

**Goal**

Run the appropriate `ito list` command, present the output clearly, and offer
context-aware suggestions based on the results (e.g., which changes are ready
for implementation, which are partially complete, or what to work on next).

**CLI Reference**

```text
ito list [OPTIONS]

Item types (default: changes):
  --changes     List changes (default)
  --specs       List specs
  --modules     List modules

Progress filters (changes only, mutually exclusive):
  --ready       Changes ready for implementation (have proposal, specs, tasks, and pending work)
  --completed   Changes with all tasks done
  --partial     Changes with some but not all tasks done
  --pending     Changes with no tasks started

Other options:
  --sort <ORDER>  Sort order: recent (default) or name
  --json          Output as JSON
```

**Steps**

1. **Parse user intent** from the arguments:
   - Determine if they want changes, specs, or modules
   - Determine any progress filter (ready, completed, partial, pending)
   - Use `--json` for structured output that is easier to interpret programmatically

2. **Run the CLI command**:
   - Build the appropriate `ito list` invocation
   - Example: `ito list --ready --json` or `ito list --specs`

3. **Present and interpret results**:
   - Summarize the output in a readable format
   - For changes: highlight task progress, suggest which changes to work on next
   - For specs: note requirement counts
   - For modules: show change counts per module
   - If the list is empty, explain what that means and suggest next steps
     (e.g., "No ready changes found. Run `ito list --partial` to see in-progress work.")

4. **Suggest next actions** based on the results:
   - Ready changes: suggest running `/ito-apply <change-id>` to start implementing
   - Partial changes: suggest resuming work with `/ito-apply <change-id>`
   - Completed changes: suggest archiving with `/ito-archive <change-id>`
   - No changes at all: suggest creating one with `ito create change`

**Examples**

```bash
# List all changes (default)
ito list

# List only changes ready for implementation
ito list --ready

# List specs
ito list --specs

# List changes with JSON output for analysis
ito list --json

# List completed changes (candidates for archiving)
ito list --completed

# List modules
ito list --modules
```

**Guardrails**

- Always use `--json` when you need to programmatically interpret results.
- Progress filters (`--ready`, `--completed`, `--partial`, `--pending`) only apply to changes, not specs or modules.
- If the Ito project is not initialized, advise the user to run `ito init`.
