---
name: ito-loop
description: Run an ito ralph loop for a change, module, or repo-ready sequence, with safe defaults and automatic restart context on early exits.
---

<!-- ITO:START -->
<!--ITO:VERSION:0.1.30-->

# Skill: ito-loop

Run `ito ralph` for one change, one module, or the next ready work item, with safe defaults and bounded restarts.

## Inputs

Parse `/ito-loop` arguments into one of these modes:

| Input | Mode | Command shape |
|---|---|---|
| `^[0-9]{3}-[0-9]{2}_[a-z0-9-]+$` | change | `ito ralph ... --change <id>` |
| `^[0-9]{3}$` | module | `ito ralph ... --module <id>` |
| `next`, `continue`, natural language for next ready work, or empty | continue-ready | `ito ralph ... --continue-ready` |

### Optional flags (free text, best-effort)

- `--model <model-id>`
- `--max-iterations <n>`
- `--timeout <duration>` (e.g. `15m`)

## Default behavior

- Harness: current harness (`opencode`, `claude`, `codex`, `copilot`, or `pi`)
- Max iterations: 5
- Timeout: 15m
- Outer restarts on restartable failures: 2

## Procedure

1) Parse input with `ito util parse-id $ARGUMENTS`:

   ```bash
   parsed=$(ito util parse-id $ARGUMENTS)
   mode=$(echo "$parsed" | jq -r '.mode')
   id=$(echo "$parsed" | jq -r '.id // empty')
   ```

   - `mode` will be `change`, `module`, or `continue-ready`.
   - `id` is set for `change` and `module` modes; empty for `continue-ready`.
   - If the command fails, ask the user to clarify.
   - Never use `eval`, and always quote variables.

2) Choose the active harness.

3) Build one base `ito ralph` command. Ralph already manages its own internal loop, so do **not** wrap it in an unbounded retry loop.

   Command shapes:

   ```bash
   # Mode: change
   ito ralph --no-interactive --harness <harness> --change <change-id> --max-iterations 5 --timeout 15m

   # Mode: module
   ito ralph --no-interactive --harness <harness> --module <module-id> --max-iterations 5 --timeout 15m

   # Mode: continue-ready
   ito ralph --no-interactive --harness <harness> --continue-ready --max-iterations 5 --timeout 15m
   ```

   Apply any user-provided overrides on top of the defaults. Check `ito ralph --help` only if extra flags matter.

4) Run the command once.
   - Exit `0`: report success and stop.
   - Restartable non-zero exit: restart at most **2** times.
   - Non-restartable failure: report failure and stop.

5) For each bounded restart, collect context from:

     ```bash
     ito ralph --no-interactive --change <change-id> --status
     ito tasks status <change-id>
     ```

   Summarize the context into a short restart note containing:
   - `You have been restarted ...`
   - last iteration / last failure / current task summary
   - one sentence telling Ralph to continue from current state

   Append it before the rerun:

     ```bash
     ito ralph --no-interactive --change <change-id> --add-context "<restart-note>"
     ```

   Re-run the same base command.

6) After the supervised run sequence finishes:

   - **Exit 0**: report completion.
   - **Non-zero exit after bounded restarts**: report failure plus the last useful Ralph status summary.

## Guardrails

- Do not wrap Ralph in an unbounded outer loop.
- Only use restart context when `ito ralph --status` and `ito tasks status` are meaningful.
- For module or continue-ready runs, do not invent per-change restart behavior unless the failure clearly reduces to one targeted change.
<!-- ITO:END -->
